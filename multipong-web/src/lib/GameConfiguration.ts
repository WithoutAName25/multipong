import { writable } from "svelte/store"

export class GameConfiguration {
  readonly side = writable<Side>(Side.AUTO)
}

export enum Side {
  AUTO = "auto",
  ALWAYS_LEFT = "left",
  ALWAYS_RIGHT = "right",
}
