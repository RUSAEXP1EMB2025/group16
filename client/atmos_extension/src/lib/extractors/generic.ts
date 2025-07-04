import type { ExtractedContent, SiteExtractor } from "../types/content";
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

  async extract(): Promise<ExtractedContent | null> {
    try {
      // TODO: バックエンドから辞書を取得
      // 現在はプレースホルダーとして空の配列を使用
      if (this.dictionary.length === 0) {
        console.log("Dictionary not loaded yet");
        return null;
      }

      const foundWords = this.searchDictionaryInPage();

      if (foundWords.length === 0) {
        return null;
      }

      return {
        type: "dictionary-words",
        data: foundWords,
        source: "generic"
      };
    } catch (error) {
      console.error("Generic extraction failed:", error);
      return null;
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
