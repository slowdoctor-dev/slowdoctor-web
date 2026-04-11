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
    location: "Dogok-dong, Gangnam-gu, Seoul, South Korea",
    phone: "+82-2-6953-3231",
  },

  knowsAbout: [
    "Slow-aging",
    "Energy-based devices in aesthetic medicine",
    "HIFU (High-Intensity Focused Ultrasound)",
    "Radiofrequency skin tightening",
    "Scar treatment and revision",
    "Keloid management",
    "Non-incisional double eyelid surgery",
    "Lower blepharoplasty with fat repositioning",
    "Endoscopic brow lift",
    "Facial rejuvenation",
    "Dermal filler",
    "Botulinum toxin injection",
    "Biostimulator therapy",
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
