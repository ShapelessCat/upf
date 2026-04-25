# AGENTS.md

## Project summary

`upf` is a Rust library for reading and writing Unified Pseudopotential Format
(UPF) documents, currently centered on UPF `2.0.1`. The crate exposes
file/string/reader entry points for deserialization and serialization, plus a
typed `model` module for structured access to UPF sections.

The main engineering goal is structural correctness: deserialize UPF into typed
Rust data, serialize it back out, and reject documents whose required section
relationships or array lengths are invalid.

## Repository map

- `src/lib.rs`: public API surface; re-exports read/write helpers, errors, text
  helpers, and the `model` module.
- `src/error.rs`: `UpfError` enum (wraps `quick_xml`, I/O, and validation
  errors via `thiserror`).
- `src/de.rs`: read-side entry points built on `quick_xml::de`; all successful
  parses end in `UpfData::validate()`.
- `src/ser.rs`: write-side entry points built on `quick_xml::se`; documents are
  validated before serialization/writing.
- `src/text.rs`: helpers for parsing/formatting float lists and UPF boolean
  flags, with unit tests.
- `src/model.rs`: module root; re-exports all model submodules.
- `src/model/upf_data.rs`: root `UpfData` type and cross-section validation
  rules (`validate()`).
- `src/model/header.rs`: `PpHeader` with enums (`PseudopotentialType`,
  `AtomicRelativisticFormalism`, etc.).
- `src/model/mesh.rs`: `PpMesh`, `PpR`, `PpRab`.
- `src/model/info.rs`: `PpInfo`.
- `src/model/data_section.rs`: `NumericSection`, `UpfDataType` enum.
- `src/model/numeric_text.rs`: numeric deserialization helper.
- `src/model/numbered.rs`: `NumberedTag`, `Numbered<T>`, `Tagged<T>` for
  indexed UPF elements like `PP_BETA.n`, `PP_CHI.n`.
- `src/model/nonlocal.rs`: `PpNonlocal`, `PpBeta`, `PpDij`, `PpAugmentation`.
- `src/model/semilocal.rs`: `PpSemilocal`, `PpVnl` channels.
- `src/model/pseudo_wavefunctions.rs`: `PpPswfc` with `PP_CHI.n`.
- `src/model/full_wfc.rs`: `PpFullWfc` with `PP_AEWFC`/`PP_PSWFC.n`.
- `src/model/wavefunction.rs`: `PpWavefunction` shared type (with optional
  spin-orbit attributes `nn`, `jchi` and optional `n`, `pseudo_energy`,
  `cutoff_radius`, `ultrasoft_cutoff_radius`).
- `src/model/paw.rs`: PAW-specific sections.
- `src/model/gipaw.rs`: GIPAW-specific sections.
- `src/model/metagga.rs`: metagga section support (`PP_TAUMOD`, `PP_TAUATOM`).
- `src/model/spin_orb.rs`: `PpSpinOrb`, `PpRelWfc`, `PpRelBeta`.
- `tests/*.rs`: integration test files using inline UPF fixtures.
- `examples/SSSP_1.3.0_PBE_efficiency/`: real-world UPF files (1.x, 2.x, and
  future format) from the SSSP dataset, used for fixture-based testing.
- `docs/PROJECT.md`: detailed project guide.
- `docs/compare_1.x_with_2.x.md`: UPF version comparison notes.
- `docs/reference/UPF v2.0.1/`: spec and XSD for UPF 2.0.1 (reverse-engineered
  from Quantum ESPRESSO). Use the `.md` spec together with the `.xsd` schema
  for the most exact understanding.
- `docs/reference/UPF future (after v2.0.1)/`: spec and XSD for the upcoming
  format revision.

## Key dependencies

- `quick-xml` (with `serialize` + `serde-types` features): XML
  deserialization/serialization.
- `serde` / `serde_with`: derive-based (de)serialization; heavy use of
  `#[serde(rename = ...)]` to match UPF element/attribute names.
- `validify`: validation derive macros used in model types.
- `thiserror`: error enum derivation.
- `anyhow`: used in tests and examples.

## Working conventions

- Organize type definitions from the highest-level abstraction to lower-level
  building blocks: define the main type first, then define the types that
  represent its components.
- Keep parse and write paths symmetric. If a UPF field is added or changed in
  the model, update both serde mappings and tests that cover deserialization and
  serialization behavior.
- Prefer explicit model types that mirror UPF section names and attributes.
  Existing code uses `#[serde(rename = ...)]` heavily; follow that pattern
  rather than introducing custom transforms unless the format forces it.
- Put structural invariants in validation, not scattered across callers.
  `UpfData::validate()` in `src/model/upf_data.rs` is the central place for
  mesh-length checks, required-section checks, projector counts, augmentation
  sizes, and GIPAW orbital counts.
- Preserve the public API shape unless the task explicitly requires expansion.
  The crate currently exposes `from_str`, `from_reader`, `from_file`,
  `to_string`, `to_writer`, `to_file`, `UpfError`, `UpfData`, text helpers
  (`parse_f64_vec`, `format_f64_slice`, `parse_bool_flag`, `format_bool_flag`),
  and the `model` module.
- Treat round-tripping as semantic round-tripping. Existing tests compare parsed
  data structures after reserialization; they do not require byte-for-byte
  preservation of whitespace, comments, or original layout.
- Keep changes narrow. This crate is small and strongly model-driven, so broad
  refactors are usually unnecessary unless they directly improve the target
  behavior.

## Configuration

- Rust edition `2024`.

## Testing and verification

Run these commands before finishing substantial changes:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo doc --no-deps` when public API or documentation changes

CI (`.github/workflows/ci.yml`) runs all four checks on every push and PR.

When adding support for a new section or invariant, add focused tests using the
existing style: short inline XML fixtures in `tests/*.rs` plus narrow assertions
about parse results, validation failures, or semantic round-trips.

## Repo-specific notes

- Current fixtures use deterministic literal dates such as `2026-04-03`; keep
  tests stable and avoid introducing time-dependent output.
- `docs/reference/` is reference material, not generated code. Do not edit it
  unless the task is specifically about updating the bundled spec snapshot.
