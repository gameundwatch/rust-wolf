import type { Meta, StoryObj } from "@storybook/svelte";
import { Timeline } from "../src/index.js";

const meta = {
  title: "Custom/Timeline",
  component: Timeline,
  tags: ["autodocs"],
} satisfies Meta<typeof Timeline>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
