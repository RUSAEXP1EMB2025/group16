import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";

const GetLightingSignalsHttpRequestBody = z
  .object({ remo_token: z.string() })
  .passthrough();
const SiteDataRequest = z.union([
  z
    .object({ Youtube: z.object({ url: z.string() }).passthrough() })
    .passthrough(),
  z
    .object({ Netflix: z.object({ title: z.string() }).passthrough() })
    .passthrough(),
  z
    .object({
      Generic: z.object({ keywords: z.array(z.string()) }).passthrough(),
    })
    .passthrough(),
]);
const AdjustLightingHttpRequestBody = z
  .object({ remo_token: z.string(), site_data: SiteDataRequest })
  .passthrough();

export const schemas = {
  GetLightingSignalsHttpRequestBody,
  SiteDataRequest,
  AdjustLightingHttpRequestBody,
};

const endpoints = makeApi([
  {
    method: "get",
    path: "/atmoswords",
    alias: "get_atmoswords",
    description: `サイトから取得するべきキーワード辞書を取得`,
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "get",
    path: "/lighting",
    alias: "get_lighting_signals",
    description: `登録されている電気の信号を取得`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ remo_token: z.string() }).passthrough(),
      },
    ],
    response: z.void(),
  },
  {
    method: "post",
    path: "/lighting",
    alias: "adjust_lighting",
    description: `部屋の電気をサイト内容から調整`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: AdjustLightingHttpRequestBody,
      },
    ],
    response: z.void(),
  },
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
  return new Zodios(baseUrl, endpoints, options);
}
