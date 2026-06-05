import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { buildPageMetadata } from "@/lib/metadata";
import { githubUrl, repoUrl } from "@/lib/links";

export const metadata: Metadata = buildPageMetadata({
  title: "Engineer",
  description:
    "Medical informatics, infodemiology, and health IT. Building custom scheduling, inventory, and digital signage tools for a solo plastic surgery clinic.",
  path: "/engineer",
});

const interests = [
  {
    name: "Medical Informatics",
    description:
      "Applying computing to clinical workflows — charting, records, and knowledge systems that actually fit how medicine works.",
  },
  {
    name: "Infodemiology",
    description:
      "How health information spreads online — using search and social data to understand what patients are actually looking for.",
  },
  {
    name: "Health IT",
    description:
      "Building custom tools for a solo clinic — scheduling, inventory, patient flow, and digital signage — instead of relying on off-the-shelf solutions.",
  },
];

const projects: {
  name: string;
  description: string;
  repo?: string;
}[] = [
  {
    name: "lead-signage",
    description: "Digital signage system for in-clinic displays.",
  },
  {
    name: "lead-inventory",
    description: "Medical supply inventory management.",
  },
  {
    name: "lead-viewer",
    description: "Content viewer for clinic materials.",
  },
  {
    name: "workspace-md",
    description:
      "A workspace-topology spec — a sibling to agents.md defining how a human and an AI agent share a directory so memory and skills compound with use.",
    repo: "https://github.com/slowdoctor-dev/workspace-md",
  },
  {
    name: "seasoned-hand",
    description:
      "An open-source autonomous agent platform — deep task execution with learning that persists across sessions. Self-hosted and model-agnostic.",
    repo: "https://github.com/slowdoctor-dev/seasoned-hand",
  },
  {
    name: "ashy-walnut-desk",
    description:
      "A digital front-desk for regulated-service businesses — identity, interaction, and knowledge with AI augmentation and human approval.",
    repo: "https://github.com/slowdoctor-dev/ashy-walnut-desk",
  },
  {
    name: "slowdoctor.dev",
    description: "This site.",
    repo: repoUrl,
  },
];

export default function EngineerPage() {
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Engineer", href: "/engineer" },
  ]);

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
          Engineer
        </h1>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          Medicine gave me the problems. Engineering gives me the tools.
        </p>
        <p className="mt-4 text-sm">
          <a
            href={githubUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="text-accent hover:underline"
          >
            github.com/slowdoctor-dev &#8599;
          </a>
        </p>
      </section>

      {/* Thesis */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Thesis
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            I&apos;ve always been curious about computing — not just using
            tools, but understanding how they work underneath. Medicine is
            where I practice, but engineering is how I think.
          </p>
          <p>
            I&apos;d rather build something myself than rely on a tool that
            almost works. Most of what I make is for my own clinic, but the
            mindset applies everywhere — understand the problem first,
            then write the solution.
          </p>
        </div>
      </section>

      {/* Interests */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Interests
        </h2>
        <div className="space-y-3">
          {interests.map((interest) => (
            <div
              key={interest.name}
              className="rounded-2xl border border-border bg-card p-5"
            >
              <h3 className="text-base font-semibold text-foreground">
                {interest.name}
              </h3>
              <p className="mt-2 text-sm text-muted leading-relaxed">
                {interest.description}
              </p>
            </div>
          ))}
        </div>
      </section>

      {/* Projects */}
      <section className="pb-24">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Projects
        </h2>
        <div className="space-y-3">
          {projects.map((project) => (
            <div
              key={project.name}
              className="flex items-start gap-4 rounded-2xl border border-border bg-card p-4"
            >
              <div className="flex-1 min-w-0">
                <h3 className="text-sm font-semibold font-mono">
                  {project.name}
                </h3>
                <p className="mt-1 text-sm text-muted">
                  {project.description}
                </p>
              </div>
              {project.repo && (
                <a
                  href={project.repo}
                  target="_blank"
                  rel="noopener noreferrer"
                  aria-label={`${project.name} source on GitHub`}
                  className="shrink-0 text-xs text-accent hover:underline"
                >
                  Source &#8599;
                </a>
              )}
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
