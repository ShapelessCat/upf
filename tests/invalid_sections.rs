use upf::from_str;

#[test]
fn paw_header_requires_paw_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="Si" pseudo_type="PAW" relativistic="scalar"
                 is_ultrasoft="F" is_paw="T" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 z_valence="4.0" total_psenergy="-5.0"
                 wfc_cutoff="30.0" rho_cutoff="120.0"
                 l_max="1" l_max_rho="2" l_local="1"
                 mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="1" xmin="0.0" rmax="0.0" zmesh="1.0">
        <PP_R>0.0</PP_R>
        <PP_RAB>0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM>0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_PAW"));
}

#[test]
fn gipaw_header_requires_gipaw_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="Si" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="T" core_correction="F"
                 z_valence="4.0" total_psenergy="-5.0"
                 wfc_cutoff="30.0" rho_cutoff="120.0"
                 l_max="1" l_max_rho="2" l_local="1"
                 mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="1" xmin="0.0" rmax="0.0" zmesh="1.0">
        <PP_R>0.0</PP_R>
        <PP_RAB>0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM>0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_GIPAW"));
}
