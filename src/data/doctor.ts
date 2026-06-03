export const doctor = {
  name: "Joonho Lim",
  alternateName: "임준호",
  givenName: "Joonho",
  familyName: "Lim",
  title: "Joonho Lim, M.D.",
  jobTitle: "Board-Certified Plastic Surgeon",
  description:
    "Board-certified plastic surgeon specializing in slow-aging, scar treatment, and natural eye surgery. Founder of LEAD Plastic Surgery Clinic in Seoul.",
  id: "https://slowdoctor.dev/#person",
  url: "https://slowdoctor.dev",
  image: "https://slowdoctor.dev/images/profile.jpg",

  alumniOf: [
    { name: "Seoul Science High School", type: "EducationalOrganization" as const },
    { name: "Seoul National University College of Medicine", type: "CollegeOrUniversity" as const },
  ],

  training:
    "Internship and Plastic Surgery Residency at Seoul National University Hospital",

  credential: {
    name: "Plastic Surgery Specialist",
    category: "Board Certification",
    recognizedBy: "Korean Medical Association",
  },

  memberOf: [
    "Korean Society of Plastic and Reconstructive Surgeons",
    "Korean Society for Aesthetic Plastic Surgery",
  ],

  worksFor: {
    id: "https://leadps.co.kr/#organization",
    name: "LEAD Plastic Surgery Clinic",
    alternateName: "리드성형외과의원",
    url: "https://leadps.co.kr",
    // Human-readable label. "Dogok-dong" is the legal dong (법정동); the
    // building's road-name address is "21 Eonju-ro 30-gil" (see `address`).
    // Both describe the same location — do not "fix" one to match the other.
    location: "Dogok-dong, Gangnam-gu, Seoul, South Korea",
    // Canonical structured address — single source of truth for Schema.org.
    // Road-name address: 서울 강남구 언주로30길 21 지하1층 B101-31-1호 (도곡동, 06292).
    address: {
      streetAddress: "21 Eonju-ro 30-gil, B101-31-1",
      addressLocality: "Gangnam-gu",
      addressRegion: "Seoul",
      postalCode: "06292",
      addressCountry: "KR",
    },
    phone: "+82-2-6953-3231",
  },

  knowsAbout: [
    "Anti-aging medicine",
    "Non-surgical facial rejuvenation",
    "Ultrasound skin lifting",
    "Radiofrequency skin tightening",
    "Dermal filler",
    "Botulinum toxin injection",
    "Scar treatment and revision",
    "Keloid management",
    "Non-incisional blepharoplasty",
    "Lower blepharoplasty with fat repositioning",
    "Endoscopic brow lift",
  ],

  medicalSpecialty: "PlasticSurgery",

  sameAs: [
    "https://orcid.org/0000-0002-4556-1536",
    "https://scholar.google.com/citations?user=i_e44lEAAAAJ",
    "https://www.researchgate.net/profile/Joonho-Lim",
    "https://www.linkedin.com/in/slowdoctor/",
    "https://www.youtube.com/@slowdoctor",
    "https://www.instagram.com/slowdoctor_/",
    "https://www.threads.net/@slowdoctor_",
    "https://www.tiktok.com/@slowdoctor_",
    "https://blog.naver.com/plastic_talks",
  ],
} as const;
