import { invoke } from "@tauri-apps/api";

/// <reference lib="webworker" />

addEventListener("message", () => {
  console.log("asdadsuper");
  invoke("save_paths_from", { directory: "/" });
});
