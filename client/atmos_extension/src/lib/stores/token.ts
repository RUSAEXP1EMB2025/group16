// Note: WXT extension environment doesn't use SvelteKit's $app/environment
// Using WXT storage API
import { storage } from "#imports";

export interface TokenStore {
  token: string | null;
  isValid: boolean;
  lastValidated: number | null;
}

class TokenManager {
  private static readonly STORAGE_KEY = "local:atmos_nature_remo_token";

  async getToken(): Promise<string | null> {
    try {
      return await storage.getItem(TokenManager.STORAGE_KEY);
    } catch (error) {
      console.error("Failed to get token:", error);
      return null;
    }
  }

  async setToken(token: string): Promise<void> {
    try {
      await storage.setItem(TokenManager.STORAGE_KEY, token);
    } catch (error) {
      console.error("Failed to set token:", error);
      throw error;
    }
  }

  async clearToken(): Promise<void> {
    try {
      await storage.removeItem(TokenManager.STORAGE_KEY);
    } catch (error) {
      console.error("Failed to clear token:", error);
      throw error;
    }
  }

  async hasToken(): Promise<boolean> {
    const token = await this.getToken();
    return token !== null && token.length > 0;
  }

  validateTokenFormat(token: string): boolean {
    // Nature Remo APIトークンの基本的な形式チェック
    // 実際のトークン形式に応じて調整が必要
    return token.length > 10 && /^[a-zA-Z0-9._-]+$/.test(token);
  }
}

export const tokenManager = new TokenManager();
