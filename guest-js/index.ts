import { invoke } from '@tauri-apps/api/core'

export async function connect() {
  await invoke('plugin:oblivion|connect')
}
