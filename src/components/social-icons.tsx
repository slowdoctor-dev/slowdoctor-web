import { SiYoutube, SiInstagram, SiThreads, SiTiktok, SiNaver } from "react-icons/si";
import type { ComponentType } from "react";

const iconMap: Record<string, ComponentType<{ className?: string }>> = {
  YouTube: SiYoutube,
  Instagram: SiInstagram,
  Threads: SiThreads,
  TikTok: SiTiktok,
  "Naver Blog": SiNaver,
};

export function SocialIcon({ label, className }: { label: string; className?: string }) {
  const Icon = iconMap[label];
  if (!Icon) return null;
  return <Icon className={className} />;
}
