import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const require = createRequire(import.meta.url);
const wasm = require(path.resolve(process.argv[2]));
const valid = fs.readFileSync(
  path.join(repository, "fixtures/iso/valid/pacs.008.001.08-cross-field.xml"),
  "utf8",
);
const invalid = fs.readFileSync(
  path.join(repository, "fixtures/iso/invalid/pacs.008.001.08-cross-field.xml"),
  "utf8",
);

for (const name of [
  "catalogue_message",
  "detect_message",
  "parse_message",
  "serialize_message",
  "validate_message",
]) {
  if (typeof wasm[name] !== "function") throw new Error(`missing export: ${name}`);
}

const detected = wasm.detect_message(valid);
if (detected.message_id !== "pacs.008.001.08") throw new Error("detect mismatch");
const catalogue = wasm.catalogue_message(detected.message_id);
if (catalogue.schema_sha256.length !== 64) throw new Error("catalogue provenance missing");
const parsed = wasm.parse_message(valid);
const serialized = wasm.serialize_message(parsed.message_id, parsed.document);
if (!serialized.xml.includes("WP008-VALID")) throw new Error("generated round trip mismatch");
const report = wasm.validate_message(invalid, "l2");
if (report.valid || report.errors.length !== 6) throw new Error("validation mismatch");

try {
  wasm.validate_message(valid, "l1");
  throw new Error("unavailable validation layer unexpectedly succeeded");
} catch (error) {
  if (error.code !== "validation_unavailable") {
    throw new Error("unavailable validation layer was not structured");
  }
}

const canary = "SECRET-ACCOUNT-DE89370400440532013000";
for (const input of [
  `<Document>${canary}`,
  `<Document>${canary}${"x".repeat(16 * 1024 * 1024 + 1)}</Document>`,
]) {
  try {
    wasm.detect_message(input);
    throw new Error("invalid input unexpectedly succeeded");
  } catch (error) {
    if (JSON.stringify(error).includes(canary)) throw new Error("sensitive input leaked");
    if (error.code !== "invalid_input") throw new Error("unstructured bounded-input error");
  }
}

try {
  wasm.parse_message(
    '<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.09"/>',
  );
  throw new Error("unsupported model unexpectedly succeeded");
} catch (error) {
  if (error.code !== "unsupported_feature" || error.required_feature !== "model-camt") {
    throw new Error("unsupported feature preflight mismatch");
  }
}

console.log("verified structured WASM parity, limits, redaction, and feature preflight");
