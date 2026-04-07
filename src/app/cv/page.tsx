import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { practiceUrl } from "@/lib/links";
import { buildPageMetadata } from "@/lib/metadata";

export const metadata: Metadata = buildPageMetadata({
  title: "CV",
  description:
    "Curriculum vitae of Joonho Lim -- education, training, professional experience, and publications.",
  path: "/cv",
});

interface Publication {
  title: string;
  authors: string;
  journal: string;
  year: number;
  volume?: string;
  issue?: string;
  pages?: string;
  doi?: string;
  pubmed?: string;
}

const publications: Publication[] = [
  {
    title:
      "Oncologic outcomes after immediate breast reconstruction following mastectomy: comparison of implant and flap using propensity score matching",
    authors:
      "Ha JH, Hong KY, Lee HB, Moon HG, Han W, Noh DY, Lim J, Yoon S, Chang H, Jin US",
    journal: "BMC Cancer",
    year: 2020,
    volume: "20",
    issue: "1",
    pages: "78",
    doi: "10.1186/s12885-020-6568-2",
    pubmed: "32000718",
  },
  {
    title:
      "Flap reconstruction of soft tissue defect after resecting a huge hemangioma of the nose",
    authors: "Lim J, Oh J, Eun S",
    journal: "Archives of Craniofacial Surgery",
    year: 2020,
    volume: "21",
    issue: "1",
    pages: "69-72",
    doi: "10.7181/acfs.2019.00668",
    pubmed: "32126625",
  },
  {
    title:
      "The reconstruction of the central tubercle in bilateral cleft lips: bilateral lateral mucosal advancement flap with reinforcement of the orbicularis oris muscle",
    authors: "Chung J, Lim J, Kim S, Koo Y",
    journal: "Annals of Plastic Surgery",
    year: 2019,
    volume: "83",
    issue: "6",
    pages: "655-659",
    doi: "10.1097/SAP.0000000000002012",
    pubmed: "31397684",
  },
  {
    title:
      "A case of multifocal primary cutaneous anaplastic large cell lymphoma managed without surgical treatment",
    authors: "Lim J, Park E, Eun S",
    journal: "Korean Journal of Head & Neck Oncology",
    year: 2019,
    volume: "35",
    issue: "2",
    pages: "77-81",
    doi: "10.21593/kjhno/2019.35.2.77",
  },
  {
    title:
      "Correlation between speech outcomes and the amount of maxillary advancement after orthognathic surgery in patients with cleft lip and palate",
    authors: "Chung J, Lim J, Park H, Yoo A, Kim S, Koo Y",
    journal: "The Journal of Craniofacial Surgery",
    year: 2019,
    volume: "30",
    issue: "6",
    pages: "1855-1858",
    doi: "10.1097/SCS.0000000000005623",
    pubmed: "31107383",
  },
  {
    title:
      "Portable ultrasonic surgery system for chronic wounds: a multicenter randomized controlled clinical trial and in vitro characterization",
    authors:
      "Pak C, Lim J, Kim BK, Kim H, Park S, Mun GH, Kim JT, Jeong JH, Heo CY",
    journal: "Journal of Wound Management and Research",
    year: 2019,
    volume: "15",
    issue: "1",
    pages: "5-10",
    doi: "10.22467/jwmr.2019.00584",
  },
  {
    title:
      "Immediate lower extremity reconstruction using an anterolateral thigh free flap with simultaneous interposition graft of descending branches of lateral circumflex femoral vessels",
    authors: "Lim J, Kwon H, Lee KM, Pak C",
    journal: "The International Journal of Lower Extremity Wounds",
    year: 2019,
    volume: "18",
    issue: "1",
    pages: "89-93",
    doi: "10.1177/1534734618819932",
    pubmed: "31064286",
  },
];

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

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />

      {/* Header */}
      <section className="pt-24 pb-8 sm:pt-32 sm:pb-10 text-center">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
          Joonho Lim, M.D.
        </h1>
        <p className="mt-1 text-muted text-sm">임준호</p>
        <p className="mt-3 text-sm text-muted">
          LEAD Plastic Surgery Clinic &middot; Dogok-dong, Gangnam-gu, Seoul,
          South Korea
        </p>
        <p className="mt-1 text-sm text-muted">
          <a
            href={practiceUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="text-accent hover:underline"
          >
            leadps.co.kr
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
            date="2025 -- Present"
          />
          <CvEntry
            title="Seoul National University College of Medicine"
            subtitle="Doctor of Medicine (M.D.)"
            date="2006 -- 2012"
          />
          <CvEntry
            title="Seoul Science High School"
            subtitle="Early graduation, Valedictorian"
            date="2004 -- 2006"
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
            date="2016 -- 2020"
          />
          <CvEntry
            title="Internship"
            subtitle="Seoul National University Hospital"
            date="2012 -- 2013"
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
            date="2013 -- 2016"
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
            subtitle={<><a href={practiceUrl} target="_blank" rel="noopener noreferrer" className="text-accent hover:underline">LEAD Plastic Surgery Clinic</a>, Gangnam, Seoul</>}
            date="2024 -- Present"
          />
          <CvEntry
            title="Plastic Surgeon"
            subtitle="Wonderful Plastic Surgery Clinic, Gangnam, Seoul"
            date="2022 -- 2024"
          />
          <CvEntry
            title="Plastic Surgeon"
            subtitle="POP Plastic Surgery Clinic, Gangnam, Seoul"
            date="2021 -- 2022"
          />
          <CvEntry
            title="Plastic Surgeon"
            subtitle="THE Plastic Surgery Clinic, Gangnam, Seoul"
            date="2020 -- 2021"
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
