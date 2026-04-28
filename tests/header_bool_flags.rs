use upf::{from_str, to_string};

const HEADER_BOOL_FLAGS_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER
    generated="unit"
    author="tester"
    date="2026-04-03"
    comment="bool-flags"
    element="He"
    pseudo_type="NC"
    relativistic="scalar"
    is_ultrasoft="T"
    is_paw="F"
    is_coulomb="F"
    has_so="F"
    has_wfc="T"
    has_gipaw="F"
    paw_as_gipaw="F"
    core_correction="F"
    functional="PBE"
    z_valence="2.0"
    total_psenergy="-1.25"
    wfc_cutoff="20.0"
    rho_cutoff="80.0"
    l_max="0"
    l_max_rho="0"
    l_local="0"
    mesh_size="3"
    number_of_wfc="0"
    number_of_proj="1" />
  <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
    <PP_R>0.0 0.1 0.2</PP_R>
    <PP_RAB>0.1 0.1 0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
  <PP_NONLOCAL>
    <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1 0.2 0.3</PP_BETA.1>
    <PP_DIJ>0.0</PP_DIJ>
    <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
      <PP_Q type="real" size="1">0.0</PP_Q>
    </PP_AUGMENTATION>
  </PP_NONLOCAL>
  <PP_FULL_WFC>
    <PP_AEWFC.1 index="1" label="1S" l="0">0.3 0.2 0.1</PP_AEWFC.1>
    <PP_PSWFC.1 index="1" label="1S" l="0">0.1 0.2 0.3</PP_PSWFC.1>
  </PP_FULL_WFC>
  <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
</UPF>
"#;

#[test]
fn header_bool_flags_round_trip_as_upf_booleans() {
    let doc = from_str(HEADER_BOOL_FLAGS_UPF).unwrap();
    assert!(doc.header.is_ultrasoft);
    assert!(!doc.header.is_paw);
    assert!(doc.header.has_wfc);
    assert!(!doc.header.paw_as_gipaw);

    let xml = to_string(&doc).unwrap();

    assert!(xml.contains(r#"is_ultrasoft="T""#));
    assert!(xml.contains(r#"has_wfc="T""#));
    assert!(xml.contains(r#"paw_as_gipaw="F""#));
    assert!(!xml.contains(r#"is_ultrasoft=".T.""#));
    assert!(!xml.contains(r#"paw_as_gipaw=".F.""#));
}
