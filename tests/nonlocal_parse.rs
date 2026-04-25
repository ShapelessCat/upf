use std::path::PathBuf;

use upf::{from_file, from_str, to_string};

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
    assert_eq!(doc.nonlocal.dij.len(), 4);
    assert!(doc.nonlocal.augmentation.is_some());
    assert_eq!(doc.pswfc.as_ref().unwrap().orbitals.len(), 1);
}

#[test]
fn parses_numbered_semilocal_channels() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="semilocal"
                 element="Si" pseudo_type="SL" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="4.0" total_psenergy="-5.0"
                 wfc_cutoff="30.0" rho_cutoff="120.0"
                 l_max="1" l_max_rho="2" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="3">
        <PP_R type="real" size="3">0.0 0.1 0.2</PP_R>
        <PP_RAB type="real" size="3">0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="3">1.0 2.0 3.0</PP_LOCAL>
      <PP_SEMILOCAL>
        <PP_VNL.1 type="real" size="3" columns="3" l="0">0.1 0.2 0.3</PP_VNL.1>
        <PP_VNL.2 type="real" size="3" columns="3" l="1">0.4 0.5 0.6</PP_VNL.2>
      </PP_SEMILOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="3">0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();
    let semilocal = doc.semilocal.as_ref().unwrap();

    assert_eq!(semilocal.channels.len(), 2);
    assert_eq!(semilocal.channels[0].tag.to_string(), "PP_VNL.1");
    assert_eq!(semilocal.channels[0].value.l, 0);
    assert_eq!(semilocal.channels[1].tag.to_string(), "PP_VNL.2");
    assert_eq!(semilocal.channels[1].value.values, vec![0.4, 0.5, 0.6]);
}

#[test]
fn zero_projector_documents_round_trip_without_nonlocal_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="no-projectors"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 l_max="0" mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="3">
        <PP_R type="real" size="3">0.0 0.1 0.2</PP_R>
        <PP_RAB type="real" size="3">0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="3">1.0 2.0 3.0</PP_LOCAL>
      <PP_RHOATOM type="real" size="3">0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();
    let written = to_string(&doc).unwrap();

    assert!(doc.nonlocal.betas.is_empty());
    assert!(!written.contains("PP_NONLOCAL"));
}
