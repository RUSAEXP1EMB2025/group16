import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";

const GetLightingSignalsHttpRequestBody = z.object({ remo_token: z.string() }).passthrough();
const AdjustLightingHttpRequestBody = z
  .object({
    remo_token: z.string(),
    texts: z.array(z.string()),
    url: z.string()
  })
  .passthrough();

export const schemas = {
  GetLightingSignalsHttpRequestBody,
  AdjustLightingHttpRequestBody
};

const endpoints = makeApi([
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
        schema: z.object({ remo_token: z.string() }).passthrough()
      }
    ],
    response: z.void()
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
        schema: AdjustLightingHttpRequestBody
      }
    ],
    response: z.void()
  }
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
  return new Zodios(baseUrl, endpoints, options);
}
