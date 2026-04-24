# Differences Between Unified Pseudopotential Format Spec 1.x and Spec 2.x

The core difference is:

**UPF Spec 1.x is a Fortran-readable tagged text format. UPF Spec 2.x is an XML-like tagged format with structured attributes.**

Do **not** treat either one as clean XML. Spec 2.x looks much closer to XML, but the official QE page still calls it **“XML-like”**, and notes that a future overhaul would make it a “true XML format with a schema.”

## 1. File-level structure

| Area                 | UPF 1.x                                                           | UPF 2.x                                                         |
|----------------------|-------------------------------------------------------------------|-----------------------------------------------------------------|
| Root tag             | Usually no meaningful root version wrapper, or older loose layout | Starts with `<UPF version="2.0.1">` and ends with `</UPF>`      |
| Parsing model        | Search for sections, then read fixed-position/free-format values  | Search for named fields and read attributes/subfields           |
| Strictness           | Looser, more legacy/parser-driven                                 | More declarative, but still not strict XML in the official spec |
| Best parser strategy | Legacy state machine / Fortran-style scanner                      | XML-like event parser plus UPF-specific tolerant rules          |

For a Rust parser, the trap is thinking: **“2.x equals XML.”** It does not fully. The spec allows behavior that is not clean modern XML behavior, such as ignored trailing characters after a delimiter and Fortran free-format numeric data.

## 2. Header: biggest practical difference

### UPF 1.x

The header is mostly **positional body text**. Legacy QE readers read values in a fixed sequence:

```text
version
element
pseudo type
nlcc flag
functional
z_valence
total_psenergy
ecutwfc ecutrho
lmax
mesh
nwfc nbeta
wavefunction metadata...
```

Conceptually, the parser reads something like:

```text
<PP_HEADER>
  version
  element
  pseudo_type
  nlcc
  functional
  z_valence
  total_psenergy
  ecutwfc ecutrho
  lmax
  mesh
  nwfc nbeta
  ...
</PP_HEADER>
```

### UPF 2.x

The header becomes an **attribute-heavy tag**:

```xml
<PP_HEADER
  generated="..."
  author="..."
  date="..."
  element="..."
  pseudo_type="NC | SL | 1/r | US | PAW"
  relativistic="scalar | full | nonrelativistic"
  is_ultrasoft=".F."
  is_paw=".F."
  core_correction=".F."
  functional="..."
  z_valence="..."
  mesh_size="..."
  number_of_wfc="..."
  number_of_proj="..."
>
```

**Parser implication:**

For 1.x, deserialize `PP_HEADER` as ordered text records.

For 2.x, deserialize `PP_HEADER` as attributes.

Trying to unify them too early is bad design. Use two raw AST/input structs, then convert into one normalized internal model.

## 3. Top-level fields

UPF 2.x defines first-level fields like:

```text
PP_INFO
PP_HEADER
PP_MESH
PP_NLCC
PP_LOCAL
PP_NONLOCAL
PP_SEMILOCAL
PP_PSWFC
PP_FULL_WFC
PP_RHOATOM
PP_PAW
```

The official 2.x spec says `PP_HEADER` must precede `PP_MESH`, and `PP_MESH` must precede the remaining fields. Undefined fields are ignored.

UPF 1.x has similar conceptual sections, but the content inside them is older and more positional.

Common legacy sections include:

```text
PP_HEADER
PP_MESH
PP_NLCC
PP_LOCAL
PP_NONLOCAL
PP_PSWFC
PP_RHOATOM
PP_ADDINFO
GIPAW_RECONSTRUCTION_DATA
```

## 4. Mesh section

### UPF 1.x

The v1 reader searches the mesh block, then separately scans for `PP_R` and `PP_RAB`, reading arrays directly:

```xml
<PP_MESH>
  <PP_R>
    ...
  </PP_R>
  <PP_RAB>
    ...
  </PP_RAB>
</PP_MESH>
```

Mesh metadata may need to come from the header or be inferred from the data.

### UPF 2.x

The mesh is more attribute-based:

```xml
<PP_MESH dx="..." mesh="..." xmin="..." rmax="..." zmesh="...">
  <PP_R>
    ...
  </PP_R>
  <PP_RAB>
    ...
  </PP_RAB>
</PP_MESH>
```

**Parser implication:**

In 1.x, mesh metadata may be partly positional or inferred.

In 2.x, mesh metadata is mostly attached to `PP_MESH`.

## 5. Nonlocal projectors and augmentation data

This is where a parser gets ugly.

### UPF 1.x

`PP_NONLOCAL` stores beta projectors and `D_ij` / `Q_ij` data in a more positional way.

A legacy reader typically does this:

```text
for each beta projector:
    read beta label / index
    read angular momentum
    read cutoff length
    read beta radial array

read DIJ matrix

if ultrasoft or PAW:
    read QIJ / augmentation data
```

### UPF 2.x

The same physical content is split into more explicit subfields:

```text
PP_BETA
PP_DIJ
PP_QIJ
PP_Q
PP_QFCOEFF
PP_RINNER
```

This is more structured, but numeric payloads are still usually free-format arrays inside tags.

**Important:** UPF **2.0.0** is risky for ultrasoft pseudopotentials. Some UPF 2.0.0 ultrasoft files may be compromised because of a bug fixed in later versions. Prefer 2.0.1 or later when possible.

## 6. PAW / GIPAW handling

UPF 2.x formally supports more dataset types and metadata, including:

```text
NC
semilocal NC
nonlocal NC
ultrasoft
PAW
GIPAW-related reconstruction data
```

In 1.x, these features exist in a more legacy-compatible way. Readers usually detect flags like spin-orbit, PAW, ultrasoft, or GIPAW by scanning extra info and then conditionally reading additional blocks.

## 7. Validation and schema

| Format                  | Schema story                          |
|-------------------------|---------------------------------------|
| UPF 1.x                 | No useful XML schema model            |
| UPF 2.0.1 official spec | XML-like, not fully true XML          |
| Community / ESL schema  | There is an XSD attempt for UPF 2.0.1 |

A schema can be useful for **well-behaved UPF 2.0.1 files**, but it is not a universal guarantee for all real-world UPF files.

Do not assume every UPF file from the wild validates as clean XML.

## 8. Migration summary

| Problem                | UPF 1.x                          | UPF 2.x                                |
|------------------------|----------------------------------|----------------------------------------|
| Header parsing         | Ordered free-format lines        | Attributes                             |
| Metadata quality       | Weaker, sometimes implicit       | Stronger, more explicit                |
| Numeric arrays         | Free-format Fortran arrays       | Still free-format arrays inside fields |
| XML compatibility      | No                               | Partial / XML-like                     |
| Unknown fields         | Legacy readers often scan/ignore | Spec says undefined fields are ignored |
| PAW/GIPAW              | More ad hoc                      | More formally represented              |
| Validation             | Parser-specific                  | Some schema-based validation possible  |
| Best code architecture | Legacy reader                    | XML-like reader with tolerance         |

## 9. Recommended Rust model

Do **not** design one Serde struct and hope it handles both. That will rot.

Use this architecture:

```rust
enum UpfRaw {
    V1(UpfV1Raw),
    V2(UpfV2Raw),
}

struct UpfModel {
    header: Header,
    mesh: RadialMesh,
    local: Option<LocalPotential>,
    nonlocal: Option<NonlocalProjectors>,
    pswfc: Vec<PseudoWavefunction>,
    rho_atom: Option<RadialData>,
    paw: Option<PawData>,
    gipaw: Option<GipawData>,
}
```

Then convert each raw representation into a normalized model:

```rust
impl TryFrom<UpfV1Raw> for UpfModel {
    type Error = UpfError;

    fn try_from(raw: UpfV1Raw) -> Result<Self, Self::Error> {
        // Convert positional legacy data into normalized model.
        todo!()
    }
}

impl TryFrom<UpfV2Raw> for UpfModel {
    type Error = UpfError;

    fn try_from(raw: UpfV2Raw) -> Result<Self, Self::Error> {
        // Convert XML-like structured data into normalized model.
        todo!()
    }
}
```

The bad path is this:

```rust
#[derive(Deserialize)]
struct Upf {
    // one giant structure for both 1.x and 2.x
}
```

That approach will break on real files because UPF is not normal XML, and v1 is fundamentally a positional legacy format.

## 10. Practical rule

Use this mental model:

```text
UPF 1.x = tagged Fortran text format
UPF 2.x = XML-like scientific data container
UPF internal model = your own normalized semantic representation
```

For a robust parser:

```text
detect version
    -> parse with version-specific frontend
        -> normalize into shared model
            -> validate semantic invariants
```

Do not force the file format to be cleaner than it really is.
