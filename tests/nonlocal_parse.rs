use upf::from_str;

const NONLOCAL_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_INFO>Generated for test<PP_INPUTFILE>&amp;input</PP_INPUTFILE></PP_INFO>
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="nonlocal"
             element="O" pseudo_type="US" relativistic="scalar"
             is_ultrasoft="T" is_paw="F" is_coulomb="F"
             has_so="F" has_wfc="T" has_gipaw="F" core_correction="T"
             z_valence="6.0" total_psenergy="-10.0"
             wfc_cutoff="40.0" rho_cutoff="160.0"
             l_max="1" l_max_rho="2" l_local="1"
             mesh_size="3" number_of_wfc="1" number_of_proj="1" />
  <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
    <PP_R>0.0 0.1 0.2</PP_R>
    <PP_RAB>0.1 0.1 0.1</PP_RAB>
  </PP_MESH>
  <PP_NLCC>0.3 0.2 0.1</PP_NLCC>
  <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
  <PP_SEMILOCAL>
    <PP_VNL1 L="1">0.9 0.8 0.7</PP_VNL1>
  </PP_SEMILOCAL>
  <PP_NONLOCAL>
    <PP_BETA.1 index="1" angular_momentum="1" cutoff_radius="1.2" ultrasoft_cutoff_radius="1.4">
      0.1 0.2 0.3
    </PP_BETA.1>
    <PP_DIJ>1.0</PP_DIJ>
  </PP_NONLOCAL>
  <PP_PSWFC>
    <PP_CHI.1 label="2P" l="1" occupation="4.0">0.4 0.5 0.6</PP_CHI.1>
  </PP_PSWFC>
  <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
</UPF>
"#;

#[test]
fn parses_nonlocal_and_wavefunction_sections() {
    let doc = from_str(NONLOCAL_UPF).unwrap();

    assert_eq!(doc.info.as_ref().unwrap().body_text.trim(), "Generated for test");
    assert_eq!(doc.nlcc.as_ref().unwrap().values, vec![0.3, 0.2, 0.1]);
    assert_eq!(doc.semilocal.as_ref().unwrap().channels.len(), 1);
    assert_eq!(doc.nonlocal.betas.len(), 1);
    assert_eq!(doc.nonlocal.dij.values, vec![1.0]);
    assert_eq!(doc.pswfc.as_ref().unwrap().orbitals.len(), 1);
}
