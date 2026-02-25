<script lang="ts">
  import Message from "./Message.svelte";

  type Event = {
    type: string;
    time: Date;
    from: string;
    message?: MessageProps;
  };

  let { events } = $props();
</script>

<table class="w-full">
  <tbody>
    {#each events as event}
      <Message time={event.time} from={event.from} message={event.message} />
    {/each}
  </tbody>
</table>
