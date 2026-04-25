use std::path::PathBuf;

use upf::from_file;

fn example(name: &str) -> PathBuf {
    for subdir in ["UPF_1.x", "UPF_2.x"] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/SSSP_1.3.0_PBE_efficiency")
            .join(subdir)
            .join(name);
        if path.exists() {
            return path;
        }
    }

    panic!("missing example fixture: {name}");
}

#[test]
fn parses_paw_and_gipaw_subtrees_from_real_examples() {
    let doc = from_file(example("Pu.paw.z_16.ld1.uni-marburg.v0.upf")).unwrap();

    assert_eq!(doc.full_wfc.as_ref().unwrap().entries.len(), 16);
    assert_eq!(doc.paw.as_ref().unwrap().data_format, "2");
    assert_eq!(doc.gipaw.as_ref().unwrap().data_format, "2");
    assert_eq!(
        doc.gipaw
            .as_ref()
            .unwrap()
            .core_orbitals
            .as_ref()
            .unwrap()
            .orbitals
            .len(),
        13
    );
}

#[test]
fn parses_explicit_gipaw_orbitals_and_vlocal_from_real_examples() {
    let doc = from_file(example("At.us.z_17.ld1.psl.v1.0.0-high.upf")).unwrap();
    let gipaw = doc.gipaw.as_ref().unwrap();

    assert!(gipaw.orbitals.is_some());
    assert!(gipaw.vlocal.is_some());
    assert_eq!(
        gipaw.orbitals.as_ref().unwrap().number_of_valence_orbitals,
        gipaw.orbitals.as_ref().unwrap().orbitals.len()
    );
    assert!(!gipaw.orbitals.as_ref().unwrap().orbitals.is_empty());
    assert!(!gipaw.vlocal.as_ref().unwrap().ae.values.is_empty());
    assert!(!gipaw.vlocal.as_ref().unwrap().ps.values.is_empty());
}
