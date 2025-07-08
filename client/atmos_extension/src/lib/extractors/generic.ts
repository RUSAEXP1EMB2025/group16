import type { SiteDataRequest, SiteExtractor } from "../types/content";
import { getAllTextNodes } from "../utils/dom";

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
      // TODO: バックエンドから辞書を取得
      // 現在はプレースホルダーとして空の配列を使用
      if (this.dictionary.length === 0) {
        console.error("Dictionary not loaded yet");
        return;
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
    // TODO: バックエンドAPIから辞書を取得
    // const response = await fetch('/api/dictionary');
    // const dictionary = await response.json();
    // this.setDictionary(dictionary);

    // 現在はプレースホルダー
    console.log("Dictionary loading not implemented yet");
  }
}
