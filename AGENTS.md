# AGENTS.md

## Project summary

`upf` is a Rust library for reading and writing Unified Pseudopotential Format
(UPF) documents, currently centered on UPF `2.0.1`. The crate exposes
file/string/reader entry points for deserialization and serialization, plus a
typed `model` module for structured access to UPF sections.

The main engineering goal in this repository is structural correctness:
deserialize UPF into typed Rust data, serialize it back out, and reject
documents whose required section relationships or array lengths are invalid.

## Repository map

- `src/lib.rs`: public API surface; re-exports read/write helpers, errors, text
  helpers, and the `model` module.
- `src/de.rs`: read-side entry points built on `quick_xml::de`; all successful
  parses end in `UpfData::validate()`.
- `src/ser.rs`: write-side entry points built on `quick_xml::se`; documents are
  validated before serialization/writing.
- `src/model/core.rs`: root `UpfData` type, header/mesh/core numeric arrays, and
  cross-section validation rules.
- `src/model/nonlocal.rs`: nonlocal, semilocal, pseudowavefunction, and `PP_INFO`
  structures.
- `src/model/paw.rs`: PAW-specific sections.
- `src/model/gipaw.rs`: GIPAW-specific sections.
- `src/text.rs`: helpers for parsing/formatting float lists and UPF boolean
  flags, with unit tests.
- `tests/*.rs`: integration tests using small inline UPF fixtures.
- `docs/reference/upf-spec.html`: local copy of the external UPF reference. It is not clear, you need to use it together with `docs/reference/upf.xsd` for better understanding and more exact info.

## Working conventions

- Keep parse and write paths symmetric. If a UPF field is added or changed in
  the model, update both serde mappings and tests that cover deserialization and
  serialization behavior.
- Prefer explicit model types that mirror UPF section names and attributes.
  Existing code uses `#[serde(rename = ...)]` heavily; follow that pattern
  rather than introducing custom transforms unless the format forces it.
- Put structural invariants in validation, not scattered across callers.
  `UpfData::validate()` in `src/model/core.rs` is the current central place for
  mesh-length checks and required-section checks.
- Preserve the public API shape unless the task explicitly requires expansion.
  The crate currently exposes `from_str`, `from_reader`, `from_file`,
  `to_string`, `to_writer`, `to_file`, `UpfError`, `UpfData`, and the `model`
  module.
- Treat round-tripping as semantic round-tripping. Existing tests compare parsed
  data structures after reserialization; they do not require byte-for-byte
  preservation of whitespace, comments, or original layout.
- Keep changes narrow. This crate is small and strongly model-driven, so broad
  refactors are usually unnecessary unless they directly improve the target
  behavior.

## Testing and verification

Run these commands before finishing substantial changes:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo doc --no-deps` when public API or documentation changes

When adding support for a new section or invariant, add focused tests using the
existing style: short inline XML fixtures in `tests/*.rs` plus narrow assertions
about parse results, validation failures, or semantic round-trips.

## Repo-specific notes

- `README.md` currently references `docs/PROJECT.md`, but that file is not
  present in this checkout. Do not assume it exists unless it is added.
- The crate targets Rust edition `2024`.
- Current fixtures use deterministic literal dates such as `2026-04-03`; keep
  tests stable and avoid introducing time-dependent output.
- `docs/reference/upf-spec.html` is reference material, not generated code. Do
  not edit it unless the task is specifically about updating the bundled spec
  snapshot.
