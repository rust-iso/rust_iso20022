import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { spawn } from "node:child_process";

const packageDirectory = path.resolve(process.argv[2]);
const chrome = process.env.CHROME_BIN ??
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const namespace = "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08";
const html = `<!doctype html><body data-status="starting"><script type="module">
import init, { detect_message, catalogue_message } from "/rust_iso20022_wasm.js";
try {
  await init();
  const detected = detect_message('<Document xmlns="${namespace}"/>');
  const catalogue = catalogue_message(detected.message_id);
  document.body.dataset.status = "passed";
  document.body.dataset.messageId = detected.message_id;
  document.body.dataset.schemaHashLength = String(catalogue.schema_sha256.length);
} catch (error) {
  document.body.dataset.status = "failed";
  document.body.dataset.errorCode = error?.code ?? "unknown";
}
</script></body>`;

const server = http.createServer((request, response) => {
  if (request.url === "/" || request.url === "/index.html") {
    response.writeHead(200, { "content-type": "text/html" });
    response.end(html);
    return;
  }
  const name = path.basename(request.url);
  const file = path.join(packageDirectory, name);
  if (!fs.existsSync(file)) {
    response.writeHead(404).end();
    return;
  }
  const type = name.endsWith(".wasm") ? "application/wasm" : "text/javascript";
  response.writeHead(200, { "content-type": type });
  fs.createReadStream(file).pipe(response);
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const { port } = server.address();
const child = spawn(chrome, [
  "--headless=new",
  "--disable-gpu",
  "--no-sandbox",
  "--virtual-time-budget=10000",
  "--dump-dom",
  `http://127.0.0.1:${port}/`,
]);
let stdout = "";
let stderr = "";
child.stdout.on("data", (chunk) => { stdout += chunk; });
child.stderr.on("data", (chunk) => { stderr += chunk; });
const status = await new Promise((resolve) => child.on("close", resolve));
server.close();

if (status !== 0 || !stdout.includes('data-status="passed"')) {
  throw new Error(`browser runtime failed (${status}): ${stdout}\n${stderr}`);
}
if (!stdout.includes('data-message-id="pacs.008.001.08"')) {
  throw new Error(`browser detection mismatch: ${stdout}`);
}
if (!stdout.includes('data-schema-hash-length="64"')) {
  throw new Error(`browser catalogue provenance mismatch: ${stdout}`);
}
console.log("verified headless Chrome runtime parity");
