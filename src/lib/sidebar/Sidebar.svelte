<script lang="ts">
  import type { TRequest } from '$lib';
  import { getContext } from 'svelte';
  import type {Writable} from 'svelte/store';

  let reqs = getContext<Writable<Record<string, TRequest>>>('requests');
</script>

<div class="flex flex-col">
  {#if $reqs}
    {#each Object.values($reqs) as req}
      <div class="hover:bg-slate-100">
          {req.name}
      </div>
    {/each}
  {/if}
</div>
