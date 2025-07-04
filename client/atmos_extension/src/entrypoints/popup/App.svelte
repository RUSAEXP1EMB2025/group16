<script lang="ts">
  import TokenInput from "../../lib/components/TokenInput.svelte";
</script>

<main>
  <div class="header">
    <h1>Atmos</h1>
    <p class="subtitle">インテリジェント照明制御システム</p>
  </div>

  <div class="content">
    <TokenInput />
  </div>
</main>

<style>
  main {
    width: 100%;
    max-width: 450px;
    margin: 0 auto;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
    background-color: #ffffff;
    min-height: 500px;
  }

  .header {
    text-align: center;
    padding: 24px 20px 20px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border-radius: 0 0 12px 12px;
    margin-bottom: 0;
  }

  .header h1 {
    margin: 0 0 8px 0;
    font-size: 28px;
    font-weight: 700;
    letter-spacing: -0.5px;
  }

  .subtitle {
    margin: 0;
    font-size: 14px;
    opacity: 0.9;
    font-weight: 400;
  }

  .content {
    padding: 0;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #f5f5f5;
  }
</style>
