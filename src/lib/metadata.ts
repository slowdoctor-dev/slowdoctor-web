import type { Metadata } from "next";
import { AUTHOR, SITE } from "@/lib/config";

const defaultImage = {
  url: SITE.ogImage,
  width: 1200,
  height: 630,
  alt: SITE.name,
} as const;

interface PageMetadataOptions {
  title: string;
  description: string;
  path: string;
  type?: "website" | "article";
}

export function buildPageMetadata({
  title,
  description,
  path,
  type = "website",
}: PageMetadataOptions): Metadata {
  const absoluteUrl = new URL(path, SITE.url).toString();

  return {
    title,
    description,
    authors: [{ name: AUTHOR.name, url: SITE.url }],
    creator: AUTHOR.name,
    publisher: SITE.name,
    alternates: { canonical: absoluteUrl },
    openGraph: {
      title,
      description,
      url: absoluteUrl,
      type,
      siteName: SITE.name,
      locale: "en_US",
      images: [defaultImage],
    },
    twitter: {
      card: "summary_large_image",
      title,
      description,
      images: [defaultImage.url],
    },
  };
}
