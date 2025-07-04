export interface ExtractedContent {
  type: 'url' | 'title' | 'dictionary-words';
  data: string | string[];
  source: 'youtube' | 'netflix' | 'generic';
}

export interface SiteExtractor {
  canExtract(url: string): boolean;
  extract(): Promise<ExtractedContent | null>;
}

export type SiteType = 'youtube' | 'netflix' | 'generic';