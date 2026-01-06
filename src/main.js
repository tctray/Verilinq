const { invoke } = window.__TAURI__.core;

const form = document.querySelector("#url-form");
const input = document.querySelector("#url-input");
const output = document.querySelector("#output");

form.addEventListener("submit", async (e) => {
  e.preventDefault();

  const url = input.value.trim();
  if (!url) {
    output.textContent = "Please enter a URL.";
    return;
  }

  output.innerHTML = "<div class='loading'>🕷️ Spider is crawling the page...</div>";

  try {
    const results = await invoke("check_page_links", { url });
    displayResults(results);
  } catch (err) {
    output.innerHTML = `<div class='error'>Error: ${err}</div>`;
  }
});

function displayResults(results) {
  output.innerHTML = `<h3>Found ${results.length} links:</h3>`;

  if (!results.length) {
    output.innerHTML += "No links found.";
    return;
  }

  const list = document.createElement("div");
  list.className = "results-list";

  results.forEach((r) => {
    const isOk = r.status === "200";
    const item = document.createElement("div");
    item.className = isOk ? "result-item ok" : "result-item broken";

    item.innerHTML = `
      <span class="url-text">${r.url}</span>
      <span class="status-badge">${isOk ? "✅ " + r.status : "❌ " + r.status}</span>
    `;

    list.appendChild(item);
  });

  output.appendChild(list);
}
