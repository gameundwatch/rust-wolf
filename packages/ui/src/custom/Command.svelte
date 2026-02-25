<script lang="ts">
  import { Button } from "../components/ui/button/index.js";
  import { Input } from "../components/ui/input/index.js";

  let {
    placeholder = "メッセージを入力...",
    buttonLabel = "送信",
    disabled = false,
    onsubmit,
  }: {
    placeholder?: string;
    buttonLabel?: string;
    disabled?: boolean;
    onsubmit?: (value: string) => void;
  } = $props();

  let value = $state("");

  function handleSubmit() {
    const trimmed = value.trim();
    if (!trimmed) return;
    onsubmit?.(trimmed);
    value = "";
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  }
</script>

<form
  class="flex w-full p-4 gap-4 bg-primary/50"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <Input
    type="text"
    {placeholder}
    {disabled}
    bind:value
    onkeydown={handleKeydown}
    class="flex-1"
  />
  <Button type="submit" variant="default" {disabled}>
    {buttonLabel}
  </Button>
</form>
