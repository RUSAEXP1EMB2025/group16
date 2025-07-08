import { schemas } from "@/api/apiClient";
import z from "zod";

export type SiteDataRequest = z.infer<typeof schemas.SiteDataRequest>;

export interface SiteExtractor {
  canExtract(url: string): boolean;
  extract(): Promise<SiteDataRequest | undefined>;
}

export type SiteType = "youtube" | "netflix" | "generic";
