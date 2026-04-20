export default function BuiltOnStellar() {
  const features = [
    {
      icon: "payments",
      title: "Native USDC",
      description: "Settle instantly in a regulated, liquid dollar stablecoin.",
    },
    {
      icon: "speed",
      title: "Near-zero Fees",
      description:
        "Transactions cost fractions of a cent, maximizing your margins.",
    },
    {
      icon: "bolt",
      title: "Fast Finality",
      description: "Transactions are confirmed in seconds with 100% finality.",
    },
    {
      icon: "terminal",
      title: "Soroban",
      description:
        "Powered by Stellar's high-performance WASM smart contracts.",
    },
  ];

  return (
    <section className="bg-surface-container-low py-24 px-8">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl font-headline mb-4">
            The performance of Stellar
          </h2>
          <p className="text-on-surface-variant max-w-2xl mx-auto">
            Leveraging the world's most efficient blockchain for payments and
            asset issuance.
          </p>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          {features.map((feature, index) => (
            <div
              key={index}
              className="bg-surface-container-lowest p-8 rounded-xl border border-outline-variant/10 hover:shadow-lg transition-shadow"
            >
              <span className="material-symbols-outlined text-primary-container text-4xl mb-4">
                {feature.icon}
              </span>
              <h4 className="font-bold mb-2">{feature.title}</h4>
              <p className="text-sm text-on-surface-variant leading-relaxed">
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
