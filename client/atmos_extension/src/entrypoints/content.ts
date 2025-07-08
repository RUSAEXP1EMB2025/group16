import { detectSiteType, isTargetSite } from "../lib/utils/site-detector";
import { YouTubeExtractor } from "../lib/extractors/youtube";
import { NetflixExtractor } from "../lib/extractors/netflix";
import { GenericExtractor } from "../lib/extractors/generic";
import type { SiteDataRequest, SiteExtractor } from "../lib/types/content";
import { atmosApi } from "@/api/client";
import { tokenManager } from "@/lib/stores/token";

export default defineContentScript({
  matches: ["*://*.youtube.com/*", "*://*.netflix.com/*", "*://*/*"],
  main() {
    console.log("Atmos content script loaded");

    if (!isTargetSite(window.location.href)) {
      return;
    }

    const siteType = detectSiteType(window.location.href);
    let extractor: SiteExtractor;

    const extractors = {
      youtube: YouTubeExtractor,
      netflix: NetflixExtractor,
      generic: GenericExtractor
    };

    const ExtractorClass = extractors[siteType];

    if (ExtractorClass) {
      extractor = new ExtractorClass();
    } else {
      console.log("Unknown site type");
      return;
    }

    performExtraction(extractor);

    // URLの変更を検知 (SPA サイト対応)
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

async function performExtraction<T extends SiteExtractor>(extractor: T) {
  try {
    const content = await extractor.extract();

    if (content) {
      console.log("Extracted content:", content);

      const remoToken = await tokenManager.getToken();
      if (remoToken) {
        try {
          await atmosApi.adjust_lighting({ remo_token: remoToken, site_data: content });
        } catch (err) {
          console.error(err);
        }

        logExtractedContent(content);
      }
    } else {
      console.log("No content extracted");
    }
  } catch (error) {
    console.error("Content extraction failed:", error);
  }
}

function logExtractedContent(content: SiteDataRequest) {
  console.log(`[${content.source}] ${content.type}:`, content.data);
}
