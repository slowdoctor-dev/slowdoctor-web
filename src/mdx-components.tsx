import { highlight } from "sugar-high";
import NextLink from "next/link";
import type { ComponentPropsWithoutRef, ReactNode } from "react";
import type { MDXComponents } from "mdx/types";

function Pre({ children, ...props }: ComponentPropsWithoutRef<"pre">) {
  return <pre {...props}>{children}</pre>;
}

function Code({
  children,
  className,
  ...props
}: ComponentPropsWithoutRef<"code">) {
  if (!className?.startsWith("language-")) {
    return (
      <code className={className} {...props}>
        {children}
      </code>
    );
  }

  const content = Array.isArray(children)
    ? children.join("")
    : String(children ?? "");

  return (
    <code
      className={className}
      dangerouslySetInnerHTML={{ __html: highlight(content) }}
      {...props}
    />
  );
}

function Link({
  href,
  children,
  ...props
}: ComponentPropsWithoutRef<"a"> & { children?: ReactNode }) {
  // Only same-origin path links use client-side routing. Everything else
  // (http(s), protocol-relative //, mailto:, tel:, #hash) stays a plain <a>.
  // The inline check narrows `href` to a defined string for NextLink.
  if (href?.startsWith("/")) {
    return (
      <NextLink href={href} {...props}>
        {children}
      </NextLink>
    );
  }

  const isExternal = href?.startsWith("http") || href?.startsWith("//");

  return (
    <a
      href={href}
      target={isExternal ? "_blank" : undefined}
      rel={isExternal ? "noopener noreferrer" : undefined}
      {...props}
    >
      {children}
    </a>
  );
}

const components: MDXComponents = {
  a: Link,
  code: Code,
  pre: Pre,
};

export function useMDXComponents(): MDXComponents {
  return components;
}
