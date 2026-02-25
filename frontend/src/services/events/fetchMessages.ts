export const fetchMessages = async () => {
  const response = await fetch(`${import.meta.env.VITE_API_URL}/api/events`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  const data = await response.json();
  return data;
};
