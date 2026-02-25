<script lang="ts">
  import { Command, Timeline } from "@rust-wolf/ui";
  import { onMount } from "svelte";
  import { postMessage } from "./services/events/postMessage";
  import { fetchMessages } from "./services/events/fetchMessages";

  let events: Event[] = $state.raw([]);

  onMount(async () => {
    events = await fetchMessages();
  });
</script>

<div id="wrapper" class="w-full h-dvh bg-primary/5">
  <main
    class="grid grid-rows-[1fr_auto] w-full max-w-5xl h-full bg-primary/10 mx-auto"
  >
    <Timeline class="overflow-y-auto" {events} />
    <Command
      onsubmit={async (content) => {
        console.log(content);
        await postMessage("player1", "all", content);
        events = await fetchMessages();
      }}
    />
  </main>
</div>
