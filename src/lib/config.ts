import { doctor } from "@/data/doctor";

export const SITE = {
  url: doctor.url,
  name: "slowdoctor.dev",
  title: `${doctor.name} - Plastic Surgeon & Engineer`,
  titleTemplate: `%s | ${doctor.name}`,
  ogImage: "/og-default.png",
} as const;

export const AUTHOR = {
  name: doctor.name,
  korean: doctor.alternateName,
  title: doctor.title,
  jobTitle: doctor.jobTitle,
} as const;

export const DESCRIPTIONS = {
  full: "Board-certified plastic surgeon and engineer specializing in slow-aging, scar treatment, and blepharoplasty. Practicing in Gangnam, Seoul, with a focus on long-term results over quick fixes.",
  brief: "Board-certified plastic surgeon and engineer, practicing in Gangnam, Seoul.",
} as const;

export const PRACTICE = {
  name: "LEAD Plastic Surgery",
  fullName: doctor.worksFor.fullName,
  location: doctor.worksFor.location,
  phone: doctor.worksFor.phone,
} as const;
