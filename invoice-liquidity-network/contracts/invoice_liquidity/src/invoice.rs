use soroban_sdk::{contracttype, Address, Env, Vec};

// ----------------------------------------------------------------
// Status enum — tracks lifecycle of invoice
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceStatus {
    Pending,
    Funded,
    Paid,
    Defaulted,
}

// ----------------------------------------------------------------
// Invoice struct (UPDATED - token stays per invoice)
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug)]
pub struct Invoice {
    pub id: u64,
    pub freelancer: Address,
    pub lp: Address,
    pub amount: i128,
    pub discount_rate: i128,
    pub status: InvoiceStatus,
    pub token: Address, // invoice-specific token
}

// ----------------------------------------------------------------
// Storage key (UPDATED for multi-token registry)
// ----------------------------------------------------------------

#[contracttype]
pub enum StorageKey {
    Invoice(u64),
    InvoiceCount,

    // Multi-token registry
    ApprovedToken(Address), // maps token → bool
    TokenList,              // Vec<Address>
}

// ----------------------------------------------------------------
// Storage helpers (UNCHANGED CORE LOGIC)
// ----------------------------------------------------------------

pub fn save_invoice(env: &Env, invoice: &Invoice) {
    env.storage()
        .persistent()
        .set(&StorageKey::Invoice(invoice.id), invoice);
}

pub fn load_invoice(env: &Env, id: u64) -> Invoice {
    env.storage()
        .persistent()
        .get(&StorageKey::Invoice(id))
        .expect("invoice not found")
}

pub fn invoice_exists(env: &Env, id: u64) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Invoice(id))
}

pub fn next_invoice_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .persistent()
        .get(&StorageKey::InvoiceCount)
        .unwrap_or(0);

    let next = current + 1;

    env.storage()
        .persistent()
        .set(&StorageKey::InvoiceCount, &next);

    next
}

// ----------------------------------------------------------------
// TOKEN REGISTRY HELPERS (NEW LOGIC)
// ----------------------------------------------------------------

/// Check if token is approved
pub fn is_approved_token(env: &Env, token: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&StorageKey::ApprovedToken(token.clone()))
        .unwrap_or(false)
}

/// Add token to registry (admin-only logic handled in contract)
pub fn set_token_approved(env: &Env, token: &Address, approved: bool) {
    env.storage()
        .persistent()
        .set(&StorageKey::ApprovedToken(token.clone()), &approved);
}

/// Get all approved tokens
pub fn get_token_list(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&StorageKey::TokenList)
        .unwrap_or(Vec::new(env))
}

/// Add token to list
pub fn add_to_token_list(env: &Env, token: Address) {
    let mut list: Vec<Address> = get_token_list(env);
    list.push_back(token);
    env.storage().persistent().set(&StorageKey::TokenList, &list);
}

/// Remove token from list
pub fn remove_from_token_list(env: &Env, token: Address) {
    let mut list: Vec<Address> = get_token_list(env);
    list.retain(|t| t != &token);
    env.storage().persistent().set(&StorageKey::TokenList, &list);
}