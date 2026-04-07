import type { Metadata } from "next";

const defaultImage = {
  url: "/og-default.png",
  width: 1200,
  height: 630,
  alt: "slowdoctor.dev",
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
  return {
    title,
    description,
    alternates: { canonical: path },
    openGraph: {
      title,
      description,
      url: path,
      type,
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
