export const socialLinks = [
  { label: "YouTube", url: "https://www.youtube.com/@slowdoctor", handle: "@slowdoctor" },
  { label: "Instagram", url: "https://www.instagram.com/slowdoctor_/", handle: "@slowdoctor_" },
  { label: "Threads", url: "https://www.threads.net/@slowdoctor_", handle: "@slowdoctor_" },
  { label: "TikTok", url: "https://www.tiktok.com/@slowdoctor_", handle: "@slowdoctor_" },
  { label: "LinkedIn", url: "https://www.linkedin.com/in/slowdoctor/" },
] as const;

export const academicLinks = [
  { label: "ORCID", url: "https://orcid.org/0009-0002-7299-8315" },
  { label: "Google Scholar", url: "https://scholar.google.com/citations?user=XSsW3yIAAAAJ" },
  { label: "ResearchGate", url: "https://www.researchgate.net/profile/Joonho-Lim-9" },
] as const;

export const practiceUrl = "https://leadps.co.kr";

export const allProfileUrls = [
  ...academicLinks.map((l) => l.url),
  ...socialLinks.map((l) => l.url),
];
