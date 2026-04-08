import type { Metadata } from "next";
import { Inter, Plus_Jakarta_Sans, Gowun_Dodum, Noto_Sans_KR } from "next/font/google";
import Link from "next/link";
import { NavLinks } from "@/components/nav-links";
import { socialLinks } from "@/lib/links";
import { SITE, AUTHOR, DESCRIPTIONS } from "@/lib/config";
import { SocialIcon } from "@/components/social-icons";
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
  authors: [{ name: AUTHOR.name, url: SITE.url }],
  creator: AUTHOR.name,
  publisher: SITE.name,
  alternates: {
    types: {
      "application/rss+xml": "/feed.xml",
    },
  },
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
      </head>
      <body className="min-h-full flex flex-col">
        <a
          href="#main-content"
          className="skip-link"
        >
          Skip to content
        </a>
        <header className="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-md">
          <nav aria-label="Main" className="mx-auto flex max-w-3xl flex-col items-center gap-2 px-6 py-4 sm:flex-row sm:justify-between sm:gap-0">
            <Link
              href="/"
              className="text-sm font-semibold tracking-tight text-foreground hover:text-accent transition-colors border border-border rounded-md px-2.5 py-1"
            >
              slowdoctor.dev
            </Link>
            <NavLinks />
          </nav>
        </header>

        <main id="main-content" className="flex-1">
          {children}
        </main>

        <footer aria-label="Site footer" className="border-t border-border">
          <div className="mx-auto max-w-3xl px-6 py-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
            <p className="text-sm text-muted">
              &copy; {currentYear} {AUTHOR.name}
            </p>
            <div className="flex items-center gap-4">
              {socialLinks.map((link) => (
                <a
                  key={link.label}
                  href={link.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  aria-label={link.label}
                  className="text-muted hover:text-foreground transition-colors"
                >
                  <SocialIcon label={link.label} className="w-4 h-4" />
                </a>
              ))}
            </div>
          </div>
        </footer>
      </body>
    </html>
  );
}
