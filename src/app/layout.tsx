import type { Metadata } from "next";
import { Inter, Plus_Jakarta_Sans, Gowun_Dodum, Noto_Sans_KR } from "next/font/google";
import Link from "next/link";
import { JsonLd } from "@/components/json-ld";
import { socialLinks, allProfileUrls, practiceUrl } from "@/lib/links";
import { SITE, AUTHOR, DESCRIPTIONS } from "@/lib/config";
import "./globals.css";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: "swap",
});

const plusJakartaSans = Plus_Jakarta_Sans({
  variable: "--font-jakarta",
  subsets: ["latin"],
  display: "swap",
});

const gowunDodum = Gowun_Dodum({
  variable: "--font-gowun",
  weight: "400",
  subsets: ["latin"],
  display: "swap",
});

const notoSansKR = Noto_Sans_KR({
  variable: "--font-noto-kr",
  subsets: ["latin"],
  display: "swap",
});

export const metadata: Metadata = {
  title: {
    default: SITE.title,
    template: SITE.titleTemplate,
  },
  description: DESCRIPTIONS.full,
  metadataBase: new URL(SITE.url),
  openGraph: {
    title: SITE.title,
    description: DESCRIPTIONS.brief,
    url: SITE.url,
    siteName: SITE.name,
    locale: "en_US",
    type: "website",
    images: [
      {
        url: SITE.ogImage,
        width: 1200,
        height: 630,
        alt: SITE.name,
      },
    ],
  },
  twitter: {
    card: "summary",
    title: SITE.title,
    description: DESCRIPTIONS.brief,
    images: [SITE.ogImage],
  },
  robots: {
    index: true,
    follow: true,
  },
};

const personSchema = {
  "@context": "https://schema.org",
  "@type": ["Person", "Physician"],
  name: AUTHOR.name,
  alternateName: AUTHOR.korean,
  jobTitle: AUTHOR.jobTitle,
  url: SITE.url,
  description: DESCRIPTIONS.full,
  worksFor: {
    "@type": "MedicalBusiness",
    name: "LEAD Plastic Surgery",
    url: practiceUrl,
  },
  alumniOf: [
    {
      "@type": "CollegeOrUniversity",
      name: "Seoul National University College of Medicine",
    },
  ],
  medicalSpecialty: "PlasticSurgery",
  sameAs: allProfileUrls,
};

const navLinks = [
  { href: "/cv", label: "CV" },
  { href: "/physician", label: "Physician" },
  { href: "/engineer", label: "Engineer" },
  { href: "/blog", label: "Blog" },
  { href: "/links", label: "Links" },
];

const currentYear = new Date().getFullYear();

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${inter.variable} ${plusJakartaSans.variable} ${gowunDodum.variable} ${notoSansKR.variable} h-full antialiased`}>
      <head>
        <link
          rel="alternate"
          type="application/rss+xml"
          title="Blog"
          href="/feed.xml"
        />
        <JsonLd data={personSchema} />
      </head>
      <body className="min-h-full flex flex-col">
        <header className="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-md">
          <nav aria-label="Main" className="mx-auto flex max-w-3xl items-center justify-between px-6 py-4">
            <Link
              href="/"
              className="text-sm font-semibold tracking-tight text-foreground hover:text-accent transition-colors"
            >
              slowdoctor.dev
            </Link>
            <div className="flex items-center gap-6">
              {navLinks.map((link) => (
                <Link
                  key={link.href}
                  href={link.href}
                  className="text-sm text-muted hover:text-foreground transition-colors"
                >
                  {link.label}
                </Link>
              ))}
            </div>
          </nav>
        </header>

        <main className="flex-1">{children}</main>

        <footer aria-label="Site footer" className="border-t border-border">
          <div className="mx-auto max-w-3xl px-6 py-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
            <p className="text-sm text-muted">
              &copy; {currentYear} {AUTHOR.name}
            </p>
            <div className="flex items-center gap-5 text-sm text-muted">
              {socialLinks.slice(0, 4).map((link) => (
                <a
                  key={link.label}
                  href={link.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="hover:text-foreground transition-colors"
                >
                  {link.label}
                </a>
              ))}
            </div>
          </div>
        </footer>
      </body>
    </html>
  );
}
