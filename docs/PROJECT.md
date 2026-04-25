# Project Guide

## Purpose

`upf` is a Rust library for working with Unified Pseudopotential Format (UPF)
documents as typed Rust data. The current codebase supports both directions:

- read UPF text into a validated [`UpfData`](../src/model/upf_data.rs) structure
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

- `src/model/upf_data.rs`
  Defines the root `UpfData` type and the central validation logic, including
  top-level sections such as `PP_TAUMOD` and `PP_TAUATOM`.
- `src/model/header.rs`
  Defines `PP_HEADER` and its typed enums.
- `src/model/info.rs`, `src/model/mesh.rs`
  Define `PP_INFO` and `PP_MESH`.
- `src/model/nonlocal.rs`, `src/model/semilocal.rs`, `src/model/pseudo_wavefunctions.rs`
  Define `PP_NONLOCAL`, `PP_SEMILOCAL`, `PP_PSWFC`, and related numbered content.
- `src/model/full_wfc.rs`, `src/model/paw.rs`, `src/model/gipaw.rs`
  Define `PP_FULL_WFC`, `PP_PAW`, `PP_GIPAW`, and their nested structures.
- `src/model/spin_orb.rs`
  Defines `PP_SPIN_ORB`.
- `src/model/common.rs`, `src/model/common/wavefunction.rs`
  Define reusable shared section shapes, currently `PpWavefunction` for
  `PP_CHI.n`, `PP_AEWFC.n`, `PP_PSWFC.n`, and `PP_AEWFC_REL.n`.
- `src/model/internal.rs`, `src/model/internal/*.rs`
  Gather helper types and serde glue such as numbering-aware tag wrappers and
  numeric section text adapters.

### Support code

- `src/error.rs`
  Defines `UpfError` for XML decode/encode, I/O, value parsing, and validation
  failures.
- `src/text.rs`
  Provides helpers for whitespace-delimited numeric fields and UPF boolean
  flags.

## Validation rules

`UpfData::validate()` enforces structural invariants after deserialization and
before serialization. Both read and write paths share the same contract.

### Mesh and size consistency

- `PP_HEADER/@mesh_size` must match the lengths of `PP_R`, `PP_RAB`,
  `PP_LOCAL`, `PP_RHOATOM`, and any enabled metagga sections
- Declared `@size` attributes on numeric sections must match their payload
  lengths (applies to `PP_R`, `PP_RAB`, `PP_LOCAL`, `PP_RHOATOM`, `PP_NLCC`,
  `PP_TAUMOD`, `PP_TAUATOM`, `PP_Q`, `PP_MULTIPOLES`, `PP_QFCOEF`,
  `PP_RINNER`, `PP_BETA.n`, `PP_VNL.n`, `PP_CHI.n`, `PP_AEWFC.n`,
  `PP_PSWFC.n`, `PP_AEWFC_REL.n`, augmentation channels, GIPAW core/valence
  orbital sections, and PAW sections)

### Required section presence

- `PP_HEADER/@is_paw="T"` requires a `PP_PAW` section (with `PP_OCCUPATIONS`)
- `PP_HEADER/@has_gipaw="T"` requires a `PP_GIPAW` section
- `PP_HEADER/@has_so="T"` requires a `PP_SPIN_ORB` section
- `PP_HEADER/@has_wfc="T"` requires a `PP_FULL_WFC` section
- `PP_HEADER/@with_metagga_info="T"` requires both `PP_TAUMOD` and `PP_TAUATOM`
- `PP_HEADER/@core_correction="T"` requires a `PP_NLCC` section
- Non-Coulomb datasets require `PP_LOCAL`
- Ultrasoft and PAW datasets require `PP_AUGMENTATION` inside `PP_NONLOCAL`

### Count consistency

- `PP_HEADER/@number_of_proj` must match the number of `PP_BETA.n` entries
- `PP_HEADER/@number_of_wfc` must match the number of `PP_CHI.n` entries
  (when > 0)
- `PP_FULL_WFC/@number_of_wfc` must match entry family counts for
  `PP_AEWFC.n`, `PP_PSWFC.n`, and `PP_AEWFC_REL.n` (when present)
- Declared GIPAW core and valence orbital counts must match their numbered
  entries
- When `has_so` is true: `PP_RELWFC.n` count must equal `number_of_wfc`
  and `PP_RELBETA.n` count must equal `number_of_proj` (when > 0)

### Cross-section consistency

- `PP_AEWFC_REL.n` entries are only valid when both `has_so` and `is_paw`
  are true; when `has_so && is_paw && has_wfc`, at least one must be present
- When `paw_as_gipaw` is true, `PP_GIPAW_ORBITALS` and `PP_GIPAW_VLOCAL`
  must be absent (only `PP_GIPAW_CORE_ORBITALS` is allowed)
- When `q_with_l` is true, augmentation channels must use `PP_QIJL.i.j.l`
  naming (3 dot-separated indices) with `angular_momentum` present; when
  false, channels must use `PP_QIJ.i.j` naming (2 indices) without
  `angular_momentum`

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
  `AtomicRelativisticFormalism`. QE writes `no`, `scalar`, and `full`.
  The older prose reference used `nonrelativistic` for the first variant.
  The crate serializes the canonical QE spelling `no` and accepts
  `nonrelativistic` as an alias on input.
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
- `PP_TAUMOD`
- `PP_TAUATOM`
- `PP_SPIN_ORB`
- `PP_PAW`
- `PP_GIPAW`

Optional sections are represented as `Option<T>`. Repeated numbered tags such
as `PP_BETA.n`, `PP_VNL.n`, `PP_CHI.n`, `PP_RELWFC.n`, and PAW/GIPAW entry
lists are represented with numbering-aware structs and vectors that match the
serialized UPF tags.

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
