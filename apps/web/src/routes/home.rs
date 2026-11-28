use axum::{extract::State, response::Html};

use crate::config::AppConfig;

pub async fn handler(State(config): State<AppConfig>) -> Html<String> {
    let version = config.version;
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>DevOps RPi — Rust Service 5</title>
  <style>
    body {{ font-family: system-ui, sans-serif; background: #0f172a; color: #e2e8f0; margin: 2rem; }}
    main {{ max-width: 680px; margin: 0 auto; background: #1e293b; padding: 1.5rem 2rem; border-radius: 12px; }}
    code {{ background: #0f172a; padding: 0.2rem 0.45rem; border-radius: 6px; }}
    a {{ color: #38bdf8; margin-right: 1rem; }}
    pre {{ background: #0f172a; padding: 1rem; border-radius: 8px; overflow-x: auto; }}
  </style>
</head>
<body>
  <main>
    <h1>DevOps RPi — Rust (Axum) Service</h1>
    <p>Deployed version: <code>{version}</code></p>
    <p>
      <a href="./health">Health JSON</a>
      <a href="./api/info">Info JSON</a>
      <a href="/bookmarks/">Bookmarks</a>
      <a href="/files/">Files</a>
      <a href="/monitoring/">Monitoring</a>
    </p>
    <h3>Live Container Info</h3>
    <pre id="info">Loading...</pre>
  </main>
  <script>
    fetch("./api/info")
      .then(r => r.json())
      .then(d => document.getElementById("info").textContent = JSON.stringify(d, null, 2))
      .catch(err => document.getElementById("info").textContent = "Error: " + err);
  </script>
</body>
</html>"#
    ))
}
