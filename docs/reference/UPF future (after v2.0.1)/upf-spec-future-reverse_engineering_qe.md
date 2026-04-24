# Future `qe_pp:pseudo` QE Practical Authoritative Reference

## Scope

This document defines the future `qe_pp:pseudo` form reverse-engineered from
Quantum ESPRESSO's practical reader and writer in `upflib/read_upf_new.f90` and
`upflib/write_upf_new.f90`.

This document follows QE implementation behavior. It is not constrained to the
older UPF v2.0.1 document shape.

## Document Form

The future form is rooted at `qe_pp:pseudo` in the QE PP namespace. QE's writer
emits an XML declaration, namespace declarations for `qe_pp` and `xsi`, an
`xsd_version` element, and then the pseudopotential content.

In practical QE output, only the root element is namespace-qualified. Child
elements are unqualified lowercase names such as `pp_header`, `pp_mesh`, and
`pp_nonlocal`. Header values are encoded as child elements of `pp_header`.
Repeated families use repeated same-name elements such as `pp_beta` and
`pp_chi`.

## Practical Section Order

QE's practical read path is driven by this order:

1. `qe_pp:pseudo`
2. `pp_header`
3. `pp_mesh`
4. `pp_nlcc` when `core_correction` is true
5. `pp_local` when `is_coulomb` is false
6. `pp_semilocal` when `type == "SL"`
7. `pp_nonlocal` unless `number_of_proj = 0`
8. `pp_pswfc`
9. `pp_full_wfc` when `has_wfc` is true
10. `pp_rhoatom`
11. `pp_taumod` and `pp_tauatom` when `with_metagga_info` is true
12. `pp_paw` when `is_paw` is true
13. `pp_gipaw` when `has_gipaw` is true

The future form does not use a `pp_spin_orb` section in this QE reader/writer
path. Spin-orbit metadata is carried on relevant wavefunction and projector
entries.

## Field Specifications

### pp_info

`pp_info` is descriptive metadata. QE writes structured children such as
`generated`, `creator`, `created`, optional `input`, `type`,
`relativistic_effects`, `element`, `functional`, `suggested_basis`, repeated
`valence_orbital`, and optional `generation_configuration`.

The practical reader does not interpret this section into the in-memory UPF
object.

### pp_header

`pp_header` carries machine-readable dataset metadata as child elements. The
practical header surface includes:

- identity: `generated`, `author`, `date`, `comment`, `element`
- classification: `type`, `relativistic`
- flags: `is_ultrasoft`, `is_paw`, `is_coulomb`, `has_so`, `has_wfc`,
  `has_gipaw`, `paw_as_gipaw`, `core_correction`, `with_metagga_info`
- scalars and counts: `functional`, `z_valence`, `total_psenergy`,
  `wfc_cutoff`, `rho_cutoff`, `l_max`, `l_max_rho`, `l_local`, `mesh_size`,
  `number_of_wfc`, `number_of_proj`

The header must be processed before mesh-sized or count-sized arrays because
later allocations depend on `mesh_size`, `number_of_wfc`, and `number_of_proj`.

### pp_mesh

`pp_mesh` contains `pp_r` and `pp_rab`. QE reads mesh attributes such as `mesh`,
`dx`, `xmin`, `rmax`, and `zmesh` when present. If `pp_mesh@mesh` differs from
the header `mesh_size`, QE updates the in-memory mesh size from the section
attribute.

Numeric children may carry metadata such as `type`, `size`, and `columns`; the
reader is primarily driven by header and mesh counts, not by those metadata
attributes.

### pp_nlcc

`pp_nlcc` contains pseudized core charge on the radial mesh. QE reads it only
when `core_correction` is true. Otherwise the in-memory array is allocated and
filled with zeros.

### pp_local

`pp_local` contains local potential samples. QE reads it only when `is_coulomb`
is false.

### pp_nonlocal

`pp_nonlocal` contains projectors, the dense `D_ij` matrix, and augmentation
data for ultrasoft or PAW datasets. If `number_of_proj = 0`, QE returns before
opening this section.

Projectors are repeated `pp_beta` elements. Practical projector metadata
includes `index`, `label`, `angular_momentum`, `cutoff_radius_index`,
`cutoff_radius`, `ultrasoft_cutoff_radius`, and `tot_ang_mom` when spin-orbit
metadata is present.

`pp_dij` contains the dense projector coupling matrix and is written with row
and column metadata before the numeric matrix.

`pp_augmentation` is present only for ultrasoft or PAW datasets. Its metadata is
stored as child elements, including `q_with_l`, `nqf`, `nqlc`, and PAW-specific
fields such as `shape`, `cutoff_r`, `cutoff_r_index`, `augmentation_epsilon`,
and `l_max_aug`. QE uses repeated `pp_qijl` elements when `q_with_l` is true and
repeated `pp_qij` elements when it is false.

The future `qe_pp:pseudo` form does not use the UPF-v2.0.1-only `PP_QFCOEF` and
`PP_RINNER` sections.

### pp_semilocal

`pp_semilocal` is read only when `type == "SL"`. It contains repeated `vnl`
elements. Each entry carries `l` and, when spin-orbit metadata is relevant, `j`.

### pp_pswfc

`pp_pswfc` is opened in the normal read path even when `number_of_wfc = 0`.
Entries are repeated `pp_chi` elements. Practical metadata includes `index`,
`label`, `l`, `occupation`, optional `n`, optional `pseudo_energy`, optional
`cutoff_radius`, optional `ultrasoft_cutoff_radius`, and spin-orbit metadata
`nn` and `jchi` when present.

### pp_full_wfc

`pp_full_wfc` is read and written only when `has_wfc` is true. It contains
repeated all-electron orbital entries `pp_aewfc`, optional relativistic
all-electron entries `pp_aewfc_rel`, and pseudo entries `pp_pswfc`. QE's writer
sets the section attribute `number_of_wfc` equal to `number_of_proj`.

### pp_rhoatom

`pp_rhoatom` contains atomic charge density on the current mesh. QE always reads
and writes this section in the normal future-form path.

### pp_taumod and pp_tauatom

`pp_taumod` and `pp_tauatom` are standalone sections read and written only when
`with_metagga_info` is true. They are not wrapped in a container.

### pp_paw

`pp_paw` is present only when `is_paw` is true. Practical section attributes are
`paw_data_format` and `core_energy`. Practical child sections are
`pp_occupations`, `pp_ae_nlcc`, and `pp_ae_vloc`.

All-electron and pseudo projector-linked wavefunctions are not serialized inside
`pp_paw` in this QE path; they belong to `pp_full_wfc`. Additional PAW working
arrays are derived in memory after parsing and are not part of the wire format.

### pp_gipaw

`pp_gipaw` is present only when `has_gipaw` is true. It carries
`gipaw_data_format`.

The core-orbital subtree stores `number_of_core_orbitals` as a child element and
uses repeated `pp_gipaw_core_orbital` entries.

When `paw_as_gipaw` is true, QE derives GIPAW valence orbitals and local
potentials from PAW, local-potential, and full-wavefunction data. When
`paw_as_gipaw` is false, QE reads repeated `pp_gipaw_orbital` entries and
`pp_gipaw_vlocal` with AE and PS local-potential subsections.

## Compatibility Rules

- The future form is stricter about repeated-entry index mismatches than the UPF
  v2.0.1 compatibility paths.
- `pp_info` is writer-visible but not part of the structured reader contract.
- Practical QE output namespace-qualifies only the root `qe_pp:pseudo` element;
  child elements remain unqualified.
