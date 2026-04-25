use std::{
    fs,
    io::Cursor,
    time::{SystemTime, UNIX_EPOCH},
};

use upf::{UpfError, from_file, from_reader, from_str};

const INVALID_MESH: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
             element="He" pseudo_type="NC" relativistic="scalar"
             is_ultrasoft="F" is_paw="F" is_coulomb="F"
             has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
             functional="PBE"
             z_valence="2.0" total_psenergy="-1.25"
             wfc_cutoff="20.0" rho_cutoff="80.0"
             l_max="0" l_max_rho="0" l_local="0"
             mesh_size="3" number_of_wfc="0" number_of_proj="0" />
  <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
    <PP_R>0.0 0.1</PP_R>
    <PP_RAB>0.1 0.1 0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
  <PP_NONLOCAL />
  <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
</UPF>
"#;

#[test]
fn reader_entry_point_matches_string_entry_point() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="minimal"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R>0.0 0.1 0.2</PP_R>
        <PP_RAB>0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;
    let from_text = from_str(xml).unwrap();
    let from_reader = from_reader(Cursor::new(xml)).unwrap();
    assert_eq!(from_text, from_reader);
}

#[test]
fn file_entry_point_reads_from_disk() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="minimal"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R>0.0 0.1 0.2</PP_R>
        <PP_RAB>0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("upf-read-api-{nanos}.upf"));
    fs::write(&path, xml).unwrap();

    let doc = from_file(&path).unwrap();
    assert_eq!(doc.header.element, "He");
    fs::remove_file(path).unwrap();
}

#[test]
fn invalid_mesh_lengths_are_rejected() {
    let err = from_str(INVALID_MESH).unwrap_err();
    match err {
        UpfError::Validation(msg) => assert!(msg.contains("PP_R")),
        other => panic!("expected Validation error, got {other:?}"),
    }
}

#[test]
fn reads_metagga_and_spin_orbit_sections_when_enabled() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="extended"
                 element="He" pseudo_type="NC" relativistic="full"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="T" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="T"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R type="real" size="3" columns="3">0.0 0.1 0.2</PP_R>
        <PP_RAB type="real" size="3" columns="3">0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="3" columns="3">1.0 2.0 3.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="3" columns="3">0.2 0.3 0.4</PP_RHOATOM>
      <PP_TAUMOD type="real" size="3" columns="3">0.01 0.02 0.03</PP_TAUMOD>
      <PP_TAUATOM type="real" size="3" columns="3">0.04 0.05 0.06</PP_TAUATOM>
      <PP_SPIN_ORB>
        <PP_RELWFC.2 els="2P" nn="2" lchi="1" jchi="1.5" oc="1.0" />
        <PP_RELBETA.3 index="3" lll="1" jjj="0.5" />
      </PP_SPIN_ORB>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();

    assert!(doc.header.with_metagga_info);
    assert_eq!(doc.taumod.as_ref().unwrap(), &vec![0.01, 0.02, 0.03]);
    assert_eq!(doc.tauatom.as_ref().unwrap(), &vec![0.04, 0.05, 0.06]);
    assert_eq!(doc.spin_orb.as_ref().unwrap().relwfcs.len(), 1);
    assert_eq!(doc.spin_orb.as_ref().unwrap().relbetas.len(), 1);
    assert_eq!(
        doc.spin_orb.as_ref().unwrap().relwfcs[0].tag.to_string(),
        "PP_RELWFC.2"
    );
    assert_eq!(
        doc.spin_orb.as_ref().unwrap().relbetas[0].tag.to_string(),
        "PP_RELBETA.3"
    );
}

#[test]
fn missing_nonlocal_section_is_allowed_when_projector_count_is_zero() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="no-nonlocal"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R>0.0 0.1 0.2</PP_R>
        <PP_RAB>0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();
    assert!(doc.nonlocal.betas.is_empty());
}
