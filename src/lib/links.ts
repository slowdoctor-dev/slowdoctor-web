export const socialLinks = [
  { label: "YouTube", url: "https://www.youtube.com/@slowdoctor", handle: "@slowdoctor" },
  { label: "Instagram", url: "https://www.instagram.com/slowdoctor_/", handle: "@slowdoctor_" },
  { label: "Threads", url: "https://www.threads.net/@slowdoctor_", handle: "@slowdoctor_" },
  { label: "TikTok", url: "https://www.tiktok.com/@slowdoctor_", handle: "@slowdoctor_" },
  { label: "LinkedIn", url: "https://www.linkedin.com/in/slowdoctor/", handle: "@slowdoctor" },
] as const;

export const academicLinks = [
  { label: "ORCID", url: "https://orcid.org/0000-0002-4556-1536" },
  { label: "Google Scholar", url: "https://scholar.google.com/citations?user=i_e44lEAAAAJ" },
  { label: "ResearchGate", url: "https://www.researchgate.net/profile/Joonho-Lim" },
] as const;

export const practiceUrl = "https://leadps.co.kr";

export const allProfileUrls = [
  ...academicLinks.map((l) => l.url),
  ...socialLinks.map((l) => l.url),
];
