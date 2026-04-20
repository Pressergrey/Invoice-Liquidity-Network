export default function Navbar() {
  return (
    <nav className="fixed top-0 w-full z-50 bg-white/80 backdrop-blur-md border-b border-slate-200/15 shadow-sm h-20">
      <div className="flex justify-between items-center px-8 h-full max-w-7xl mx-auto">
        <div className="text-2xl font-bold text-[#81a6c6] tracking-tight">ILN</div>
        <div className="hidden md:flex items-center gap-8">
          <a
            className="text-slate-600 hover:text-primary transition-colors duration-200 text-sm font-medium"
            href="#"
          >
            How it works
          </a>
          <a
            className="text-slate-600 hover:text-primary transition-colors duration-200 text-sm font-medium"
            href="#"
          >
            For Freelancers
          </a>
          <a
            className="text-slate-600 hover:text-primary transition-colors duration-200 text-sm font-medium"
            href="#"
          >
            For LPs
          </a>
          <a
            className="text-slate-600 hover:text-primary transition-colors duration-200 text-sm font-medium"
            href="#"
          >
            Docs
          </a>
          <a
            className="text-slate-600 hover:text-primary transition-colors duration-200 text-sm font-medium"
            href="#"
          >
            GitHub
          </a>
        </div>
        <div className="flex items-center gap-4">
          <button className="hidden lg:flex items-center px-4 py-2 border border-primary text-primary rounded-lg text-sm font-medium hover:bg-primary/5 transition-all active:scale-95 duration-150">
            View on GitHub
          </button>
          <button className="bg-primary text-on-primary px-6 py-2 rounded-lg text-sm font-bold shadow-md hover:bg-primary/90 transition-all active:scale-95 duration-150">
            Launch App
          </button>
        </div>
      </div>
    </nav>
  );
}
