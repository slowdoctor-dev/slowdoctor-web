import type { Metadata } from "next";
import { Inter, Plus_Jakarta_Sans, Gowun_Dodum, Noto_Sans_KR } from "next/font/google";
import Link from "next/link";
import { JsonLd } from "@/components/json-ld";
import { socialLinks, allProfileUrls, practiceUrl } from "@/lib/links";
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
    default: "Joonho Lim - Plastic Surgeon & Engineer",
    template: "%s | Joonho Lim",
  },
  description:
    "Board-certified plastic surgeon and engineer specializing in slow-aging, scar treatment, and blepharoplasty. Building an AI-operated clinic.",
  metadataBase: new URL("https://slowdoctor.dev"),
  openGraph: {
    title: "Joonho Lim - Plastic Surgeon & Engineer",
    description:
      "Board-certified plastic surgeon and engineer building an AI-operated clinic.",
    url: "https://slowdoctor.dev",
    siteName: "slowdoctor.dev",
    locale: "en_US",
    type: "website",
  },
  twitter: {
    card: "summary",
    title: "Joonho Lim - Plastic Surgeon & Engineer",
    description:
      "Board-certified plastic surgeon and engineer building an AI-operated clinic.",
  },
  robots: {
    index: true,
    follow: true,
  },
};

const personSchema = {
  "@context": "https://schema.org",
  "@type": ["Person", "Physician"],
  name: "Joonho Lim",
  alternateName: "\uc784\uc900\ud638",
  jobTitle: "Board-Certified Plastic Surgeon",
  url: "https://slowdoctor.dev",
  description:
    "Board-certified plastic surgeon and engineer specializing in slow-aging, scar treatment, and blepharoplasty. Building an AI-operated clinic.",
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
  { href: "/physician", label: "Physician" },
  { href: "/engineer", label: "Engineer" },
  { href: "/links", label: "Links" },
  { href: "/blog", label: "Blog" },
];

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${inter.variable} ${plusJakartaSans.variable} ${gowunDodum.variable} ${notoSansKR.variable} h-full antialiased`}>
      <head>
        <JsonLd data={personSchema} />
      </head>
      <body className="min-h-full flex flex-col">
        <header className="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-md">
          <nav aria-label="Main" className="mx-auto flex max-w-3xl items-center justify-between px-6 py-4">
            <Link
              href="/"
              className="text-sm font-semibold tracking-tight text-foreground hover:text-accent transition-colors"
            >
              Joonho Lim
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
              &copy; 2026 Joonho Lim
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
