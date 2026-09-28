import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const inventory = JSON.parse(
  fs.readFileSync(path.join(repository, "tests/compatibility/wasm-exports.json"), "utf8"),
);
const mode = process.argv[2];
const target = path.resolve(process.argv[3]);

function fail(message) {
  console.error(`error: ${message}`);
  process.exit(1);
}

if (mode === "declarations") {
  const declarations = fs.readFileSync(target, "utf8");
  for (const name of inventory.exports) {
    const pattern = new RegExp(`export function ${name}\\s*\\(`);
    if (!pattern.test(declarations)) {
      fail(`missing WASM declaration export: ${name}`);
    }
  }
  console.log(`verified ${inventory.exports.length} WASM declaration exports`);
} else if (mode === "module") {
  const require = createRequire(import.meta.url);
  const wasm = require(target);
  for (const name of inventory.exports) {
    if (typeof wasm[name] !== "function") {
      fail(`missing WASM runtime export: ${name}`);
    }
  }
  const sample = inventory.smoke;
  const xml = `<Document xmlns="${sample.namespace}"/>`;
  if (wasm.detect(xml) !== sample.message_id) fail("detect smoke mismatch");
  if (wasm.from_namespace(sample.namespace) !== sample.message_id) {
    fail("from_namespace smoke mismatch");
  }
  if (!wasm.catalogue_contains(sample.message_id)) fail("catalogue smoke mismatch");
  if (wasm.business_area_of(sample.message_id) !== sample.business_area) {
    fail("business area smoke mismatch");
  }
  console.log(`verified ${inventory.exports.length} WASM runtime exports and core smoke cases`);
} else {
  fail("usage: check-wasm-compat.mjs declarations <d.ts> | module <node-js>");
}

