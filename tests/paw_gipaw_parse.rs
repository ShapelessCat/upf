use upf::from_str;

const PAW_GIPAW_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="paw-gipaw"
             element="Si" pseudo_type="PAW" relativistic="scalar"
             is_ultrasoft="F" is_paw="T" is_coulomb="F"
             has_so="F" has_wfc="T" has_gipaw="T" core_correction="F"
             z_valence="4.0" total_psenergy="-5.0"
             wfc_cutoff="30.0" rho_cutoff="120.0"
             l_max="1" l_max_rho="2" l_local="1"
             mesh_size="3" number_of_wfc="1" number_of_proj="1" />
  <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
    <PP_R>0.0 0.1 0.2</PP_R>
    <PP_RAB>0.1 0.1 0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
  <PP_NONLOCAL />
  <PP_FULL_WFC>
    <PP_AEWFC.1 label="3S" l="0">0.1 0.2 0.3</PP_AEWFC.1>
    <PP_PSWFC.1 label="3S" l="0">0.1 0.2 0.3</PP_PSWFC.1>
  </PP_FULL_WFC>
  <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
  <PP_PAW>
    <PP_PAW_FORMAT_VERSION>1.0</PP_PAW_FORMAT_VERSION>
    <PP_AUGMENTATION q_with_l="F" augmentation_epsilon="1e-12" cutoff_r="1.5" l_max_aug="2">
      <PP_QIJL.1.1.0>0.1 0.2 0.3</PP_QIJL.1.1.0>
    </PP_AUGMENTATION>
    <PP_AE_RHO_ATC>0.5 0.6 0.7</PP_AE_RHO_ATC>
    <PP_AEWFC.1 label="3S" l="0">0.1 0.2 0.3</PP_AEWFC.1>
    <PP_PSWFC_FULL.1 label="3S" l="0">0.1 0.2 0.3</PP_PSWFC_FULL.1>
    <PP_AEVLOC>1.0 1.1 1.2</PP_AEVLOC>
    <PP_KDIFF>0.0 0.0 0.0</PP_KDIFF>
    <PP_OCCUP>2.0</PP_OCCUP>
    <PP_GRID_RECON>0.0 0.1 0.2</PP_GRID_RECON>
  </PP_PAW>
  <PP_GIPAW>
    <PP_GIPAW_FORMAT_VERSION>1.0</PP_GIPAW_FORMAT_VERSION>
    <GIPAW_LOCAL_DATA>
      <GIPAW_VLOCAL_AE>1.0 1.1 1.2</GIPAW_VLOCAL_AE>
      <GIPAW_VLOCAL_PS>0.8 0.9 1.0</GIPAW_VLOCAL_PS>
    </GIPAW_LOCAL_DATA>
  </PP_GIPAW>
</UPF>
"#;

#[test]
fn parses_paw_and_gipaw_subtrees() {
    let doc = from_str(PAW_GIPAW_UPF).unwrap();

    assert_eq!(doc.full_wfc.as_ref().unwrap().entries.len(), 2);
    assert_eq!(doc.paw.as_ref().unwrap().format_version, "1.0");
    assert_eq!(doc.gipaw.as_ref().unwrap().format_version, "1.0");
}
