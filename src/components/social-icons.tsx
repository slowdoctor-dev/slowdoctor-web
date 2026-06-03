import { SiYoutube, SiInstagram, SiThreads, SiTiktok, SiNaver, SiGithub } from "react-icons/si";
import type { IconType } from "react-icons";

const iconMap: Record<string, IconType> = {
  YouTube: SiYoutube,
  Instagram: SiInstagram,
  Threads: SiThreads,
  TikTok: SiTiktok,
  "Naver Blog": SiNaver,
  GitHub: SiGithub,
};

export function SocialIcon({ label, className }: { label: string; className?: string }) {
  const Icon = iconMap[label];
  if (!Icon) return null;
  return <Icon aria-hidden="true" className={className} focusable="false" />;
}
