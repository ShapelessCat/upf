# UPF Round-Trip Design

## Goal

Implement full UPF 2.0.1 read/write support for this crate using `serde` and
`quick-xml`, without introducing a separate XML parser layer, while exposing a
typed `UpfData` model and the public entry points promised in the README:
`from_str`, `from_reader`, and `from_file`.

## Scope

This design targets the full published UPF structure, including:

- Core sections: `PP_INFO`, `PP_HEADER`, `PP_MESH`, `PP_NLCC`, `PP_LOCAL`,
  `PP_NONLOCAL`, `PP_SEMILOCAL`, `PP_PSWFC`, `PP_FULL_WFC`, `PP_RHOATOM`
- PAW data: `PP_PAW_FORMAT_VERSION`, `PP_AUGMENTATION`, `PP_AE_NLCC`,
  `PP_AE_RHO_ATC`, `PP_AEWFC`, `PP_PSWFC_FULL`, `PP_AEVLOC`, `PP_KDIFF`,
  `PP_OCCUP`, `PP_GRID_RECON`
- GIPAW data: `PP_GIPAW_FORMAT_VERSION`, `GIPAW_CORE_ORBITALS`,
  `GIPAW_LOCAL_DATA`, `GIPAW_ORBITALS`

The local reference copy of the spec is
[`docs/reference/upf-spec.html`](/Users/shapeless_cat/MzProjects/tmp/upf/docs/reference/upf-spec.html).

## Specification Anchors

The implementation is anchored to these UPF specification rules:

1. The file root is `<UPF version="...">`.
2. `PP_INFO` should appear first for human readers.
3. `PP_HEADER` must precede `PP_MESH`.
4. `PP_MESH` must precede all remaining fields.
5. Field contents may be numeric data, character strings, or other fields.
6. Numeric field bodies are written in Fortran free format, which means the XML
   structure alone is not enough: many sections store scientific data as raw
   whitespace-delimited text within a tag body.

These rules drive the distinction between:

- XML attributes on a tag
- Nested XML child sections
- Plain text payloads stored inside a tag body

## Architecture

The crate will use a two-layer design:

1. Public typed model layer
   - `UpfData` and nested section structs represent the UPF domain.
   - This is the API users interact with after deserialization and before
     serialization.
2. Local serde/text helper layer
   - Small helpers translate whitespace-delimited text payloads into typed Rust
     vectors, scalars, and record bodies.
   - `quick-xml` handles the XML element and attribute mapping directly.

There will be no standalone parsing AST and no separate tokenizer for the XML
envelope. The only custom parsing logic will be section-local text decoding for
UPF bodies that the spec defines as free-format values inside a field.

## Data Model

The top-level shape mirrors the spec ordering:

```rust
pub struct UpfData {
    pub version: String,
    pub info: Option<PpInfo>,
    pub header: PpHeader,
    pub mesh: PpMesh,
    pub nlcc: Option<PpNlcc>,
    pub local: PpLocal,
    pub nonlocal: PpNonlocal,
    pub semilocal: Option<PpSemilocal>,
    pub pswfc: Option<PpPswfc>,
    pub full_wfc: Option<PpFullWfc>,
    pub rhoatom: PpRhoAtom,
    pub paw: Option<PpPaw>,
    pub gipaw: Option<PpGipaw>,
}
```

### Attribute Mapping

Fields expressed by the spec as XML attributes will use serde names prefixed
with `@`.

Example:

```rust
#[derive(Serialize, Deserialize)]
struct PpHeader {
    #[serde(rename = "@element")]
    element: String,
    #[serde(rename = "@pseudo_type")]
    pseudo_type: String,
    #[serde(rename = "@mesh_size")]
    mesh_size: usize,
    #[serde(rename = "@number_of_proj")]
    number_of_proj: usize,
}
```

This matches spec fragments like:

```xml
<PP_HEADER element="O" pseudo_type="US" mesh_size="1234" number_of_proj="2" />
```

### Text Payload Mapping

Sections whose body is a scalar or whitespace-delimited vector will use
`#[serde(rename = "$text")]`.

Examples from the spec:

- `PP_R`
- `PP_RAB`
- `PP_NLCC`
- `PP_LOCAL`
- `PP_RHOATOM`
- `PP_GIPAW_FORMAT_VERSION`

Example:

```rust
#[derive(Serialize, Deserialize)]
struct NumericArray {
    #[serde(rename = "$text")]
    values: Vec<f64>,
}
```

This matches:

```xml
<PP_R>0.0 0.1 0.2 0.3</PP_R>
```

### Structured Nested Sections

Sections that contain repeated tagged records or nested scientific subformats
will be represented as dedicated typed structs rather than a single `$text`
field. Important cases:

- `PP_NONLOCAL`
  - repeated `PP_BETA`
  - `PP_DIJ`
  - optional augmentation structures including `PP_QIJ`, `PP_RINNER`,
    `PP_QFCOEF`
- `PP_PSWFC`
  - repeated pseudo-wavefunction records
- `PP_FULL_WFC`
  - repeated all-electron / pseudo full-wavefunction records
- `PP_PAW`
  - PAW-only subtree with augmentation and auxiliary arrays
- `PP_GIPAW`
  - GIPAW-only subtree with core orbitals, local data, and orbital pairs

These types may still contain `$text` children for numeric arrays, but their
outer structure will be modeled directly as nested XML fields.

## Section Design

### `PP_INFO`

`PP_INFO` is for human-readable regeneration metadata and may include free text
plus a `PP_INPUTFILE` nested section. The design will preserve it losslessly as
textual content plus the optional nested input file block.

### `PP_HEADER`

`PP_HEADER` is a pure attribute container in the spec and will be modeled
primarily with typed attribute fields:

- identity metadata: `generated`, `author`, `date`, `comment`, `element`
- physical classification: `pseudo_type`, `relativistic`
- boolean feature flags: `is_ultrasoft`, `is_paw`, `is_coulomb`, `has_so`,
  `has_wfc`, `has_gipaw`, `core_correction`
- counts and shape metadata: `mesh_size`, `number_of_wfc`, `number_of_proj`
- cutoffs and limits: `z_valence`, `total_psenergy`, `wfc_cutoff`,
  `rho_cutoff`, `l_max`, `l_max_rho`, `l_local`

Boolean values will be normalized through serde helpers to support UPF-style
textual booleans such as `T`, `F`, `.T.`, and `.F.` if encountered in real
files.

### `PP_MESH`

`PP_MESH` has container attributes and two required numeric vector children:

- `PP_R`
- `PP_RAB`

The model keeps those vectors typed as `Vec<f64>`.

### `PP_NLCC`, `PP_LOCAL`, `PP_RHOATOM`

These are direct numeric payload sections and will be represented by thin
wrapper types over `Vec<f64>`.

### `PP_SEMILOCAL`

`PP_SEMILOCAL` is optional and contains repeated semilocal channel sections such
as `PP_VNL1`, `PP_VNL2`, each with channel attributes and a numeric payload.
The model will treat these as a sequence of semilocal channels with:

- the original tag name
- angular metadata (`L`, optionally `J`)
- radial values

### `PP_NONLOCAL`

`PP_NONLOCAL` is the most structurally important nested section for nonlocal
projectors. It needs explicit subtypes for:

- beta projectors
- the `DIJ` matrix payload
- optional augmentation data and associated radii/coefficient tables

The design prioritizes semantic structure over mirroring every record as a raw
string because this section is central to downstream scientific use.

### `PP_PSWFC` and `PP_FULL_WFC`

Wavefunction sections will be modeled as repeated orbital records with:

- identifying attributes such as labels and angular momentum metadata
- occupancy where applicable
- radial values stored as numeric vectors

### `PP_PAW`

The PAW subtree will be encapsulated behind `Option<PpPaw>` so PAW-specific
logic stays isolated from non-PAW files. It includes:

- format version
- augmentation data
- all-electron auxiliary arrays including `PP_AE_NLCC`, `PP_AE_RHO_ATC`, and
  `PP_AEVLOC`
- pseudo and all-electron wavefunctions needed by PAW
- occupancy data
- kinetic-difference data from `PP_KDIFF`
- reconstruction-grid metadata from `PP_GRID_RECON`

### `PP_GIPAW`

The GIPAW subtree will be encapsulated behind `Option<PpGipaw>`. It includes:

- format version
- core orbital records
- local AE/PS potential data
- AE/PS orbital reconstruction data

## Public API

The public API will expose:

- `upf::from_str(&str) -> Result<UpfData, UpfError>`
- `upf::from_reader<R: BufRead>(R) -> Result<UpfData, UpfError>`
- `upf::from_file<P: AsRef<Path>>(P) -> Result<UpfData, UpfError>`
- `upf::to_string(&UpfData) -> Result<String, UpfError>`
- `upf::to_writer<W: Write>(&mut W, &UpfData) -> Result<(), UpfError>`
- `upf::to_file<P: AsRef<Path>>(P, &UpfData) -> Result<(), UpfError>`

The user explicitly requested bi-directional support, so writing support is part
of the design, not a follow-up.

## Round-Trip Rules

The serializer must produce valid UPF-form XML with the correct section order
and field names. Round-trip guarantees are:

- deserialize valid UPF into typed `UpfData`
- serialize `UpfData` back into a valid UPF document
- preserve scientifically meaningful numeric content exactly at the parsed value
  level
- preserve section presence and ordering required by the spec

Text formatting details such as whitespace normalization, indentation, or line
wrapping do not need to match the original byte-for-byte source.

## Error Handling

The crate will expose a dedicated `UpfError` type built with `thiserror`.
Expected error classes:

- XML decode and encode failures from `quick-xml`
- invalid numeric payloads in free-format text sections
- missing required sections
- inconsistent metadata, such as declared sizes that disagree with parsed array
  lengths
- unsupported malformed variants that violate the published spec shape

Validation should happen after deserialization into typed structures so the XML
mapping stays straightforward and the domain checks remain explicit.

## Validation Strategy

The implementation will validate:

- required top-level section presence
- conditional section presence for PAW and GIPAW flags
- array lengths versus header counts where the spec gives a clear contract
- nonlocal and wavefunction record consistency

Validation will not silently coerce broken files.

## Testing Strategy

The implementation will be developed test-first and verified with:

- focused unit tests for payload parsing helpers
- XML serde tests for attributes and `$text` sections
- round-trip tests for representative UPF snippets
- full-document tests covering:
  - minimal core UPF
  - norm-conserving with `PP_SEMILOCAL`
  - ultrasoft with `PP_NONLOCAL`
  - PAW with `PP_PAW` and `PP_FULL_WFC`
  - GIPAW-enabled UPF with GIPAW subtree
- negative tests for malformed counts and invalid payload text

## Implementation Notes

The crate should be split into focused modules rather than one large `lib.rs`.
Expected structure:

- `src/lib.rs` for the public API surface
- `src/error.rs` for `UpfError`
- `src/model.rs` or `src/model/*` for UPF section structs
- `src/de.rs` and `src/ser.rs` for top-level read/write helpers
- `src/text/*` for whitespace-delimited payload codecs and boolean adapters
- `tests/*` for round-trip and invalid-input coverage

## Non-Goals

- byte-for-byte formatting preservation
- heuristic support for arbitrary non-spec vendor extensions in the first pass
- an alternate handwritten XML parser

## Decision Summary

The crate will implement full UPF 2.0.1 support with:

- `serde` + `quick-xml` for XML structure
- `@attribute` mapping for XML attributes
- `$text` mapping for scalar/vector payload bodies
- local typed helpers for irregular scientific record sections
- full read and write APIs around a typed `UpfData` model
