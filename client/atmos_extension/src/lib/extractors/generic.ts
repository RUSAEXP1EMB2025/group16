import type { SiteDataRequest, SiteExtractor } from "../types/content";
import { getAllTextNodes } from "../utils/dom";
import { atmosApi } from "../../api/client";

export class GenericExtractor implements SiteExtractor {
  private dictionary: string[] = [];

  canExtract(url: string): boolean {
    const hostname = new URL(url).hostname.toLowerCase();
    return (
      !hostname.includes("youtube.com") &&
      !hostname.includes("youtu.be") &&
      !hostname.includes("netflix.com")
    );
  }

  async extract(): Promise<SiteDataRequest | undefined> {
    try {
      if (this.dictionary.length === 0) {
        await this.loadDictionary();
      }

      const keywords = this.searchDictionaryInPage();

      if (keywords.length === 0) {
        return;
      }

      return {
        Generic: {
          keywords
        }
      };
    } catch (error) {
      console.error("generic extraction failed:", error);
      return;
    }
  }

  private searchDictionaryInPage(): string[] {
    const foundWords: string[] = [];

    const textNodes = getAllTextNodes(document.body);
    const pageText = textNodes.join(" ").toLowerCase();

    for (const word of this.dictionary) {
      if (pageText.includes(word.toLowerCase())) {
        foundWords.push(word);
      }
    }

    return Array.from(new Set(foundWords));
  }

  setDictionary(dictionary: string[]): void {
    this.dictionary = dictionary;
  }

  async loadDictionary(): Promise<void> {
    try {
      const response = await atmosApi.get_atmoswords();
      const atmoswords = Array.from(response.atmoswords);
      this.setDictionary(atmoswords);
      console.log(`Dictionary loaded: ${atmoswords.length} words`);
    } catch (error) {
      console.error("Failed to load dictionary:", error);
      throw error;
    }
  }
}
