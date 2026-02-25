import type { Meta, StoryObj } from "@storybook/svelte";
import { Command } from "../src/index.js";

const meta = {
  title: "Custom/Command",
  component: Command,
  tags: ["autodocs"],
  argTypes: {
    placeholder: { control: "text" },
    buttonLabel: { control: "text" },
    disabled: { control: "boolean" },
  },
} satisfies Meta<typeof Command>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};

export const CustomPlaceholder: Story = {
  args: {
    placeholder: "発言を入力してください...",
    buttonLabel: "発言",
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
};
