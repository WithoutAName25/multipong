<script lang="ts">
  import GameMenu from "./GameMenu.svelte"
  import GameCanvas from "./GameCanvas.svelte"
  import { GameConfiguration } from "$lib/GameConfiguration"

  let playing = false
  let gameConfig = new GameConfiguration()
</script>

{#if playing}
  <GameCanvas {gameConfig} />
{:else}
  <GameMenu {gameConfig} />
{/if}
