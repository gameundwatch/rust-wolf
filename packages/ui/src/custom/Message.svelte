<script lang="ts">
  export type MessageProps = {
    to: string;
    content: string;
  };

  let {
    time,
    from,
    message,
  }: {
    time: Date;
    from: string;
    message: MessageProps;
  } = $props();
</script>

<tr class="grid grid-cols-[1fr_3fr] px-4 py-1">
  <td class="font-bold">{from}</td>
  <td>{message.content}</td>
</tr>
