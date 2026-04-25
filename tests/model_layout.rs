use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn assert_item_order(source: &str, items: &[&str]) {
    let mut previous = None;

    for item in items {
        let position = source
            .find(item)
            .unwrap_or_else(|| panic!("expected to find {item}"));
        if let Some(previous) = previous {
            assert!(
                previous < position,
                "expected {item} to appear after the previous item"
            );
        }
        previous = Some(position);
    }
}

#[test]
fn model_layout_uses_non_mod_rs_and_dedicated_top_level_type_modules() {
    let root = repo_root();

    assert!(
        root.join("src/model.rs").is_file(),
        "expected src/model.rs to exist"
    );
    assert!(
        !root.join("src/model/mod.rs").exists(),
        "expected src/model/mod.rs to be removed"
    );

    for path in [
        "src/model/upf_data.rs",
        "src/model/info.rs",
        "src/model/header.rs",
        "src/model/mesh.rs",
        "src/model/semilocal.rs",
        "src/model/nonlocal.rs",
        "src/model/pseudo_wavefunctions.rs",
        "src/model/full_wfc.rs",
        "src/model/paw.rs",
        "src/model/gipaw.rs",
        "src/model/wavefunction.rs",
        "src/model/numbered.rs",
    ] {
        assert!(root.join(path).is_file(), "expected {path} to exist");
    }
}

#[test]
fn model_files_define_parent_items_before_child_items() {
    let root = repo_root();

    assert_item_order(
        &std::fs::read_to_string(root.join("src/model.rs")).unwrap(),
        &[
            "mod upf_data;",
            "mod info;",
            "mod header;",
            "pub use upf_data::*;",
            "pub use info::*;",
            "pub use header::*;",
        ],
    );
    assert_item_order(
        &std::fs::read_to_string(root.join("src/model/header.rs")).unwrap(),
        &[
            "pub struct PpHeader",
            "pub enum PseudopotentialType",
            "pub enum AtomicRelativisticFormalism",
        ],
    );
    assert_item_order(
        &std::fs::read_to_string(root.join("src/model/nonlocal.rs")).unwrap(),
        &["pub struct PpNonlocal", "pub struct PpBeta"],
    );
}
