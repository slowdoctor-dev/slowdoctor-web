import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { practiceUrl } from "@/lib/links";

export const metadata: Metadata = {
  title: "Physician",
  description:
    "Board-certified plastic surgeon specializing in slow-aging, scar treatment, and blepharoplasty.",
  alternates: { canonical: "/physician" },
};

export default function PhysicianPage() {
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Physician", href: "/physician" },
  ]);

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Physician
        </h1>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          Choosing the right way over the fast way.
        </p>
      </section>

      {/* Clinical Philosophy */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Clinical Philosophy
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            I practice slow-aging, not anti-aging. The goal is not to reverse
            time but to age well -- gracefully, naturally, and at the right
            pace. Every face has its own timeline, and my work is about
            respecting that timeline while guiding it in a better direction.
          </p>
          <p>
            My clinical focus sits on three axes.{" "}
            <strong className="text-foreground">Slow-aging</strong>: non-surgical
            facial rejuvenation using energy-based devices and injectables,
            calibrated for long-term results over quick fixes.{" "}
            <strong className="text-foreground">Scars</strong>: evidence-based,
            non-surgical scar treatment combining lasers, microneedling, and
            subcision for both surgical scars and acne scarring.{" "}
            <strong className="text-foreground">Natural Eyes</strong>:
            blepharoplasty that preserves individuality -- minimal, precise, and
            designed to look like nothing was done at all.
          </p>
          <p>
            In a field driven by trends and speed, I choose the slower path.
            Better outcomes take more time. That patience is the foundation of
            everything I do.
          </p>
        </div>
      </section>

      {/* Practice */}
      <section className="pb-24">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Practice
        </h2>
        <div className="rounded-lg border border-border bg-card p-6">
          <h3 className="text-lg font-semibold">LEAD Plastic Surgery</h3>
          <p className="mt-1 text-sm text-muted">
            Dogok-dong, Gangnam-gu, Seoul, South Korea
          </p>
          <p className="mt-1 text-sm text-muted">+82-2-6953-3231</p>
          <a
            href={practiceUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="mt-4 inline-block text-sm text-accent hover:underline"
          >
            leadps.co.kr &rarr;
          </a>
        </div>
      </section>
    </div>
  );
}
