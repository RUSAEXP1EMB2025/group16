import type { SiteType } from "../types/content";

export function detectSiteType(url: string): SiteType {
  const hostname = new URL(url).hostname.toLowerCase();

  if (hostname.includes("youtube.com") || hostname.includes("youtu.be")) {
    return "youtube";
  }

  if (hostname.includes("netflix.com")) {
    return "netflix";
  }

  return "generic";
}

export function isTargetSite(url: string): boolean {
  const siteType = detectSiteType(url);
  return siteType === "youtube" || siteType === "netflix" || siteType === "generic";
}
