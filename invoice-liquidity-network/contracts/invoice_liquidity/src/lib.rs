#![no_std]

mod errors;
mod invoice;

use soroban_sdk::{
    contract, contractimpl,
    token::Client as TokenClient,
    Address, Env, Vec,
};

use errors::ContractError;
use invoice::{
    invoice_exists, load_invoice, next_invoice_id, save_invoice,
    Invoice, InvoiceStatus,
};

// ----------------------------------------------------------------
// Storage Key (FIXED - single source of truth)
// ----------------------------------------------------------------

#[contracttype]
pub enum StorageKey {
    Invoice(u64),
    InvoiceCount,

    ApprovedToken(Address),
    TokenList,
}

// ----------------------------------------------------------------
// CONTRACT
// ----------------------------------------------------------------

#[contract]
pub struct InvoiceLiquidityContract;

#[contractimpl]
impl InvoiceLiquidityContract {

    // ------------------------------------------------------------
    // initialize (multi-token aware)
    // ------------------------------------------------------------
    pub fn initialize(env: Env, token: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&StorageKey::InvoiceCount) {
            return Err(ContractError::Unauthorized);
        }

        // approve first token (USDC or default)
        env.storage()
            .persistent()
            .set(&StorageKey::ApprovedToken(token.clone()), &true);

        let mut list: Vec<Address> = Vec::new(&env);
        list.push_back(token.clone());

        env.storage()
            .persistent()
            .set(&StorageKey::TokenList, &list);

        Ok(())
    }

    // ------------------------------------------------------------
    // submit_invoice (NOW TOKEN-AWARE)
    // ------------------------------------------------------------
    pub fn submit_invoice(
        env: Env,
        freelancer: Address,
        payer: Address,
        amount: i128,
        due_date: u64,
        discount_rate: u32,
        token: Address,
    ) -> Result<u64, ContractError> {

        freelancer.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        if discount_rate == 0 || discount_rate > 5_000 {
            return Err(ContractError::InvalidDiscountRate);
        }

        let now = env.ledger().timestamp();
        if due_date <= now {
            return Err(ContractError::InvalidDueDate);
        }

        // token validation
        if !is_approved_token(&env, &token) {
            return Err(ContractError::Unauthorized);
        }

        let id = next_invoice_id(&env);

        let invoice = Invoice {
            id,
            freelancer,
            payer,
            amount,
            due_date,
            discount_rate,
            status: InvoiceStatus::Pending,
            funder: None,
            funded_at: None,
            token,
        };

        save_invoice(&env, &invoice);

        env.events().publish(
            (soroban_sdk::symbol_short!("submitted"),),
            id,
        );

        Ok(id)
    }

    // ------------------------------------------------------------
    // fund_invoice (USES invoice.token)
    // ------------------------------------------------------------
    pub fn fund_invoice(
        env: Env,
        funder: Address,
        invoice_id: u64,
    ) -> Result<(), ContractError> {

        funder.require_auth();

        if !invoice_exists(&env, invoice_id) {
            return Err(ContractError::InvoiceNotFound);
        }

        let mut invoice = load_invoice(&env, invoice_id);

        match invoice.status {
            InvoiceStatus::Funded => return Err(ContractError::AlreadyFunded),
            InvoiceStatus::Paid => return Err(ContractError::AlreadyPaid),
            InvoiceStatus::Defaulted => return Err(ContractError::InvoiceDefaulted),
            InvoiceStatus::Pending => {}
        }

        let discount_amount =
            invoice.amount * (invoice.discount_rate as i128) / 10_000;

        let freelancer_payout = invoice.amount - discount_amount;

        let token = token_client(&env, &invoice.token);
        let contract = env.current_contract_address();

        token.transfer(&funder, &contract, &invoice.amount);
        token.transfer(&contract, &invoice.freelancer, &freelancer_payout);

        let now = env.ledger().timestamp();

        invoice.status = InvoiceStatus::Funded;
        invoice.funder = Some(funder.clone());
        invoice.funded_at = Some(now);

        save_invoice(&env, &invoice);

        env.events().publish(
            (soroban_sdk::symbol_short!("funded"),),
            invoice_id,
        );

        Ok(())
    }

    // ------------------------------------------------------------
    // mark_paid (USES invoice.token)
    // ------------------------------------------------------------
    pub fn mark_paid(
        env: Env,
        invoice_id: u64,
    ) -> Result<(), ContractError> {

        if !invoice_exists(&env, invoice_id) {
            return Err(ContractError::InvoiceNotFound);
        }

        let mut invoice = load_invoice(&env, invoice_id);

        invoice.payer.require_auth();

        match invoice.status {
            InvoiceStatus::Pending => return Err(ContractError::NotFunded),
            InvoiceStatus::Paid => return Err(ContractError::AlreadyPaid),
            InvoiceStatus::Defaulted => return Err(ContractError::InvoiceDefaulted),
            InvoiceStatus::Funded => {}
        }

        let funder = invoice.funder.clone().ok_or(ContractError::NotFunded)?;

        let discount_amount =
            invoice.amount * (invoice.discount_rate as i128) / 10_000;

        let token = token_client(&env, &invoice.token);
        let contract = env.current_contract_address();

        token.transfer(&invoice.payer, &contract, &invoice.amount);
        token.transfer(&contract, &funder, &(invoice.amount + discount_amount));

        invoice.status = InvoiceStatus::Paid;

        save_invoice(&env, &invoice);

        env.events().publish(
            (soroban_sdk::symbol_short!("paid"),),
            invoice_id,
        );

        Ok(())
    }

    // ------------------------------------------------------------
    // read-only helpers
    // ------------------------------------------------------------
    pub fn get_invoice(env: Env, invoice_id: u64) -> Result<Invoice, ContractError> {
        if !invoice_exists(&env, invoice_id) {
            return Err(ContractError::InvoiceNotFound);
        }
        Ok(load_invoice(&env, invoice_id))
    }

    pub fn get_invoice_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&StorageKey::InvoiceCount)
            .unwrap_or(0)
    }
}

// ----------------------------------------------------------------
// TOKEN HELPERS
// ----------------------------------------------------------------

fn token_client(env: &Env, token: &Address) -> TokenClient<'_> {
    TokenClient::new(env, token)
}

fn is_approved_token(env: &Env, token: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&StorageKey::ApprovedToken(token.clone()))
        .unwrap_or(false)
}

// ----------------------------------------------------------------
// TEST MODULES
// ----------------------------------------------------------------

mod test;
mod tests_security;