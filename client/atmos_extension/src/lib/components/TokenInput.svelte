<script lang="ts">
  import { tokenManager } from "../stores/token";
  import { onMount } from "svelte";

  let token = "";
  let isLoading = false;
  let message = "";
  let messageType: "success" | "error" | "info" = "info";
  let hasExistingToken = false;
  let showToken = false;

  onMount(async () => {
    await loadExistingToken();
  });

  async function loadExistingToken() {
    try {
      const existingToken = await tokenManager.getToken();
      if (existingToken) {
        hasExistingToken = true;
        token = existingToken;
      }
    } catch (error) {
      console.error("Failed to load existing token:", error);
    }
  }

  async function saveToken() {
    if (!token.trim()) {
      showMessage("トークンを入力してください", "error");
      return;
    }

    if (!tokenManager.validateTokenFormat(token.trim())) {
      showMessage("無効なトークン形式です", "error");
      return;
    }

    isLoading = true;
    try {
      await tokenManager.setToken(token.trim());
      hasExistingToken = true;
      showMessage("トークンが正常に保存されました", "success");
    } catch (error) {
      console.error("Failed to save token:", error);
      showMessage(`トークンの保存に失敗しました: ${error}`, "error");
    } finally {
      isLoading = false;
    }
  }

  async function clearToken() {
    isLoading = true;
    try {
      await tokenManager.clearToken();
      token = "";
      hasExistingToken = false;
      showMessage("トークンが削除されました", "info");
    } catch (error) {
      console.error("Failed to clear token:", error);
      showMessage("トークンの削除に失敗しました", "error");
    } finally {
      isLoading = false;
    }
  }

  function showMessage(text: string, type: "success" | "error" | "info") {
    message = text;
    messageType = type;
    setTimeout(() => {
      message = "";
    }, 3000);
  }

  function toggleTokenVisibility() {
    showToken = !showToken;
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === "Enter") {
      saveToken();
    }
  }
</script>

<div class="token-input-container">
  <div class="header">
    <p class="description">
      Nature Remo
      APIトークンを入力して、スマートライト制御を有効にしてください。
    </p>
  </div>

  <div class="input-group">
    <label for="token-input">APIトークン:</label>
    <div class="input-wrapper">
      <input
        id="token-input"
        type={showToken ? "text" : "password"}
        bind:value={token}
        on:keypress={handleKeyPress}
        placeholder="Nature Remo APIトークンを入力..."
        disabled={isLoading}
        class="token-input"
      />
      <button
        type="button"
        on:click={toggleTokenVisibility}
        class="visibility-toggle"
        disabled={isLoading}
        title={showToken ? "非表示" : "表示"}
      >
        {showToken ? "x" : "o"}
      </button>
    </div>
  </div>

  {#if message}
    <div class="message {messageType}">
      {message}
    </div>
  {/if}

  <div class="button-group">
    <button
      on:click={saveToken}
      disabled={isLoading || !token.trim()}
      class="save-button"
    >
      {isLoading ? "保存中..." : hasExistingToken ? "更新" : "保存"}
    </button>

    {#if hasExistingToken}
      <button on:click={clearToken} disabled={isLoading} class="clear-button">
        削除
      </button>
    {/if}
  </div>

  <div class="help-text">
    <p>
      <strong>トークンの取得方法:</strong><br />
      1.
      <a href="https://home.nature.global/" target="_blank" rel="noopener"
        >Nature Remo</a
      >
      にログイン<br />
      2. 設定 → API → 新しいトークンを作成<br />
      3. 生成されたトークンをここに入力
    </p>
  </div>
</div>

<style>
  .token-input-container {
    width: 100%;
    max-width: 400px;
    margin: 0 auto;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
  }

  .header {
    text-align: center;
    margin-bottom: 24px;
  }

  .header h2 {
    margin: 0 0 8px 0;
    color: #333;
    font-size: 24px;
  }

  .description {
    margin: 0;
    color: #666;
    font-size: 14px;
    line-height: 1.4;
    margin: 2em 1em;
  }

  .input-group {
    margin-bottom: 16px;
    margin: 1em;
  }

  .input-group label {
    display: block;
    margin-bottom: 8px;
    font-weight: 500;
    color: #333;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .token-input {
    flex: 1;
    padding: 12px 40px 12px 12px;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 14px;
    font-family: monospace;
    transition: border-color 0.2s;
  }

  .token-input:focus {
    outline: none;
    border-color: #007bff;
  }

  .token-input:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  .visibility-toggle {
    position: absolute;
    right: 8px;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    padding: 4px;
    border-radius: 4px;
    transition: background-color 0.2s;
  }

  .visibility-toggle:hover {
    background-color: #f0f0f0;
  }

  .visibility-toggle:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .message {
    padding: 12px;
    border-radius: 6px;
    margin-bottom: 16px;
    font-size: 14px;
    font-weight: 500;
  }

  .message.success {
    color: #155724;
  }

  .message.error {
    color: #721c24;
  }

  .message.info {
    color: #0c5460;
  }

  .button-group {
    display: flex;
    flex-direction: row;
    justify-content: center;
    gap: 1em;
  }

  .save-button {
    padding: 12px 24px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    cursor: pointer;
    height: 3em;
    transition: background-color 0.2s;
  }

  .save-button:hover:not(:disabled) {
    background-color: #0056b3;
  }

  .save-button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }

  .clear-button {
    padding: 12px 24px;
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    height: 3em;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .clear-button:hover:not(:disabled) {
    background-color: #c82333;
  }

  .clear-button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }

  .help-text {
    padding: 16px;
    border-radius: 6px;
    font-size: 13px;
    line-height: 1.4;
  }

  .help-text p {
    margin: 0;
    color: #555;
  }

  .help-text a {
    color: #007bff;
    text-decoration: none;
  }

  .help-text a:hover {
    text-decoration: underline;
  }
</style>
