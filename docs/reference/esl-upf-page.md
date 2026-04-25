# UPF

Local Markdown transcription based on <https://esl.cecam.org/en/data/upf/index.html>.
This file captures the substantive page content in a local Markdown form.

## Introduction

Current UPF specification version: **2.0.1**.

Readers listed on the page: [Quantum Espresso](https://www.quantum-espresso.org/), [abinit](https://www.abinit.org/), and [gpaw](https://wiki.fysik.dtu.dk/gpaw/) (the page marks the list as incomplete).

Generators listed on the page: [Quantum Espresso](https://www.quantum-espresso.org/) (also marked as incomplete).

The page points to the official UPF documentation by P. Giannozzi on the Quantum Espresso site and describes UPF 2.0.1 as the latest stable version since 2010. It also warns that ultrasoft pseudopotentials written as version 2.0.0 may be affected by an older bug fixed in later versions.

### Content

The page describes UPF as a container for most element-specific data used by Quantum Espresso, including norm-conserving pseudopotentials, ultrasoft pseudopotentials, PAW datasets, bare `1/r` potentials, and core-wavefunction information used to reconstruct all-electron density in magnetic-resonance or X-ray absorption workflows.

### Units

The page states that UPF follows Quantum Espresso atomic Rydberg units: `e^2 = 2`, `m = 1/2`, `hbar = 1`. Lengths are in Bohr radii, energies are in Ry, and potentials are multiplied by `e` so they carry energy units.

### Format

The page explains that UPF was originally designed around the IOTK FORTRAN90 library. The format is XML-like, but not fully XML-compliant. One example given is numbered element names such as `<PP_BETA.1>` and `<PP_BETA.2>`.

The syntax summary given on the page includes:

- Elements open with tags such as `<FOO>` and close with `</FOO>`.
- Empty elements may be written as `<FOO/>`.
- Element names are case-sensitive, and trailing characters after `>` are to be ignored.
- Elements may carry attributes. For some attributes, notably `type` and `size`, the page says the enclosed data should be consistent with the attribute values.
- The root element must be `UPF` and must carry a `version` attribute.

```xml
<UPF version="2.0.1">
  ...
</UPF>
```

The page also defines UPF "data elements" with optional attributes such as `type`, `size`, `columns`, `len`, and `fmt`. It notes that IOTK enforces the expected intrinsic type when `type` is omitted and ignores formatting-oriented attributes such as `columns` and `fmt` during reading.

Finally, the page says that although UPF is not XML, it can still be validated against a schema. It credits initial XSD work to L. Talirz and links to the ongoing effort on GitHub.

## Specification of the elements

The page says the list that follows reflects the elements and subelements that can occur in UPF as written by Quantum Espresso.

### PP_INFO

`PP_INFO` is described as the human-readable, non-parse-oriented part of a UPF file. It is optional. The page says it usually contains:

- Generator name and version
- Author
- Generation date
- Pseudopotential type, with allowed values `NC`, `US`, and `PAW`
- Element label, limited to two characters
- Exchange-correlation functional
- Suggested plane-wave cutoffs for wavefunctions and density
- Relativistic treatment: non-relativistic, scalar-relativistic, or fully relativistic
- Information about the effective local ionic potential used before unscreening valence electrons
- Whether spin-orbit or GIPAW reconstruction data is present
- Valence and generation configurations, including cases where they differ
- Further author comments
- An optional nested `<PP_INPUTFILE>` block containing the generation input file

The page gives additional discussion of local-potential choices, including the meaning of `lloc = -1` and `lloc = -2`, and notes that for PAW this local potential corresponds to the Kresse-Joubert zero-potential construction.

### PP_HEADER

The page describes `PP_HEADER` as a special element: structurally empty, but carrying a large set of attributes that are essential for parsing the rest of the file. It says the header must be processed before the remaining parseable sections because its values control how later sections are read.

Specific attributes called out on the page include:

- `is_coulomb`: indicates a bare `1/r` potential instead of a pseudopotential
- `has_wfc`: says whether all-electron partial waves are written
- `paw_as_gipaw`: indicates whether PAW data is reused for GIPAW reconstruction
- `lmax` and `lmax_rho`: maximum angular-momentum channels in the potential and density, respectively

#### Comparison with PAW-XML

The page includes a comparison between UPF and PAW-XML. It notes that UPF stores `element` and `z_valence` as header attributes, while the atomic number can often be inferred from `zmesh` in `PP_MESH`. It also notes that UPF uses QE functional names rather than LibXC names, that `pseudo_type` and the boolean flags introduce some redundancy, and that `core_energy` in `PP_PAW` has no strict PAW-XML equivalent.

#### Parsing implications

The page stresses that header attributes determine the presence or reading behavior of sibling elements. It gives Quantum Espresso examples:

- The default radial-grid size comes from the header if `PP_MESH` omits it.
- `number_of_proj` controls how many `PP_BETA` entries are read.
- `number_of_wfc` similarly controls `PP_CHI` entries.
- `is_ultrasoft` and `is_paw` determine whether `PP_AUGMENTATION` is read.
- `has_wfc` controls reading of `PP_FULL_WFC`.
- `has_so` affects relativistic sections such as `PP_SPIN_ORB` and `PP_AEWFC_REL`.
- `is_paw` controls the PAW subtree.
- `has_gipaw` and `paw_as_gipaw` control GIPAW reconstruction sections and assignments.

The page also lists a set of strictly needed, non-defaulted attributes, including `element`, `pseudo_type`, `relativistic`, `is_ultrasoft`, `is_paw`, `core_correction`, `functional`, `z_valance`, `mesh_size`, `number_of_wfc`, and `number_of_proj`.

### PP_MESH

The page says UPF assumes a single radial grid shared by all radial functions in the file. That grid definition is stored in `PP_MESH`, which must follow `PP_HEADER`.

```xml
<PP_MESH dx="dx" mesh="m" xmin="xmin" rmax="rmax" zmesh="Z">
  <PP_R type="real" size="m" columns="c">
     r(1) r(2) ... r(m)
  </PP_R>
  <PP_RAB type="real" size="m" columns="c">
     rab(1) rab(2) ... rab(m)
  </PP_RAB>
</PP_MESH>
```

The page explains the attributes `dx`, `mesh`, `xmin`, `rmax`, and `zmesh`, including the interpretation of `zmesh` as the atomic number used to construct the radial logarithmic grid. It gives the analytic form `r_i = (1 / Z) exp(xmin) exp((i - 1) dx)` and the derivative `dr_i/di = r_i dx`.

It then stresses that QE itself uses the numerical subelements `PP_R` and `PP_RAB`, not the analytic reconstruction parameters, so other analytic grid constructions can also be represented if the numeric arrays are consistent. The page notes an exception for certain QE codes such as `atomic` and parts of GIPAW that reuse the analytic form.

Finally, it says that `PP_R` and `PP_RAB` are data elements and that their `size` attributes, when present, must match the mesh size.

### PP_NLCC

The page says that when `PP_HEADER` has `core_correction="true"`, the file must contain `PP_NLCC`. This section stores the radial core-charge density used for non-linear core correction.

```xml
<PP_NLCC type="real" size="m" columns="c">
  rho_core(1) rho_core(2) ... rho_core(m)
</PP_NLCC>
```

It states that the stored radial density is normalized so that integrating `4 * pi * rho_core(r) dr` yields the total core charge, without additional factors. It also notes that the stored density need not equal the exact all-electron core density; a pseudized core charge can also be used.

The page includes a theory note: because exchange-correlation potentials are not linear in the density, a simple unscreening of the valence part introduces an error. Non-linear core correction fixes this by evaluating exchange-correlation on the combined valence-plus-core density during runtime. The page also notes that in practice a smooth approximate core density may suffice in the region where valence and core densities overlap.

Source page footer date shown on the page: May 12, 2020.
