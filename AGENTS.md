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

- `src/lib.rs`: public API surface; re-exports read/write helpers, errors, and
  the `model` module.
- `src/error.rs`: `UpfError` enum (wraps `quick_xml`, I/O, and validation
  errors via `thiserror`). The `Validation` variant collects multiple invariant
  violations in one pass.
- `src/de.rs`: read-side entry points built on `quick_xml::de`; all successful
  parses end in `UpfData::validate()`.
- `src/ser.rs`: write-side entry points built on `quick_xml::se`; documents are
  validated before serialization/writing.
- `src/model.rs`: module root; re-exports all section submodules, the `common`
  submodule (public), and `Numbered`, `NumberedTag` from `internal`.
- `src/model/upf_data.rs`: root `UpfData` serde struct mapping each first-level
  UPF section (`PP_INFO`, `PP_HEADER`, `PP_MESH`, etc.). Pure data — no
  validation logic here.
- `src/model/header.rs`: `PpHeader` with enums (`PseudopotentialType`,
  `AtomicRelativisticFormalism`, etc.).
- `src/model/mesh.rs`: `PpMesh`, `PpR`, `PpRab`.
- `src/model/info.rs`: `PpInfo`.
- `src/model/nonlocal.rs`: `PpNonlocal`, `PpBeta`, `PpDij`, `PpAugmentation`,
  and augmentation channel types (`AugmentationChannel`, `PpQijChannel`,
  `PpQijlChannel`).
- `src/model/semilocal.rs`: `PpSemilocal`, `PpVnl` channels.
- `src/model/pseudo_wavefunctions.rs`: `PpPswfc` with `PP_CHI.n`.
- `src/model/full_wfc.rs`: `PpFullWfc` with `PP_AEWFC`/`PP_PSWFC.n`.
- `src/model/paw.rs`: PAW-specific sections.
- `src/model/gipaw.rs`: GIPAW-specific sections.
- `src/model/spin_orb.rs`: `PpSpinOrb`, `PpRelWfc`, `PpRelBeta`.
- `src/model/common/`: reusable section shapes shared across top-level sections.
- `src/model/common/wavefunction.rs`: `PpWavefunction` shared type (with
  optional spin-orbit attributes `nn`, `jchi` and optional `n`,
  `pseudo_energy`, `cutoff_radius`, `ultrasoft_cutoff_radius`).
- `src/model/internal/`: private serde/helper glue (not part of public API).
- `src/model/internal/scalar_text.rs`: `bool_flag` serde adapter, Fortran
  `d`-exponent `f64` deserializers, `format_bool_flag`.
- `src/model/internal/numeric_values.rs`: `parse_f64_vec`, `format_f64_slice`,
  and `deserialize_f64_values` serde helper.
- `src/model/internal/numeric_section.rs`: serde adapters for UPF numeric data
  sections (elements with optional `type`/`size`/`columns` attributes and
  whitespace-delimited body text).
- `src/model/internal/tagged_children.rs`: `NumberedTag`, `Numbered<T>`,
  `Tagged<T>` for indexed UPF elements like `PP_BETA.n`, `PP_CHI.n`.
- `src/model_validation.rs`: module root for the validation layer (separate from
  model serde definitions).
- `src/model_validation/common.rs`: shared validation infrastructure
  (`ValidationErrors`, `ValidationContext`, length-check helpers).
- `src/model_validation/upf_data.rs`: `UpfData::validate()` entry point;
  top-level mesh-size checks and flag-vs-section presence checks.
- `src/model_validation/{mesh,semilocal,nonlocal,pseudo_wavefunctions,full_wfc,paw,gipaw,spin_orb}.rs`:
  per-section validation rules.
- `tests/*.rs`: integration test files using inline UPF fixtures.
- `examples/SSSP_1.3.0_PBE_efficiency/`: real-world UPF files organized into
  `UPF_1.x/`, `UPF_2.x/`, and `UPF_future/` subdirectories from the SSSP
  dataset, used for fixture-based testing.
- `docs/PROJECT.md`: detailed project guide.
- `docs/reference/UPF_v2.0.1/`: spec and XSD for UPF 2.0.1 (reverse-engineered
  from Quantum ESPRESSO). Use the `.md` spec together with the `.xsd` schema
  for the most exact understanding.
- `docs/reference/UPF_v3_candidate_in_qe/`: spec and XSD for the upcoming
  format revision.

## Key dependencies

- `quick-xml` (with `serialize` + `serde-types` features): XML
  deserialization/serialization.
- `serde` / `serde_with`: derive-based (de)serialization; heavy use of
  `#[serde(rename = ...)]` to match UPF element/attribute names.
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
  `UpfData::validate()` is implemented in `src/model_validation/upf_data.rs`
  and delegates to per-section validators in `src/model_validation/`. This is
  the central place for mesh-length checks, required-section checks, projector
  counts, augmentation sizes, and GIPAW orbital counts.
- Preserve the public API shape unless the task explicitly requires expansion.
  The crate currently exposes `from_str`, `from_reader`, `from_file`,
  `to_string`, `to_writer`, `to_file`, `UpfError`, `UpfData`, and the `model`
  module (which flat-re-exports all section types plus `Numbered`, `NumberedTag`,
  `Tagged`, and the `common` submodule).
- Treat round-tripping as semantic round-tripping. Existing tests compare parsed
  data structures after reserialization; they do not require byte-for-byte
  preservation of whitespace, comments, or original layout.
- Keep changes narrow. This crate is small and strongly model-driven, so broad
  refactors are usually unnecessary unless they directly improve the target
  behavior.
- Model types (`src/model/`) are pure serde structs; validation logic lives in
  `src/model_validation/`. Keep these layers separate — do not add validation
  logic to model files or serde glue to validation files.
- Serde helpers and internal plumbing live in `src/model/internal/`. These are
  crate-private; do not re-export them beyond what `src/model.rs` already
  exposes (`Numbered`, `NumberedTag`).

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
