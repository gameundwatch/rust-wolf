import type { Meta, StoryObj } from "@storybook/svelte";
import { Message } from "../src/index.js";

const meta = {
  title: "Custom/Message",
  component: Message,
  tags: ["autodocs"],
  argTypes: {
    from: { control: "text" },
    time: { control: "date" },
  },
} satisfies Meta<typeof Message>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    from: "プレイヤー1",
    time: new Date(),
    message: {
      to: "all",
      content: "おはようございます。誰が人狼だと思いますか？",
    },
  },
};

export const DirectMessage: Story = {
  args: {
    from: "プレイヤー2",
    time: new Date(),
    message: {
      to: "プレイヤー1",
      content: "あの人が怪しいと思う...",
    },
  },
};

export const LongMessage: Story = {
  args: {
    from: "プレイヤー3",
    time: new Date(),
    message: {
      to: "all",
      content:
        "昨日の投票結果を見ると、プレイヤー4とプレイヤー5の動きが不自然でした。特にプレイヤー4は占い師を名乗っているにもかかわらず、結果の報告が遅れていた点が気になります。",
    },
  },
};
