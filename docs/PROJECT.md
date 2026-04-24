# Project Guide

## Purpose

`upf` is a Rust library for working with Unified Pseudopotential Format (UPF)
documents as typed Rust data. The current codebase supports both directions:

- read UPF text into a validated [`UpfData`](../src/model/core.rs) structure
- write a validated `UpfData` value back to UPF text

The project is aimed at semantic round-tripping. A document can be parsed,
serialized, and parsed again into the same Rust data model, even if the exact
whitespace or original layout is not preserved.

## Public API

The crate exposes six primary entry points:

- `from_str`: parse a UPF document from a UTF-8 string
- `from_reader`: parse a UPF document from a buffered reader
- `from_file`: parse a UPF document from a file path
- `to_string`: serialize a validated `UpfData` into UPF text
- `to_writer`: serialize a validated `UpfData` into any writer
- `to_file`: serialize a validated `UpfData` to a file path

Parse and write operations use the shared public model type `UpfData` and
return `Result<_, UpfError>`.

## Current architecture

The implementation is organized around serde-based XML mapping rather than a
custom parser pipeline.

### Entry points

- `src/de.rs`
  Read-side APIs. These use `quick_xml::de` to deserialize a full document into
  `UpfData`, then run semantic validation.
- `src/ser.rs`
  Write-side APIs. These validate `UpfData` first, then use `quick_xml::se` to
  serialize it back into UPF text.

### Public model

- `src/model/core.rs`
  Defines the root `UpfData` type, `PP_HEADER`, `PP_MESH`, shared numeric
  arrays, and the central validation logic.
- `src/model/nonlocal.rs`
  Defines `PP_INFO`, `PP_NONLOCAL`, `PP_SEMILOCAL`, `PP_PSWFC`, and related
  nested nodes.
- `src/model/paw.rs`
  Defines PAW-specific sections such as `PP_FULL_WFC`, `PP_PAW`, and
  `PP_AUGMENTATION`.
- `src/model/gipaw.rs`
  Defines GIPAW-specific sections.

### Support code

- `src/error.rs`
  Defines `UpfError` for XML decode/encode, I/O, value parsing, and validation
  failures.
- `src/text.rs`
  Provides helpers for whitespace-delimited numeric fields and UPF boolean
  flags.

## Validation rules

The crate currently enforces a small set of structural invariants in
`UpfData::validate()`:

- `PP_HEADER/@mesh_size` must match the lengths of `PP_R`, `PP_RAB`,
  `PP_LOCAL`, and `PP_RHOATOM`
- `PP_HEADER/@is_paw="T"` requires a `PP_PAW` section
- `PP_HEADER/@has_gipaw="T"` requires a `PP_GIPAW` section

These checks run after deserialization and before serialization, so both read
and write paths enforce the same structural contract.

## Schema notes

This repository bundles the prose UPF reference in `docs/reference/`, but
there is also sibling schema work in `/Users/shapeless_cat/MzProjects/upf-schema`
that is useful for understanding edge cases in real UPF files.

The crate currently follows these documented interpretations:

- `PP_HEADER/@pseudo_type` is modeled as the typed Rust enum
  `PseudopotentialType` and serializes as the compact UPF values `NC`, `SL`,
  `1/r`, `US`, and `PAW`.
- `PP_HEADER/@relativistic` is modeled as
  `AtomicRelativisticFormalism`. The prose reference uses
  `nonrelativistic`, `scalar`, and `full`, while the sibling XSD uses `no`,
  `scalar`, and `full`. The crate accepts both `nonrelativistic` and `no`
  when reading, and writes the canonical prose spelling
  `nonrelativistic`.
- UPF boolean flags are currently treated as `T`/`F` values in this crate.
  This matches the example fixtures used in the repository and the current
  round-trip serializer output.
- The sibling XSD defines a broader `upf-logical` type that accepts both
  `.true.` / `.false.` and `T` / `F`.
- Two `PP_HEADER` attributes, `has_wfc` and `paw_as_gipaw`, are typed as
  `xs:NCName` in the sibling XSD even though the prose reference describes
  them as booleans. The crate intentionally keeps the stronger boolean
  interpretation for both fields.

These notes are not an attempt to make the crate schema-driven. They document
where real-world schema material and the prose reference disagree, and where
the library has chosen one interpretation for a stable typed API.

## Supported UPF sections

The current top-level model covers these sections:

- `PP_INFO`
- `PP_HEADER`
- `PP_MESH`
- `PP_NLCC`
- `PP_LOCAL`
- `PP_SEMILOCAL`
- `PP_NONLOCAL`
- `PP_PSWFC`
- `PP_FULL_WFC`
- `PP_RHOATOM`
- `PP_PAW`
- `PP_GIPAW`

Optional sections are represented as `Option<T>`. Repeated numbered tags such
as `PP_BETA.n`, `PP_CHI.n`, and PAW/GIPAW entry lists are represented with enums
and vectors that match the serialized UPF tags.

## Current scope and limitations

- The code is built around the UPF `2.0.1` structure currently represented in
  `src/model`.
- Serialization aims to produce valid UPF for the supported model, not to
  preserve original comments, formatting, or unknown sections byte-for-byte.
- The crate does not currently preserve unsupported top-level sections.
- Input still needs to be readable by `quick-xml`; the old custom
  normalization/tree pipeline described in previous docs is no longer part of
  the implementation.

## Testing strategy

The repository uses focused inline fixtures in `tests/*.rs` to cover:

- basic parsing of core sections
- file/string/reader read APIs
- file/string/writer write APIs
- semantic round-tripping
- validation failures for inconsistent sections
- PAW, GIPAW, and nonlocal subtree coverage

## Abbreviation glossary

- `UPF`: Unified Pseudopotential Format
- `PP`: pseudopotential
- `NC`: norm-conserving
- `US`: ultrasoft
- `PAW`: projector augmented wave
- `GIPAW`: gauge including projector augmented wave
- `AE`: all-electron
- `PS`: pseudo
- `WFC`: wavefunction
- `NLCC`: nonlinear core correction
- `RHOATOM`: atomic charge density
- `RAB`: radial integration measure
- `DIJ`: nonlocal projector coupling matrix

## Verification

The current repository verification commands are:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo doc --no-deps` when public API docs or rustdoc are touched
