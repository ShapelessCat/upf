use upf::{from_str, model::PseudopotentialType};

#[test]
fn header_accepts_true_false_aliases() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="Pu" pseudo_type="PAW" relativistic="scalar"
                 is_ultrasoft="TRUE" is_paw="true" is_coulomb="false"
                 has_so="false" has_wfc="true" has_gipaw="true" paw_as_gipaw="true"
                 core_correction="true" functional="PBE"
                 z_valence="16.0" total_psenergy="-1.0" wfc_cutoff="30.0" rho_cutoff="120.0"
                 l_max="3" l_max_rho="6" l_local="-2" mesh_size="1" number_of_wfc="0" number_of_proj="1"/>
      <PP_MESH mesh="1"><PP_R type="real" size="1">0.0</PP_R><PP_RAB type="real" size="1">0.1</PP_RAB></PP_MESH>
      <PP_NLCC type="real" size="1">0.0</PP_NLCC>
      <PP_LOCAL type="real" size="1">0.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l="true" nqf="0" nqlc="0">
          <PP_Q type="real" size="1">0.0</PP_Q>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_FULL_WFC />
      <PP_RHOATOM type="real" size="1">0.0</PP_RHOATOM>
      <PP_PAW paw_data_format="2">
        <PP_OCCUPATIONS type="real" size="1">0.0</PP_OCCUPATIONS>
        <PP_AE_NLCC type="real" size="1">0.0</PP_AE_NLCC>
        <PP_AE_VLOC type="real" size="1">0.0</PP_AE_VLOC>
      </PP_PAW>
      <PP_GIPAW gipaw_data_format="2">
        <PP_GIPAW_CORE_ORBITALS number_of_core_orbitals="0"></PP_GIPAW_CORE_ORBITALS>
      </PP_GIPAW>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();
    assert!(doc.header.is_paw);
    assert!(doc.header.has_gipaw);
    assert_eq!(doc.header.total_psenergy, Some(-1.0));
}

#[test]
fn header_accepts_dotted_logical_aliases() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="As" pseudo_type="USPP" relativistic="scalar"
                 is_ultrasoft=".TRUE." is_paw=".FALSE." is_coulomb=".FALSE."
                 has_so=".FALSE." has_wfc=".FALSE." has_gipaw=".FALSE." core_correction=".FALSE."
                 functional="PBE" z_valence="5.0" total_psenergy="-1.0"
                 l_max="2" mesh_size="1" number_of_wfc="0" number_of_proj="1"/>
      <PP_MESH mesh="1"><PP_R type="real" size="1">0.0</PP_R><PP_RAB type="real" size="1">0.1</PP_RAB></PP_MESH>
      <PP_LOCAL type="real" size="1">0.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l=".FALSE." nqf="0" nqlc="0">
          <PP_Q type="real" size="1">0.0</PP_Q>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.0</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();
    assert!(doc.header.is_ultrasoft);
    assert!(!doc.header.is_paw);
    assert!(!doc.header.core_correction);
}

#[test]
fn header_accepts_uspp_alias_when_present() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="As" pseudo_type="USPP" relativistic="scalar"
                 is_ultrasoft="T" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE" z_valence="5.0" total_psenergy="-1.0"
                 l_max="2" mesh_size="1" number_of_wfc="0" number_of_proj="1"/>
      <PP_MESH mesh="1"><PP_R type="real" size="1">0.0</PP_R><PP_RAB type="real" size="1">0.1</PP_RAB></PP_MESH>
      <PP_LOCAL type="real" size="1">0.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="0">
          <PP_Q type="real" size="1">0.0</PP_Q>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.0</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();
    assert_eq!(doc.header.pseudo_type, PseudopotentialType::Ultrasoft);
    assert_eq!(doc.header.total_psenergy, Some(-1.0));
    assert_eq!(doc.header.wfc_cutoff, None);
    assert_eq!(doc.header.rho_cutoff, None);
    assert_eq!(doc.header.l_max_rho, None);
    assert_eq!(doc.header.l_local, None);
}
