import Link from "next/link";

export default function NotFound() {
  return (
    <div className="mx-auto max-w-3xl px-6 pt-24 pb-24 sm:pt-32">
      <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">404</h1>
      <p className="mt-4 text-lg text-muted">Page not found.</p>
      <Link
        href="/"
        className="mt-8 inline-block text-sm text-accent hover:underline"
      >
        &larr; Back to home
      </Link>
    </div>
  );
}
