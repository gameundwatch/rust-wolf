import type { Meta, StoryObj } from "@storybook/svelte";
import { Input } from "../src/index.js";

const meta = {
  title: "UI/Input",
  component: Input,
  tags: ["autodocs"],
  argTypes: {
    type: {
      control: "select",
      options: ["text", "email", "password", "number", "search"],
    },
    placeholder: {
      control: "text",
    },
    disabled: {
      control: "boolean",
    },
  },
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    type: "text",
  },
};

export const WithPlaceholder: Story = {
  args: {
    type: "text",
    placeholder: "メッセージを入力...",
  },
};

export const Email: Story = {
  args: {
    type: "email",
    placeholder: "email@example.com",
  },
};

export const Password: Story = {
  args: {
    type: "password",
    placeholder: "パスワード",
  },
};

export const Disabled: Story = {
  args: {
    type: "text",
    placeholder: "無効な入力欄",
    disabled: true,
  },
};
