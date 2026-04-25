use upf::from_str;

#[test]
fn paw_header_requires_paw_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="Si" pseudo_type="PAW" relativistic="scalar"
                 is_ultrasoft="F" is_paw="T" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
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
                 functional="PBE"
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

#[test]
fn declared_projector_count_must_match_beta_entries() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="H" pseudo_type="USPP" relativistic="scalar"
                 is_ultrasoft="T" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="1.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="2" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
          <PP_Q type="real" size="1">0.0</PP_Q>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("projectors"));
}

#[test]
fn declared_wavefunction_count_must_match_pswfc_entries() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="2" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_PSWFC>
        <PP_CHI.1 label="1S" l="0">0.5</PP_CHI.1>
      </PP_PSWFC>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("wavefunctions"));
}

#[test]
fn numeric_section_size_must_match_payload_length() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="2">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_R"));
    assert!(err.to_string().contains("declares size"));
}

#[test]
fn core_correction_requires_nlcc_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="T"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_NLCC"));
}

#[test]
fn full_wfc_flag_requires_full_wfc_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="T" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_FULL_WFC"));
}

#[test]
fn metagga_flag_requires_taumod_and_tauatom_sections() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="T"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
      <PP_TAUMOD type="real" size="1">0.3</PP_TAUMOD>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_TAUMOD"));
    assert!(err.to_string().contains("PP_TAUATOM"));
}

#[test]
fn spin_orbit_flag_requires_spin_orb_section() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="He" pseudo_type="NC" relativistic="full"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="T" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.0"
                 l_max="0" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_SPIN_ORB"));
}

#[test]
fn semilocal_channel_size_must_match_payload_length() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="Si" pseudo_type="SL" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="4.0" total_psenergy="-5.0"
                 l_max="1" mesh_size="1" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_SEMILOCAL>
        <PP_VNL.1 type="real" size="2" l="0">0.1</PP_VNL.1>
      </PP_SEMILOCAL>
      <PP_NONLOCAL />
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_VNL.1"));
    assert!(err.to_string().contains("declares size"));
}

#[test]
fn projector_size_must_match_payload_length() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="H" pseudo_type="USPP" relativistic="scalar"
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
        <PP_BETA.1 type="real" size="2" index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
          <PP_Q type="real" size="1">0.0</PP_Q>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_BETA.1"));
    assert!(err.to_string().contains("declares size"));
}

#[test]
fn full_wfc_declared_number_of_wfc_must_match_entry_families() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="O" pseudo_type="PAW" relativistic="scalar"
                 is_ultrasoft="F" is_paw="T" is_coulomb="F"
                 has_so="F" has_wfc="T" has_gipaw="F" paw_as_gipaw="F"
                 core_correction="F" with_metagga_info="F"
                 functional="PBE"
                 z_valence="6.0" total_psenergy="-1.0"
                 l_max="1" mesh_size="1" number_of_wfc="1" number_of_proj="1" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
          <PP_Q type="real" size="1">0.0</PP_Q>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_PSWFC>
        <PP_CHI.1 label="2S" l="0">0.5</PP_CHI.1>
      </PP_PSWFC>
      <PP_FULL_WFC number_of_wfc="2">
        <PP_AEWFC.1 index="1" label="2S" l="0">0.6</PP_AEWFC.1>
        <PP_PSWFC.1 index="1" label="2S" l="0">0.7</PP_PSWFC.1>
      </PP_FULL_WFC>
      <PP_PAW paw_data_format="2">
        <PP_OCCUPATIONS type="real" size="1">2.0</PP_OCCUPATIONS>
      </PP_PAW>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_FULL_WFC"));
    assert!(err.to_string().contains("number_of_wfc"));
}

#[test]
fn spin_orb_relwfc_count_must_match_number_of_wfc() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="Bi" pseudo_type="NC" relativistic="full"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="T" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="5.0" total_psenergy="-2.0"
                 l_max="1" mesh_size="1" number_of_wfc="2" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_PSWFC>
        <PP_CHI.1 label="6S" l="0">0.5</PP_CHI.1>
        <PP_CHI.2 label="6P" l="1">0.6</PP_CHI.2>
      </PP_PSWFC>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
      <PP_SPIN_ORB>
        <PP_RELWFC.1 els="6S" nn="6" lchi="0" jchi="0.5" oc="2.0" />
      </PP_SPIN_ORB>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_RELWFC"));
}

#[test]
fn spin_orb_relbeta_count_must_match_number_of_proj() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="Bi" pseudo_type="NC" relativistic="full"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="T" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="5.0" total_psenergy="-2.0"
                 l_max="1" mesh_size="1" number_of_wfc="0" number_of_proj="2" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_BETA.2 index="2" angular_momentum="1" cutoff_radius="1.0">0.2</PP_BETA.2>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
      <PP_SPIN_ORB>
        <PP_RELBETA.1 index="1" lll="0" jjj="0.5" />
      </PP_SPIN_ORB>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_RELBETA"));
}

#[test]
fn paw_as_gipaw_rejects_gipaw_orbitals() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="O" pseudo_type="PAW" relativistic="scalar"
                 is_ultrasoft="F" is_paw="T" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="T" paw_as_gipaw="T"
                 core_correction="F"
                 functional="PBE"
                 z_valence="6.0" total_psenergy="-1.0"
                 l_max="1" mesh_size="1" number_of_wfc="0" number_of_proj="1" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1</PP_BETA.1>
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
          <PP_Q type="real" size="1">0.0</PP_Q>
          <PP_QIJL.1.1.0 first_index="1" second_index="1" composite_index="1" angular_momentum="0">0.2</PP_QIJL.1.1.0>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
      <PP_PAW paw_data_format="2">
        <PP_OCCUPATIONS type="real" size="1">2.0</PP_OCCUPATIONS>
      </PP_PAW>
      <PP_GIPAW gipaw_data_format="2">
        <PP_GIPAW_CORE_ORBITALS number_of_core_orbitals="1">
          <PP_GIPAW_CORE_ORBITAL.1 index="1" label="1S" n="1.0" l="0.0">0.8</PP_GIPAW_CORE_ORBITAL.1>
        </PP_GIPAW_CORE_ORBITALS>
        <PP_GIPAW_ORBITALS number_of_valence_orbitals="1">
          <PP_GIPAW_ORBITAL.1 index="1" label="2P" l="1" cutoff_radius="1.2">
            <PP_GIPAW_WFS_AE type="real" size="1">0.5</PP_GIPAW_WFS_AE>
            <PP_GIPAW_WFS_PS type="real" size="1">0.6</PP_GIPAW_WFS_PS>
          </PP_GIPAW_ORBITAL.1>
        </PP_GIPAW_ORBITALS>
      </PP_GIPAW>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("paw_as_gipaw"));
    assert!(err.to_string().contains("PP_GIPAW_ORBITALS"));
}

#[test]
fn aewfc_rel_requires_has_so_and_is_paw() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
                 element="O" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="T" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="6.0" total_psenergy="-1.0"
                 l_max="1" mesh_size="1" number_of_wfc="1" number_of_proj="0" />
      <PP_MESH mesh="1">
        <PP_R type="real" size="1">0.0</PP_R>
        <PP_RAB type="real" size="1">0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="1">1.0</PP_LOCAL>
      <PP_PSWFC>
        <PP_CHI.1 label="2S" l="0">0.5</PP_CHI.1>
      </PP_PSWFC>
      <PP_FULL_WFC number_of_wfc="1">
        <PP_AEWFC.1 label="2S" l="0">0.6</PP_AEWFC.1>
        <PP_PSWFC.1 label="2S" l="0">0.7</PP_PSWFC.1>
        <PP_AEWFC_REL.1 label="2S" l="0" nn="2" jchi="0.5">0.8</PP_AEWFC_REL.1>
      </PP_FULL_WFC>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("PP_AEWFC_REL"));
}

#[test]
fn q_with_l_true_rejects_qij_naming() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
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
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1">
          <PP_Q type="real" size="1">0.0</PP_Q>
          <PP_QIJ.1.1 first_index="1" second_index="1" composite_index="1" angular_momentum="0">0.2</PP_QIJ.1.1>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("q_with_l"));
    assert!(err.to_string().contains("PP_QIJ.1.1"));
}

#[test]
fn q_with_l_false_rejects_qijl_naming() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="invalid"
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
        <PP_AUGMENTATION q_with_l="F" nqf="0" nqlc="1">
          <PP_Q type="real" size="1">0.0</PP_Q>
          <PP_QIJL.1.1.0 first_index="1" second_index="1" composite_index="1" angular_momentum="0">0.2</PP_QIJL.1.1.0>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM type="real" size="1">0.2</PP_RHOATOM>
    </UPF>
    "#;

    let err = from_str(xml).unwrap_err();
    assert!(err.to_string().contains("q_with_l"));
    assert!(err.to_string().contains("PP_QIJL.1.1.0"));
}
