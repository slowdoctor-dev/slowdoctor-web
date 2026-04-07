import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = {
  alternates: { canonical: "/" },
};

export default function Home() {
  return (
    <div className="mx-auto max-w-3xl px-6">
      {/* Hero */}
      <section className="pt-24 pb-16 sm:pt-32 sm:pb-20">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Joonho Lim
        </h1>
        <p className="mt-4 text-lg text-accent font-medium">
          Choosing the right way over the fast way.
        </p>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          Board-certified plastic surgeon and engineer building an AI-operated
          clinic.
        </p>
      </section>

      {/* Two Axes */}
      <section className="pb-24 grid grid-cols-1 sm:grid-cols-2 gap-4">
        <Link
          href="/physician"
          className="group rounded-lg border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
        >
          <h2 className="text-lg font-semibold group-hover:text-accent transition-colors">
            As a Physician
          </h2>
          <p className="mt-2 text-sm text-muted">
            Slow-aging, scars, and natural eyes.
          </p>
          <span className="mt-4 inline-block text-sm text-accent opacity-0 group-hover:opacity-100 transition-opacity">
            Learn more &rarr;
          </span>
        </Link>
        <Link
          href="/engineer"
          className="group rounded-lg border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
        >
          <h2 className="text-lg font-semibold group-hover:text-accent transition-colors">
            As an Engineer
          </h2>
          <p className="mt-2 text-sm text-muted">
            Building an AI-operated clinic.
          </p>
          <span className="mt-4 inline-block text-sm text-accent opacity-0 group-hover:opacity-100 transition-opacity">
            Learn more &rarr;
          </span>
        </Link>
      </section>

    </div>
  );
}
