export const postMessage = async (
  from: string,
  to: string,
  content: string,
) => {
  const response = await fetch(`${import.meta.env.VITE_API_URL}/api/events`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      type: "message",
      time: new Date().toISOString(),
      from,
      message: { to, content },
    }),
  });
  if (response.ok) {
    return Promise.resolve();
  } else {
    throw new Error("Failed to post message");
  }
};
