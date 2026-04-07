import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Engineer",
  description:
    "Building an AI-operated clinic with Claude Code, multi-agent systems, and modern web infrastructure.",
  alternates: { canonical: "/engineer" },
};

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

export default function EngineerPage() {
  return (
    <div className="mx-auto max-w-3xl px-6">
      {/* Header */}
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Engineer
        </h1>
        <p className="mt-4 text-lg text-muted leading-relaxed max-w-xl">
          I run my clinic entirely with AI -- from content creation to
          operations.
        </p>
      </section>

      {/* What I Build */}
      <section className="pb-16">
        <h2 className="text-sm font-medium text-accent uppercase tracking-wider mb-6">
          What I Build
        </h2>
        <div className="space-y-5 text-foreground/90 leading-relaxed">
          <p>
            LEAD Plastic Surgery operates with a single physician and zero
            dedicated staff for marketing, design, or IT. Instead, I built a
            multi-agent AI system powered by Claude Code that handles
            everything: market research, blog content, graphic design,
            administrative tasks, and software development.
          </p>
          <p>
            The system uses the Model Context Protocol (MCP) to connect Claude
            to over 20 local and external tools -- from Naver Search and Google
            Search Console to Playwright browsers and custom APIs. Each agent
            has a defined role, its own instruction set, and access to shared
            knowledge. The orchestrator coordinates them, and I review and
            approve the output.
          </p>
          <p>
            This is not a weekend experiment. It is how the clinic actually
            runs, every day. The goal is to prove that a solo practitioner can
            deliver enterprise-level operations by treating AI as infrastructure,
            not a novelty.
          </p>
        </div>
      </section>

      {/* Projects */}
      <section className="pb-16">
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

      {/* Placeholder */}
      <section className="pb-24">
        <p className="text-sm text-muted">More details coming soon.</p>
      </section>
    </div>
  );
}
