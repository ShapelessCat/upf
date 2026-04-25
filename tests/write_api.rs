use std::{fs, io::Cursor, path::PathBuf};

use upf::{from_file, from_str, to_file, to_writer};

const SAMPLE: &str = r#"
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
fn writer_entry_point_emits_xml() {
    let doc = from_str(SAMPLE).unwrap();
    let mut buf = Cursor::new(Vec::new());
    to_writer(&mut buf, &doc).unwrap();

    let xml = String::from_utf8(buf.into_inner()).unwrap();
    assert!(xml.contains("<UPF version=\"2.0.1\">"));
    assert!(xml.contains("<PP_HEADER"));
}

#[test]
fn writer_emits_computed_size_without_type_or_columns_for_plain_numeric_sections() {
    let doc = from_str(SAMPLE).unwrap();
    let mut buf = Cursor::new(Vec::new());

    to_writer(&mut buf, &doc).unwrap();

    let xml = String::from_utf8(buf.into_inner()).unwrap();
    assert!(xml.contains("<PP_R size=\"3\">"));
    assert!(xml.contains("<PP_RAB size=\"3\">"));
    assert!(xml.contains("<PP_LOCAL size=\"3\">"));
    assert!(xml.contains("<PP_RHOATOM size=\"3\">"));
    assert!(!xml.contains("<PP_R type="));
    assert!(!xml.contains("<PP_R columns="));
    assert!(!xml.contains("<PP_LOCAL type="));
    assert!(!xml.contains("<PP_RHOATOM columns="));
}

#[test]
fn file_entry_point_writes_to_disk() {
    let doc = from_str(SAMPLE).unwrap();
    let path = std::env::temp_dir().join("upf-write-api.upf");
    to_file(&path, &doc).unwrap();

    let xml = fs::read_to_string(&path).unwrap();
    assert!(xml.contains("<PP_RHOATOM size=\"3\">"));
}

#[test]
fn writer_emits_extended_sections() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="write-extended"
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
    let doc = from_str(xml).unwrap();
    let mut buf = Cursor::new(Vec::new());

    to_writer(&mut buf, &doc).unwrap();

    let written = String::from_utf8(buf.into_inner()).unwrap();
    assert!(written.contains("with_metagga_info=\"T\""));
    assert!(written.contains("<PP_TAUMOD"));
    assert!(written.contains("<PP_TAUATOM"));
    assert!(written.contains("<PP_SPIN_ORB>"));
    assert!(written.contains("PP_RELWFC.2"));
    assert!(written.contains("PP_RELBETA.3"));
    assert!(written.contains("PP_GIPAW_ORBITAL.1"));
    assert!(written.contains("PP_GIPAW_VLOCAL_AE"));
}

#[test]
fn writer_recomputes_pp_dij_size_without_preserving_read_time_type_rows_or_columns() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="dij-write"
                 element="Ne" pseudo_type="USPP" relativistic="scalar"
                 is_ultrasoft="T" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="8.0" total_psenergy="-1.0"
                 l_max="1" mesh_size="2" number_of_wfc="0" number_of_proj="2" />
      <PP_MESH mesh="2">
        <PP_R>0.0 0.1</PP_R>
        <PP_RAB>0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 1.1</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0" cutoff_radius="1.0">0.1 0.2</PP_BETA.1>
        <PP_BETA.2 index="2" angular_momentum="1" cutoff_radius="1.1">0.3 0.4</PP_BETA.2>
        <PP_DIJ type="real" size="99" rows="9" columns="9">0.1 0.2 0.3 0.4</PP_DIJ>
        <PP_AUGMENTATION q_with_l="F" nqf="0" nqlc="1">
          <PP_Q>0.0 0.1 0.2 0.3</PP_Q>
          <PP_QIJ.1.1 first_index="1" second_index="1" composite_index="1">0.0 0.0</PP_QIJ.1.1>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM>0.2 0.3</PP_RHOATOM>
    </UPF>
    "#;
    let doc = from_str(xml).unwrap();
    let mut buf = Cursor::new(Vec::new());

    to_writer(&mut buf, &doc).unwrap();

    let written = String::from_utf8(buf.into_inner()).unwrap();
    assert!(written.contains("<PP_DIJ size=\"4\">"));
    assert!(!written.contains("<PP_DIJ type="));
    assert!(!written.contains("rows="));
    assert!(!written.contains("columns="));
}

#[test]
fn writer_preserves_real_fixture_metadata() {
    let doc = from_file(example("Fe.pbe-spn-kjpaw_psl.0.2.1.UPF")).unwrap();
    let mut buf = Cursor::new(Vec::new());

    to_writer(&mut buf, &doc).unwrap();

    let written = String::from_utf8(buf.into_inner()).unwrap();
    let reparsed = from_str(&written).unwrap();
    assert!(written.contains("cutoff_radius_index=\"874\""));
    assert!(written.contains("<PP_FULL_WFC number_of_wfc=\"6\">"));
    assert_eq!(reparsed.full_wfc.as_ref().unwrap().number_of_wfc, Some(6));
    assert_eq!(
        reparsed.full_wfc.as_ref().unwrap().entries[0].value.index,
        Some(1)
    );
}
