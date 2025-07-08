import type { SiteDataRequest, SiteExtractor } from "../types/content";

export class YouTubeExtractor implements SiteExtractor {
  canExtract(url: string): boolean {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.includes("youtube.com") || hostname.includes("youtu.be");
  }

  async extract(): Promise<SiteDataRequest | undefined> {
    try {
      const url = window.location.href;

      const videoId = this.extractVideoId(url);
      if (!videoId) {
        return;
      }

      return {
        Youtube: { url }
      };
    } catch (error) {
      console.error("YouTube extraction failed:", error);
      return;
    }
  }

  private extractVideoId(url: string): string | undefined {
    const patterns = [
      /(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]{11})/,
      /youtube\.com\/embed\/([a-zA-Z0-9_-]{11})/,
      /youtube\.com\/v\/([a-zA-Z0-9_-]{11})/
    ];

    for (const pattern of patterns) {
      const match = url.match(pattern);
      if (match) {
        return match[1];
      }
    }

    return;
  }
}
