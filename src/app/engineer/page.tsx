import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { buildPageMetadata } from "@/lib/metadata";

export const metadata: Metadata = buildPageMetadata({
  title: "Engineer",
  description:
    "Building an AI-operated clinic with Claude Code, multi-agent systems, and modern web infrastructure.",
  path: "/engineer",
});

const interests = [
  {
    name: "Medical Informatics",
    description:
      "Applying computing and data systems to clinical workflows -- from AI-assisted charting to structured knowledge bases.",
  },
  {
    name: "Infodemiology",
    description:
      "Studying how health information spreads online and using search/social data to understand public health behavior.",
  },
  {
    name: "AI-operated Healthcare",
    description:
      "Proving that a solo practitioner can deliver enterprise-level operations by treating AI as infrastructure, not a novelty.",
  },
];

const techStack = [
  "Next.js",
  "TypeScript",
  "Claude Code",
  "MCP",
  "Docker",
  "WSL2",
  "Node.js",
  "SQLite",
  "Tailwind",
];

const projects = [
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
    name: "medical-scribe",
    description: "AI-powered medical note transcription.",
    tag: "in progress",
  },
  {
    name: "slowdoctor.dev",
    description: "This site.",
    tag: "meta",
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
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Engineer
        </h1>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          Medicine gave me the problems. Engineering gives me the tools.
        </p>
      </section>

      {/* Thesis */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Thesis
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            Healthcare is full of workflows that were designed for a world
            before software -- charting, scheduling, inventory, patient
            communication, even clinical decision-making. Most solutions come
            from engineers who have never been in a consultation room. The
            result is tools that technically work but miss the context that
            matters.
          </p>
          <p>
            My current experiment: run an entire clinic with a single physician
            and a multi-agent AI system, no dedicated marketing or IT staff.
            Not to prove that AI replaces people, but to find out which parts
            of medicine genuinely benefit from automation and which ones
            should never be delegated.
          </p>
          <p>
            This is a thesis, not an answer. I am still testing it.
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
              className="rounded-lg border border-border bg-card p-5"
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

      {/* Tech Stack */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          Tech Stack
        </h2>
        <div className="flex flex-wrap gap-2">
          {techStack.map((tech) => (
            <span
              key={tech}
              className="text-sm text-foreground/80 border border-border rounded-full px-3 py-1"
            >
              {tech}
            </span>
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
              className="flex items-start gap-4 rounded-lg border border-border bg-card p-4"
            >
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <h3 className="text-sm font-semibold font-mono">
                    {project.name}
                  </h3>
                  {project.tag && (
                    <span className="text-xs text-accent bg-accent-muted px-2 py-0.5 rounded-full">
                      {project.tag}
                    </span>
                  )}
                </div>
                <p className="mt-1 text-sm text-muted">
                  {project.description}
                </p>
              </div>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
