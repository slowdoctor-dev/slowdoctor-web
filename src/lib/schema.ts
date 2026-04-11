import { doctor } from "@/data/doctor";

export function generatePersonSchema() {
  return {
    "@context": "https://schema.org",
    "@type": ["Person", "Physician"],
    "@id": doctor.id,
    name: doctor.name,
    alternateName: doctor.alternateName,
    givenName: doctor.givenName,
    familyName: doctor.familyName,
    jobTitle: doctor.jobTitle,
    description: doctor.description,
    url: doctor.url,
    image: doctor.image || undefined,
    medicalSpecialty: doctor.medicalSpecialty,
    alumniOf: doctor.alumniOf.map((school) => ({
      "@type": school.type,
      name: school.name,
    })),
    worksFor: {
      "@type": ["MedicalBusiness", "LocalBusiness"],
      "@id": doctor.worksFor.id,
      name: doctor.worksFor.name,
      url: doctor.worksFor.url,
    },
    memberOf: doctor.memberOf.map((org) => ({
      "@type": "Organization",
      name: org,
    })),
    hasCredential: {
      "@type": "EducationalOccupationalCredential",
      credentialCategory: doctor.credential.category,
      name: doctor.credential.name,
      recognizedBy: {
        "@type": "Organization",
        name: doctor.credential.recognizedBy,
      },
    },
    knowsAbout: doctor.knowsAbout,
    sameAs: doctor.sameAs,
  };
}

export function generatePracticeSchema() {
  return {
    "@context": "https://schema.org",
    "@type": ["MedicalBusiness", "LocalBusiness"],
    "@id": doctor.worksFor.id,
    name: doctor.worksFor.name,
    alternateName: doctor.worksFor.alternateName,
    url: doctor.worksFor.url,
    telephone: doctor.worksFor.phone,
    // Structured address for Schema.org — derived from doctor.worksFor.location
    address: {
      "@type": "PostalAddress",
      addressLocality: "Gangnam-gu",
      addressRegion: "Seoul",
      addressCountry: "KR",
    },
    medicalSpecialty: doctor.medicalSpecialty,
    founder: {
      "@type": "Person",
      "@id": doctor.id,
      name: doctor.name,
    },
  };
}
