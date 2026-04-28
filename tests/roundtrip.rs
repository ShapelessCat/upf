use upf::{from_str, to_string};

const ROUNDTRIP_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="roundtrip"
             element="He" pseudo_type="NC" relativistic="no"
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
    <PP_DIJ type="real" size="1">0.0</PP_DIJ>
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
      <PP_GIPAW_CORE_ORBITAL.1 index="1" label="1S" n="1.0" l="0.0">0.8</PP_GIPAW_CORE_ORBITAL.1>
    </PP_GIPAW_CORE_ORBITALS>
    <PP_GIPAW_ORBITALS number_of_valence_orbitals="1">
      <PP_GIPAW_ORBITAL.1 index="1" label="2P" l="1" cutoff_radius="1.2">
        <PP_GIPAW_WFS_AE type="real" size="1">0.9</PP_GIPAW_WFS_AE>
        <PP_GIPAW_WFS_PS type="real" size="1">1.0</PP_GIPAW_WFS_PS>
      </PP_GIPAW_ORBITAL.1>
    </PP_GIPAW_ORBITALS>
    <PP_GIPAW_VLOCAL>
      <PP_GIPAW_VLOCAL_AE type="real" size="1">1.1</PP_GIPAW_VLOCAL_AE>
      <PP_GIPAW_VLOCAL_PS type="real" size="1">1.2</PP_GIPAW_VLOCAL_PS>
    </PP_GIPAW_VLOCAL>
  </PP_GIPAW>
</UPF>
"#;

const EXTENDED_ROUNDTRIP_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="extended-roundtrip"
             element="At" pseudo_type="NC" relativistic="full"
             is_ultrasoft="F" is_paw="F" is_coulomb="F"
             has_so="T" has_wfc="F" has_gipaw="T" paw_as_gipaw="F"
             core_correction="F" with_metagga_info="T"
             functional="PBE"
             z_valence="7.0" total_psenergy="-3.5"
             l_max="1" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
  <PP_MESH mesh="1">
    <PP_R type="real" size="1">0.0</PP_R>
    <PP_RAB type="real" size="1">0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
  <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
  <PP_TAUMOD type="real" size="1">0.3</PP_TAUMOD>
  <PP_TAUATOM type="real" size="1">0.4</PP_TAUATOM>
  <PP_SPIN_ORB>
    <PP_RELWFC.2 els="2P" nn="2" lchi="1" jchi="1.5" oc="1.0" />
    <PP_RELBETA.3 index="3" lll="1" jjj="0.5" />
  </PP_SPIN_ORB>
  <PP_GIPAW gipaw_data_format="2">
    <PP_GIPAW_CORE_ORBITALS number_of_core_orbitals="1">
      <PP_GIPAW_CORE_ORBITAL.1 index="1" label="1S" n="1.0" l="0.0">0.8</PP_GIPAW_CORE_ORBITAL.1>
    </PP_GIPAW_CORE_ORBITALS>
    <PP_GIPAW_ORBITALS number_of_valence_orbitals="1">
      <PP_GIPAW_ORBITAL.1 index="1" label="2P" l="1" cutoff_radius="1.2" ultrasoft_cutoff_radius="1.4">
        <PP_GIPAW_WFS_AE type="real" size="1">0.5</PP_GIPAW_WFS_AE>
        <PP_GIPAW_WFS_PS type="real" size="1">0.6</PP_GIPAW_WFS_PS>
      </PP_GIPAW_ORBITAL.1>
    </PP_GIPAW_ORBITALS>
    <PP_GIPAW_VLOCAL>
      <PP_GIPAW_VLOCAL_AE type="real" size="1">0.7</PP_GIPAW_VLOCAL_AE>
      <PP_GIPAW_VLOCAL_PS type="real" size="1">0.8</PP_GIPAW_VLOCAL_PS>
    </PP_GIPAW_VLOCAL>
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
    assert!(xml.contains("PP_GIPAW_CORE_ORBITAL.1"));
    assert!(xml.contains("PP_GIPAW_ORBITAL.1"));
    assert!(xml.contains("PP_GIPAW_VLOCAL_AE"));
    assert_eq!(doc, reparsed);
}

#[test]
fn to_string_round_trips_extended_sections() {
    let doc = from_str(EXTENDED_ROUNDTRIP_UPF).unwrap();
    let xml = to_string(&doc).unwrap();
    let reparsed = from_str(&xml).unwrap();

    assert!(xml.contains("PP_TAUMOD"));
    assert!(xml.contains("PP_TAUATOM"));
    assert!(xml.contains("PP_SPIN_ORB"));
    assert!(xml.contains("PP_RELWFC.2"));
    assert!(xml.contains("PP_RELBETA.3"));
    assert!(xml.contains("PP_GIPAW_ORBITAL.1"));
    assert!(xml.contains("PP_GIPAW_VLOCAL_AE"));
    assert_eq!(doc, reparsed);
}

#[test]
fn relativistic_no_round_trips_as_no() {
    let doc = from_str(ROUNDTRIP_UPF).unwrap();
    let xml = to_string(&doc).unwrap();

    assert!(xml.contains(r#"relativistic="no""#));
    assert!(!xml.contains("nonrelativistic"));
}

const SPIN_ORBIT_WFC_ROUNDTRIP_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="so-wfc"
             element="Bi" pseudo_type="PAW" relativistic="full"
             is_ultrasoft="F" is_paw="T" is_coulomb="F"
             has_so="T" has_wfc="T" has_gipaw="F" core_correction="F"
             functional="PBE"
             z_valence="5.0" total_psenergy="-2.0"
             l_max="1" mesh_size="1" number_of_wfc="1" number_of_proj="1" />
  <PP_MESH mesh="1">
    <PP_R type="real" size="1">0.0</PP_R>
    <PP_RAB type="real" size="1">0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
  <PP_NONLOCAL>
    <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
    <PP_DIJ type="real" size="1">0.0</PP_DIJ>
    <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
      <PP_Q type="real" size="1">0.0</PP_Q>
      <PP_QIJL.1.1.0 first_index="1" second_index="1" composite_index="1" angular_momentum="0">0.2</PP_QIJL.1.1.0>
    </PP_AUGMENTATION>
  </PP_NONLOCAL>
  <PP_PSWFC>
    <PP_CHI.1 label="6P" l="1" nn="6" jchi="1.5" occupation="3.0">0.5</PP_CHI.1>
  </PP_PSWFC>
  <PP_FULL_WFC number_of_wfc="1">
    <PP_AEWFC.1 label="6P" l="1">0.6</PP_AEWFC.1>
    <PP_PSWFC.1 label="6P" l="1">0.7</PP_PSWFC.1>
    <PP_AEWFC_REL.1 label="6P" l="1" nn="6" jchi="1.5">0.8</PP_AEWFC_REL.1>
  </PP_FULL_WFC>
  <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
  <PP_SPIN_ORB>
    <PP_RELWFC.1 els="6P" nn="6" lchi="1" jchi="1.5" oc="3.0" />
    <PP_RELBETA.1 index="1" lll="1" jjj="0.5" />
  </PP_SPIN_ORB>
  <PP_PAW paw_data_format="2">
    <PP_OCCUPATIONS type="real" size="1">2.0</PP_OCCUPATIONS>
    <PP_AE_NLCC type="real" size="1">0.1</PP_AE_NLCC>
    <PP_AE_VLOC type="real" size="1">0.2</PP_AE_VLOC>
  </PP_PAW>
</UPF>
"#;

#[test]
fn to_string_round_trips_spin_orbit_wavefunction_attributes() {
    let doc = from_str(SPIN_ORBIT_WFC_ROUNDTRIP_UPF).unwrap();
    let xml = to_string(&doc).unwrap();
    let reparsed = from_str(&xml).unwrap();

    assert!(xml.contains(r#"nn="6""#));
    assert!(xml.contains(r#"jchi="1.5""#));
    assert!(xml.contains("PP_AEWFC_REL.1"));
    assert_eq!(doc, reparsed);
}

const QIJ_ROUNDTRIP_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="qij-roundtrip"
             element="Na" pseudo_type="USPP" relativistic="scalar"
             is_ultrasoft="T" is_paw="F" is_coulomb="F"
             has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
             functional="PBE"
             z_valence="1.0" total_psenergy="-1.0"
             l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="1" />
  <PP_MESH mesh="1">
    <PP_R type="real" size="1">0.0</PP_R>
    <PP_RAB type="real" size="1">0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
  <PP_NONLOCAL>
    <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
    <PP_DIJ type="real" size="1">0.0</PP_DIJ>
    <PP_AUGMENTATION q_with_l="F" nqf="0" nqlc="1">
      <PP_Q type="real" size="1">0.0</PP_Q>
      <PP_QIJ.1.1 first_index="1" second_index="1" composite_index="1">0.3</PP_QIJ.1.1>
    </PP_AUGMENTATION>
  </PP_NONLOCAL>
  <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
</UPF>
"#;

#[test]
fn to_string_round_trips_qij_augmentation() {
    let doc = from_str(QIJ_ROUNDTRIP_UPF).unwrap();
    let xml = to_string(&doc).unwrap();
    let reparsed = from_str(&xml).unwrap();

    assert!(xml.contains("PP_QIJ.1.1"));
    assert!(!xml.contains("PP_QIJL"));
    assert_eq!(doc, reparsed);
}
