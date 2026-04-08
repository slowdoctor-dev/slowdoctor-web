import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { practiceUrl } from "@/lib/links";
import { buildPageMetadata } from "@/lib/metadata";
import { AUTHOR, PRACTICE } from "@/lib/config";
import { publications } from "@/lib/cv";

export const metadata: Metadata = buildPageMetadata({
  title: "CV",
  description:
    "Curriculum vitae of Joonho Lim – education, training, professional experience, and publications.",
  path: "/cv",
});

function CvEntry({
  title,
  subtitle,
  date,
}: {
  title: React.ReactNode;
  subtitle?: React.ReactNode;
  date: string;
}) {
  return (
    <div className="flex justify-between gap-4">
      <div>
        <p className="text-foreground font-medium">{title}</p>
        {subtitle && <p className="text-muted">{subtitle}</p>}
      </div>
      <p className="text-muted whitespace-nowrap shrink-0">{date}</p>
    </div>
  );
}

export default function CvPage() {
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "CV", href: "/cv" },
  ]);

  const publicationSchemas = publications.map((pub) => ({
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    headline: pub.title,
    author: pub.authors.split(", ").map((name) => ({
      "@type": "Person",
      name,
    })),
    datePublished: String(pub.year),
    isPartOf: {
      "@type": "Periodical",
      name: pub.journal,
    },
    ...(pub.doi && { url: `https://doi.org/${pub.doi}` }),
    ...(pub.doi && {
      identifier: {
        "@type": "PropertyValue",
        propertyID: "DOI",
        value: pub.doi,
      },
    }),
  }));

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      {publicationSchemas.map((schema, i) => (
        <JsonLd key={i} data={schema} />
      ))}

      {/* Header */}
      <section className="pt-24 pb-8 sm:pt-32 sm:pb-10 text-center">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
          {AUTHOR.title}
        </h1>
        <p className="mt-1 text-muted text-sm">{AUTHOR.korean}</p>
        <p className="mt-3 text-sm text-muted">
          {PRACTICE.fullName} &middot; {PRACTICE.location}
        </p>
        <p className="mt-1 text-sm text-muted">
          <a
            href={practiceUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="text-accent hover:underline"
          >
            LEAD Plastic Surgery Clinic website
          </a>
        </p>
      </section>

      <hr className="border-border mb-10" />

      {/* Education */}
      <section className="pb-10">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Education
        </h2>
        <div className="space-y-3 text-sm">
          <CvEntry
            title="Korea National Open University"
            subtitle="B.S. in Statistics and Data Science / B.S. in Computer Science (Double Major, in progress)"
            date="2025 – Present"
          />
          <CvEntry
            title="Seoul National University College of Medicine"
            subtitle="Doctor of Medicine (M.D.)"
            date="2006 – 2012"
          />
          <CvEntry
            title="Seoul Science High School"
            subtitle="Early graduation, Valedictorian"
            date="2004 – 2006"
          />
        </div>
      </section>

      {/* Postgraduate Training */}
      <section className="pb-10">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Postgraduate Training
        </h2>
        <div className="space-y-3 text-sm">
          <CvEntry
            title="Residency, Department of Plastic and Reconstructive Surgery"
            subtitle="Seoul National University Hospital"
            date="2016 – 2020"
          />
          <CvEntry
            title="Internship"
            subtitle="Seoul National University Hospital"
            date="2012 – 2013"
          />
        </div>
      </section>

      {/* Military Service */}
      <section className="pb-10">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Military Service
        </h2>
        <div className="text-sm">
          <CvEntry
            title="Military Medical Officer"
            subtitle="Republic of Korea Army, Daejeon"
            date="2013 – 2016"
          />
        </div>
      </section>

      {/* Licensure & Board Certification */}
      <section className="pb-10">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Licensure & Board Certification
        </h2>
        <div className="space-y-3 text-sm">
          <CvEntry
            title="Board Certification in Plastic Surgery"
            subtitle="Ministry of Health and Welfare, Republic of Korea"
            date="2020"
          />
          <CvEntry
            title="Physician License"
            subtitle="Ministry of Health and Welfare, Republic of Korea"
            date="2012"
          />
        </div>
      </section>

      {/* Professional Experience */}
      <section className="pb-10">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Professional Experience
        </h2>
        <div className="space-y-3 text-sm">
          <CvEntry
            title="Founder & Director"
            subtitle={<><a href={practiceUrl} target="_blank" rel="noopener noreferrer" className="text-accent hover:underline">LEAD Plastic Surgery Clinic website</a>, Gangnam, Seoul</>}
            date="2024 – Present"
          />
          <CvEntry
            title="Plastic Surgeon"
            subtitle="Wonderful Plastic Surgery Clinic, Gangnam, Seoul"
            date="2022 – 2024"
          />
          <CvEntry
            title="Plastic Surgeon"
            subtitle="POP Plastic Surgery Clinic, Gangnam, Seoul"
            date="2021 – 2022"
          />
          <CvEntry
            title="Plastic Surgeon"
            subtitle="THE Plastic Surgery Clinic, Gangnam, Seoul"
            date="2020 – 2021"
          />
        </div>
      </section>

      {/* Professional Memberships */}
      <section className="pb-10">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Professional Memberships
        </h2>
        <ul className="space-y-1 text-sm text-foreground">
          <li>Korean Society of Plastic and Reconstructive Surgeons</li>
          <li>Korean Society of Aesthetic Plastic Surgery</li>
        </ul>
      </section>

      {/* Peer-Reviewed Publications */}
      <section className="pb-24">
        <h2 className="text-xs font-semibold text-accent uppercase tracking-widest mb-4">
          Peer-Reviewed Publications
        </h2>
        <ol className="list-decimal list-outside ml-4 space-y-4 text-sm">
          {publications.map((pub, i) => (
            <li key={i} className="text-foreground/90 leading-relaxed pl-1">
              {pub.authors}. {pub.title}.{" "}
              <span className="italic">{pub.journal}</span>.{" "}
              {pub.year}
              {pub.volume && `;${pub.volume}`}
              {pub.issue && `(${pub.issue})`}
              {pub.pages && `:${pub.pages}`}.
              {(pub.doi || pub.pubmed) && (
                <span className="ml-1">
                  {pub.doi && (
                    <a
                      href={`https://doi.org/${pub.doi}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-accent hover:underline"
                    >
                      DOI
                    </a>
                  )}
                  {pub.doi && pub.pubmed && (
                    <span className="text-muted mx-1">&middot;</span>
                  )}
                  {pub.pubmed && (
                    <a
                      href={`https://pubmed.ncbi.nlm.nih.gov/${pub.pubmed}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-accent hover:underline"
                    >
                      PubMed
                    </a>
                  )}
                </span>
              )}
            </li>
          ))}
        </ol>
      </section>
    </div>
  );
}
