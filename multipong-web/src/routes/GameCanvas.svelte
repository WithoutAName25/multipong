<script lang="ts">
  import type { GameConfiguration } from "$lib/GameConfiguration"

  export let gameConfig: GameConfiguration
  let canvas: HTMLCanvasElement
</script>

<canvas bind:this={canvas} />
