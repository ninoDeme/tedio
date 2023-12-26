<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { message } from '@tauri-apps/api/dialog';

  let name = '';
  let url = '';
  let greetMsg = '';

  async function setReq() {
    try {
      console.log(`Added: ${await invoke('set_request', { name, url })}`);
    } catch (e: any) {
      console.error(e);
      console.log(e);
      await message(e, { title: 'Error', type: 'error' });
    }
  }
</script>

<form on:submit={setReq} class="flex flex-col items-start">
  <label class="mt-2" for="name-input">Name</label>
  <input id="name-input" bind:value={name} />
  <label class="mt-2" for="url-input">URL</label>
  <input id="url-input" bind:value={url} />

  <button>Add Request</button>
</form>

{#if !!greetMsg}
  <h1>{greetMsg}</h1>
{/if}

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
