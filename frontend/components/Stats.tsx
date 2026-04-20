export default function Stats() {
  const stats = [
    { label: "Total Invoices", value: "1,247" },
    { label: "Volume", value: "$2.4M" },
    { label: "Avg. Yield", value: "3.2%" },
    { label: "Active LPs", value: "89" },
  ];

  return (
    <section className="bg-surface-dim py-12 px-8">
      <div className="max-w-7xl mx-auto grid grid-cols-2 md:grid-cols-4 gap-8">
        {stats.map((stat, index) => (
          <div key={index} className="p-6 bg-surface-container-low rounded-lg">
            <p className="text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-2">
              {stat.label}
            </p>
            <p className="text-3xl font-headline font-medium text-primary leading-none">
              {stat.value}
            </p>
          </div>
        ))}
      </div>
    </section>
  );
}
