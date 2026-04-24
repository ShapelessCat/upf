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
fn parses_nonlocal_and_augmentation_sections() {
    let doc = from_file(example("H.pbe-rrkjus_psl.1.0.0.UPF")).unwrap();

    assert_eq!(doc.nonlocal.betas.len(), 2);
    assert_eq!(doc.nonlocal.dij.values.len(), 4);
    assert!(doc.nonlocal.augmentation.is_some());
    assert_eq!(doc.pswfc.as_ref().unwrap().orbitals.len(), 1);
}
