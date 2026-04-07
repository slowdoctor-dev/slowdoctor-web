const DATE_ONLY_PATTERN = /^\d{4}-\d{2}-\d{2}$/;

function parseDateOnly(value: string, fileName: string) {
  if (!DATE_ONLY_PATTERN.test(value)) {
    throw new Error(
      `Invalid date in ${fileName}: expected YYYY-MM-DD, received "${value}"`,
    );
  }

  const [year, month, day] = value.split("-").map(Number);
  const parsed = new Date(Date.UTC(year, month - 1, day));

  if (
    parsed.getUTCFullYear() !== year ||
    parsed.getUTCMonth() !== month - 1 ||
    parsed.getUTCDate() !== day
  ) {
    throw new Error(`Invalid calendar date in ${fileName}: "${value}"`);
  }

  return parsed;
}

function formatDate(value: string, fileName: string) {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
    timeZone: "UTC",
  }).format(parseDateOnly(value, fileName));
}

module.exports = { parseDateOnly, formatDate };
