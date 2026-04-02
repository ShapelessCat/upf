use std::{fs, io::Cursor};

use upf::{from_str, to_file, to_writer};

const SAMPLE: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="roundtrip"
             element="He" pseudo_type="NC" relativistic="scalar"
             is_ultrasoft="F" is_paw="F" is_coulomb="F"
             has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
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
fn file_entry_point_writes_to_disk() {
    let doc = from_str(SAMPLE).unwrap();
    let path = std::env::temp_dir().join("upf-write-api.upf");
    to_file(&path, &doc).unwrap();

    let xml = fs::read_to_string(&path).unwrap();
    assert!(xml.contains("<PP_RHOATOM>"));
}
