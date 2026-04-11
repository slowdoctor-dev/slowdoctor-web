import type { Axes } from "@/lib/blog";

const axes: { key: keyof Axes; label: string }[] = [
  { key: "physician", label: "Physician" },
  { key: "engineer", label: "Engineer" },
  { key: "life", label: "Life" },
];

export function AxisBar({ values }: { values: Axes }) {
  return (
    <div role="group" aria-label="Post axes" className="space-y-1.5">
      {axes.map(({ key, label }) => (
        <div key={key} className="flex items-center gap-2 text-xs">
          <span className="w-16 text-muted shrink-0">{label}</span>
          <div
            aria-hidden="true"
            className="flex-1 h-1.5 rounded-full bg-border overflow-hidden"
          >
            <div
              className="h-full rounded-full bg-accent"
              style={{ width: `${values[key] * 10}%` }}
            />
          </div>
          <span className="w-4 text-right text-muted tabular-nums">
            {values[key]}
          </span>
        </div>
      ))}
    </div>
  );
}
