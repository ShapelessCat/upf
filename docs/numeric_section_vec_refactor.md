# NumericSection Vec Refactor

This note records the size rules used after replacing the public
`NumericSection` / `Option<NumericSection>` fields with raw `Vec<f64>` /
`Option<Vec<f64>>`.

For these fields, XML attributes `type`, `columns`, and `size` are ignored on
read. Serialization computes and writes `size` from the vector length and does
not emit `type` or `columns`.

The formulas below are backed by Quantum ESPRESSO's UPF reader and internal
array definitions in the local QE checkout:

- `upflib/pseudo_types.f90`
- `upflib/read_upf_new.f90`
- `upflib/write_upf_new.f90`
- `upflib/read_upf_v1.f90`
- `upflib/read_uspp.f90`
- `upflib/upf_to_internal.f90`

## Effective `nqlc`

QE treats augmentation `nqlc = 0` as a shorthand for `2 * l_max + 1`.

`effective_nqlc = augmentation.nqlc` unless `augmentation.nqlc == 0`, in which
case `effective_nqlc = 2 * header.l_max + 1`.

## Size Rules

| Field | Expected size |
| --- | --- |
| `PP_R` | `header.mesh_size` |
| `PP_RAB` | `header.mesh_size` |
| `PP_NLCC` | `header.mesh_size` |
| `PP_LOCAL` | `header.mesh_size` whenever present |
| `PP_RHOATOM` | `header.mesh_size` |
| `PP_TAUMOD` | `header.mesh_size` |
| `PP_TAUATOM` | `header.mesh_size` |
| `PP_OCCUPATIONS` | `header.number_of_proj` |
| `PP_AE_NLCC` | `header.mesh_size` |
| `PP_AE_VLOC` | `header.mesh_size` |
| `PP_Q` | `header.number_of_proj * header.number_of_proj` |
| `PP_MULTIPOLES` | `header.number_of_proj * header.number_of_proj * (2 * header.l_max + 1)` |
| `PP_QFCOEF` | `augmentation.nqf * effective_nqlc * header.number_of_proj * header.number_of_proj` |
| `PP_RINNER` | `effective_nqlc` |
| `PP_GIPAW_WFS_AE` | `header.mesh_size` |
| `PP_GIPAW_WFS_PS` | `header.mesh_size` |
| `PP_GIPAW_VLOCAL_AE` | `header.mesh_size` |
| `PP_GIPAW_VLOCAL_PS` | `header.mesh_size` |

## QE Shape References

- `PP_Q` maps to QE `qqq(nbeta, nbeta)` in `read_upf_new.f90`.
- `PP_MULTIPOLES` maps to QE `augmom(nbeta, nbeta, 0:2*lmax)` in
  `read_upf_new.f90`.
- `PP_RINNER` maps to QE `rinner(nqlc)` in `read_upf_new.f90`.
- `PP_QFCOEF` maps to QE `qfcoef(nqf, nqlc, nbeta, nbeta)` in both
  `pseudo_types.f90` and `read_upf_new.f90`.

## Structured Numeric Carriers

The same rule now applies to structured numeric sections that still keep real
semantic attributes:

- `PP_BETA.n`: keep `index`, `label`, `angular_momentum`, cutoff metadata, and
  `values`
- `PP_VNL.n`: keep `l`, optional `j`, and `values`
- `PP_CHI.n`, `PP_AEWFC.n`, `PP_PSWFC.n`, `PP_AEWFC_REL.n`: keep orbital
  metadata and `values`
- `PP_QIJ.i.j`, `PP_QIJL.i.j.l`: keep projector-index metadata and `values`
- `PP_GIPAW_CORE_ORBITAL.n`: keep `index`, `label`, `n`, `l`, and `values`

For these sections, read-time `type`, `size`, and `columns` are ignored.
`UpfData::validate()` checks the payload length against `header.mesh_size`.

`PP_DIJ` is now treated as a plain `Vec<f64>` and validated against
`header.number_of_proj * header.number_of_proj`.
