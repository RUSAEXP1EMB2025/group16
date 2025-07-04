import type { ExtractedContent, SiteExtractor } from "../types/content";
import { waitForElement, extractTextFromElement } from "../utils/dom";

export class NetflixExtractor implements SiteExtractor {
  canExtract(url: string): boolean {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.includes("netflix.com");
  }

  async extract(): Promise<ExtractedContent | null> {
    try {
      if (!this.isWatchPage()) {
        return null;
      }

      const title = await this.extractTitle();
      if (!title) {
        return null;
      }

      return {
        type: "title",
        data: title,
        source: "netflix"
      };
    } catch (error) {
      console.error("Netflix extraction failed:", error);
      return null;
    }
  }

  private isWatchPage(): boolean {
    return window.location.pathname.includes("/watch/");
  }

  private async extractTitle(): Promise<string | null> {
    const titleSelectors = [
      '[data-uia="video-title"]',
      ".video-title",
      ".title-card-title",
      ".fallback-text",
      'h1[data-uia="title-card-title"]',
      ".previewModal--title",
      '[data-uia="title-card-title-text"]'
    ];

    for (const selector of titleSelectors) {
      const element = await waitForElement(selector, 3000);
      if (element) {
        const title = extractTextFromElement(element);
        if (title.length > 0) {
          return title;
        }
      }
    }

    const docTitle = document.title;
    if (docTitle && docTitle !== "Netflix") {
      const titleMatch = docTitle.match(/^(.+?)\s*-\s*Netflix$/);
      if (titleMatch) {
        return titleMatch[1].trim();
      }
    }

    return null;
  }
}
