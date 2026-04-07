import { highlight } from "sugar-high";
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
  const isExternal = href?.startsWith("http");

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
