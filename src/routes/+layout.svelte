<script lang="ts">
  import { setContext } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { message } from '@tauri-apps/api/dialog';
  import { listen } from '@tauri-apps/api/event';
  import Sidebar from '$lib/sidebar/Sidebar.svelte';
  import { writable } from 'svelte/store';

  async function getReqs(): Promise<Record<string, Request>> {
    try {
      return await invoke('get_requests');
    } catch (e: any) {
      console.error(e);
      console.log(e);
      await message(e, { title: 'Error', type: 'error' });
      return {};
    }
  }

  let reqs = writable<Record<string, Request>>();
  setContext('requests', reqs);
  getReqs().then((res) => reqs.set(res));

  const ev = listen('update-requests', (event) => {
    getReqs().then((res) => reqs.set(res));
  });
</script>

<div class="flex">
  <div class="max-w-screen-md w-1/3">
    <Sidebar />
  </div>
  <slot />
</div>
