use upf::{from_str, to_string};

const ROUNDTRIP_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="roundtrip"
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

const DYNAMIC_ROUNDTRIP_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="dynamic-roundtrip"
             element="Ne" pseudo_type="USPP" relativistic="scalar"
             is_ultrasoft="T" is_paw="F" is_coulomb="F"
             has_so="F" has_wfc="F" has_gipaw="T" core_correction="F"
             functional="PBE"
             z_valence="8.0" total_psenergy="-3.5"
             l_max="1" mesh_size="1" number_of_wfc="1" number_of_proj="1" />
  <PP_MESH mesh="1">
    <PP_R type="real" size="1">0.0</PP_R>
    <PP_RAB type="real" size="1">0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
  <PP_NONLOCAL>
    <PP_BETA.4 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.4>
    <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
      <PP_Q type="real" size="1">0.0</PP_Q>
      <PP_QIJL.1.1.0 first_index="1" second_index="1" composite_index="1" angular_momentum="0">0.2</PP_QIJL.1.1.0>
    </PP_AUGMENTATION>
  </PP_NONLOCAL>
  <PP_PSWFC>
    <PP_CHI.5 label="2P" l="1">0.5</PP_CHI.5>
  </PP_PSWFC>
  <PP_FULL_WFC>
    <PP_AEWFC.2 label="2S" l="0">0.6</PP_AEWFC.2>
    <PP_PSWFC.3 label="2P" l="1">0.7</PP_PSWFC.3>
  </PP_FULL_WFC>
  <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
  <PP_GIPAW gipaw_data_format="2">
    <PP_GIPAW_CORE_ORBITALS number_of_core_orbitals="1">
      <PP_GIPAW_CORE_ORBITAL.7 index="1" label="1S" n="1.0" l="0.0">0.8</PP_GIPAW_CORE_ORBITAL.7>
    </PP_GIPAW_CORE_ORBITALS>
  </PP_GIPAW>
</UPF>
"#;

#[test]
fn to_string_round_trips_a_document() {
    let doc = from_str(ROUNDTRIP_UPF).unwrap();
    let xml = to_string(&doc).unwrap();
    let reparsed = from_str(&xml).unwrap();

    assert_eq!(doc, reparsed);
}

#[test]
fn to_string_round_trips_dynamic_tag_sections() {
    let doc = from_str(DYNAMIC_ROUNDTRIP_UPF).unwrap();
    let xml = to_string(&doc).unwrap();
    let reparsed = from_str(&xml).unwrap();

    assert!(xml.contains("PP_BETA.4"));
    assert!(xml.contains("PP_QIJL.1.1.0"));
    assert!(xml.contains("PP_CHI.5"));
    assert!(xml.contains("PP_AEWFC.2"));
    assert!(xml.contains("PP_PSWFC.3"));
    assert!(xml.contains("PP_GIPAW_CORE_ORBITAL.7"));
    assert_eq!(doc, reparsed);
}
