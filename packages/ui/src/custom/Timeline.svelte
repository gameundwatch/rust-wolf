<script lang="ts">
  import Message from "./Message.svelte";
  import type { Event } from "@rust-wolf/bindings/Event";
  let { events }: { events: Event[] } = $props();
</script>

<table class="w-full">
  <tbody>
    {#each events as event}
      <Message time={event.time} from={event.from} message={event.message} />
    {/each}
  </tbody>
</table>
