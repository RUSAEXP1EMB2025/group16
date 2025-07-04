import { detectSiteType, isTargetSite } from "../lib/utils/site-detector";
import { YouTubeExtractor } from "../lib/extractors/youtube";
import { NetflixExtractor } from "../lib/extractors/netflix";
import { GenericExtractor } from "../lib/extractors/generic";
import type { ExtractedContent } from "../lib/types/content";

export default defineContentScript({
  matches: ["*://*.youtube.com/*", "*://*.netflix.com/*", "*://*/*"],
  main() {
    console.log("Atmos content script loaded");

    if (!isTargetSite(window.location.href)) {
      return;
    }

    const siteType = detectSiteType(window.location.href);
    let extractor;

    switch (siteType) {
      case "youtube":
        extractor = new YouTubeExtractor();
        break;
      case "netflix":
        extractor = new NetflixExtractor();
        break;
      case "generic":
        extractor = new GenericExtractor();
        break;
      default:
        console.log("Unknown site type");
        return;
    }

    // Perform initial extraction
    performExtraction(extractor);

    // Monitor URL changes (SPA site support)
    let currentUrl = window.location.href;
    const observer = new MutationObserver(() => {
      if (window.location.href !== currentUrl) {
        currentUrl = window.location.href;
        console.log("URL changed, re-extracting content");
        performExtraction(extractor);
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true
    });
  }
});

async function performExtraction(extractor: any) {
  try {
    const content: ExtractedContent | null = await extractor.extract();

    if (content) {
      console.log("Extracted content:", content);

      // TODO: バックエンドに送信
      // await sendToBackend(content);

      // 現在はコンソールに出力
      logExtractedContent(content);
    } else {
      console.log("No content extracted");
    }
  } catch (error) {
    console.error("Content extraction failed:", error);
  }
}

function logExtractedContent(content: ExtractedContent) {
  console.log(`[${content.source}] ${content.type}:`, content.data);
}

// TODO: バックエンドとの通信実装
// async function sendToBackend(content: ExtractedContent) {
//   // API呼び出し実装
// }
