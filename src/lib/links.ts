import { doctor } from "@/data/doctor";

export const socialLinks = [
  { label: "YouTube", url: "https://www.youtube.com/@slowdoctor", handle: "@slowdoctor" },
  { label: "Instagram", url: "https://www.instagram.com/slowdoctor_/", handle: "@slowdoctor_" },
  { label: "Threads", url: "https://www.threads.net/@slowdoctor_", handle: "@slowdoctor_" },
  { label: "TikTok", url: "https://www.tiktok.com/@slowdoctor_", handle: "@slowdoctor_" },
  { label: "Naver Blog", url: "https://blog.naver.com/plastic_talks", handle: "plastic_talks" },
] as const;

export const medicalLinks = [
  { label: "ORCID", url: "https://orcid.org/0000-0002-4556-1536", detail: "0000-0002-4556-1536" },
  { label: "Google Scholar", url: "https://scholar.google.com/citations?user=i_e44lEAAAAJ", detail: "Publications" },
  { label: "ResearchGate", url: "https://www.researchgate.net/profile/Joonho-Lim", detail: "Profile" },
  { label: "LinkedIn", url: "https://www.linkedin.com/in/slowdoctor/", detail: "@slowdoctor" },
] as const;

export const practiceUrl = doctor.worksFor.url;
