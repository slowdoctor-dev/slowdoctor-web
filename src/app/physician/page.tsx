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

interface Publication {
  title: string;
  authors: string;
  journal: string;
  year: number;
  doi?: string;
  pubmed?: string;
}

const publications: Publication[] = [
  {
    title:
      "Oncologic outcomes after immediate breast reconstruction following mastectomy: comparison of implant and flap using propensity score matching",
    authors: "Ha JH, Hong KY, Lee HB, Moon HG, Han W, Noh DY, Lim J, Yoon S, Chang H, Jin US",
    journal: "BMC Cancer",
    year: 2020,
    doi: "10.1186/s12885-020-6568-2",
    pubmed: "32000718",
  },
  {
    title:
      "Flap reconstruction of soft tissue defect after resecting a huge hemangioma of the nose",
    authors: "Lim J, Oh J, Eun S",
    journal: "Archives of Craniofacial Surgery",
    year: 2020,
    doi: "10.7181/acfs.2019.00668",
    pubmed: "32126625",
  },
  {
    title:
      "The reconstruction of the central tubercle in bilateral cleft lips: bilateral lateral mucosal advancement flap with reinforcement of the orbicularis oris muscle",
    authors: "Chung J, Lim J, Kim S, Koo Y",
    journal: "Annals of Plastic Surgery",
    year: 2019,
    doi: "10.1097/SAP.0000000000002012",
    pubmed: "31397684",
  },
  {
    title:
      "Correlation between speech outcomes and the amount of maxillary advancement after orthognathic surgery in patients with cleft lip and palate",
    authors: "Chung J, Lim J, Park H, Yoo A, Kim S, Koo Y",
    journal: "The Journal of Craniofacial Surgery",
    year: 2019,
    doi: "10.1097/SCS.0000000000005623",
    pubmed: "31107383",
  },
  {
    title:
      "Immediate lower extremity reconstruction using an anterolateral thigh free flap with simultaneous interposition graft of descending branches of lateral circumflex femoral vessels",
    authors: "Lim J, Kwon H, Lee KM, Pak C",
    journal: "The International Journal of Lower Extremity Wounds",
    year: 2019,
    doi: "10.1177/1534734618819932",
    pubmed: "31064286",
  },
];

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
      </section>

      {/* Professional Background */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Professional Background
        </h2>
        <ul className="space-y-3 text-foreground leading-relaxed">
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>Seoul Science High School (graduated early, valedictorian)</span>
          </li>
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>Seoul National University College of Medicine</span>
          </li>
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>
              Internship & Residency: Seoul National University Hospital,
              Plastic Surgery
            </span>
          </li>
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>Board-Certified Plastic Surgeon</span>
          </li>
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>
              Member: Korean Society of Plastic and Reconstructive Surgeons,
              Korean Society of Aesthetic Plastic Surgery
            </span>
          </li>
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>
              Former: Attending, The Plastic Surgery / Pop Plastic Surgery /
              Wonderful Plastic Surgery (Apgujeong, Seoul)
            </span>
          </li>
          <li className="flex gap-3">
            <span className="text-muted select-none shrink-0">&mdash;</span>
            <span>
              Current: Founder & Lead Physician,{" "}
              <a
                href={practiceUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="text-accent hover:underline"
              >
                LEAD Plastic Surgery
              </a>
            </span>
          </li>
        </ul>
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

      {/* Publications */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Publications
        </h2>
        <div className="space-y-4">
          {publications.map((publication) => (
            <article
              key={`${publication.title}-${publication.year}`}
              className="rounded-lg border border-border bg-card p-5"
            >
              <h3 className="text-base font-semibold text-foreground">
                {publication.title}
              </h3>
              <p className="mt-2 text-sm text-muted">{publication.authors}</p>
              <p className="mt-1 text-sm text-muted">
                <span className="italic">{publication.journal}</span>,{" "}
                {publication.year}
              </p>
              {(publication.doi || publication.pubmed) && (
                <div className="mt-3 flex flex-wrap gap-4 text-sm">
                  {publication.doi && (
                    <a
                      href={`https://doi.org/${publication.doi}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-accent hover:underline"
                    >
                      DOI
                    </a>
                  )}
                  {publication.pubmed && (
                    <a
                      href={`https://pubmed.ncbi.nlm.nih.gov/${publication.pubmed}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-accent hover:underline"
                    >
                      PubMed
                    </a>
                  )}
                </div>
              )}
            </article>
          ))}
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
