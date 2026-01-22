// sw.js
self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);

  // Check if the requested file is a WASM file
  if (url.pathname.endsWith(".wasm")) {
    event.respondWith(
      fetch(event.request)
        .then((response) => {
          // Clone the response to modify its headers
          const newResponse = new Response(response.body, {
            status: response.status,
            statusText: response.statusText,
            headers: {
              "Content-Type": "application/wasm",
            },
          });
          return newResponse;
        })
        .catch((error) => {
          console.error("Error fetching WASM file:", error);
          throw error;
        }),
    );
  }
});
