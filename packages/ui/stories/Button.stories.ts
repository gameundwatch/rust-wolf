import type { Meta, StoryObj } from "@storybook/svelte";
import { Button } from "../src/index.js";

const meta = {
  title: "UI/Button",
  component: Button,
  tags: ["autodocs"],
  argTypes: {
    variant: {
      control: "select",
      options: [
        "default",
        "secondary",
        "destructive",
        "outline",
        "ghost",
        "link",
      ],
    },
    size: {
      control: "select",
      options: ["default", "sm", "lg", "icon"],
    },
    disabled: {
      control: "boolean",
    },
  },
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: createTextSnippet("Button"),
    variant: "default",
  },
};

export const Secondary: Story = {
  args: {
    children: createTextSnippet("Secondary"),
    variant: "secondary",
  },
};

export const Destructive: Story = {
  args: {
    children: createTextSnippet("Destructive"),
    variant: "destructive",
  },
};

export const Outline: Story = {
  args: {
    children: createTextSnippet("Outline"),
    variant: "outline",
  },
};

export const Ghost: Story = {
  args: {
    children: createTextSnippet("Ghost"),
    variant: "ghost",
  },
};

export const Link: Story = {
  args: {
    children: createTextSnippet("Link"),
    variant: "link",
  },
};

export const Small: Story = {
  args: {
    children: createTextSnippet("Small"),
    size: "sm",
  },
};

export const Large: Story = {
  args: {
    children: createTextSnippet("Large"),
    size: "lg",
  },
};

export const Disabled: Story = {
  args: {
    children: createTextSnippet("Disabled"),
    disabled: true,
  },
};

function createTextSnippet(text: string) {
  return ((anchor: Node) => {
    const node = document.createTextNode(text);
    anchor.parentNode?.insertBefore(node, anchor);
    return {
      [Symbol.dispose]() {
        node.remove();
      },
    };
  }) as any;
}
