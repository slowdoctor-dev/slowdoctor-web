"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navLinks = [
  { href: "/cv", label: "CV" },
  { href: "/physician", label: "Physician" },
  { href: "/engineer", label: "Engineer" },
  { href: "/blog", label: "Blog" },
  { href: "/links", label: "Links" },
] as const;

function isActivePath(pathname: string, href: string) {
  return pathname === href || pathname.startsWith(`${href}/`);
}

export function NavLinks() {
  const pathname = usePathname();

  return (
    <div className="flex items-center gap-5 sm:gap-6">
      {navLinks.map((link) => {
        const isActive = isActivePath(pathname, link.href);

        return (
          <Link
            key={link.href}
            href={link.href}
            aria-current={isActive ? "page" : undefined}
            className={`text-sm transition-colors ${
              isActive
                ? "font-medium text-foreground"
                : "text-muted hover:text-foreground"
            }`}
          >
            {link.label}
          </Link>
        );
      })}
    </div>
  );
}
