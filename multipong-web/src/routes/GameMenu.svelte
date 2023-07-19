<script lang="ts">
  import type { GameConfiguration } from "$lib/GameConfiguration"
  import { Side } from "$lib/GameConfiguration"

  export let gameConfig: GameConfiguration
  let { side } = gameConfig
</script>

<select bind:value={$side}>
  <option value={Side.AUTO}>Auto</option>
  <option value={Side.ALWAYS_LEFT}>Always Left</option>
  <option value={Side.ALWAYS_RIGHT}>Always Right</option>
</select>
