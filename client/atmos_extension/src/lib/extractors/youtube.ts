import type { ExtractedContent, SiteExtractor } from "../types/content";

export class YouTubeExtractor implements SiteExtractor {
  canExtract(url: string): boolean {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.includes("youtube.com") || hostname.includes("youtu.be");
  }

  async extract(): Promise<ExtractedContent | null> {
    try {
      const currentUrl = window.location.href;

      const videoId = this.extractVideoId(currentUrl);
      if (!videoId) {
        return null;
      }

      return {
        type: "url",
        data: currentUrl,
        source: "youtube"
      };
    } catch (error) {
      console.error("YouTube extraction failed:", error);
      return null;
    }
  }

  private extractVideoId(url: string): string | null {
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

    return null;
  }
}
