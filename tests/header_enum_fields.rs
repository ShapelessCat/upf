use upf::{
    from_str,
    model::{AtomicRelativisticFormalism, PseudopotentialType},
    to_string,
};

const HEADER_ENUMS_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER
    generated="unit"
    author="tester"
    date="2026-04-03"
    comment="header-enums"
    element="He"
    pseudo_type="NC"
    relativistic="scalar"
    is_ultrasoft="F"
    is_paw="F"
    is_coulomb="F"
    has_so="F"
    has_wfc="F"
    has_gipaw="F"
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
    number_of_proj="0" />
  <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
    <PP_R>0.0 0.1 0.2</PP_R>
    <PP_RAB>0.1 0.1 0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
  <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
</UPF>
"#;

#[test]
fn header_enums_deserialize_into_typed_variants() {
    let doc = from_str(HEADER_ENUMS_UPF).unwrap();

    assert_eq!(doc.header.pseudo_type, PseudopotentialType::NormConserving);
    assert_eq!(
        doc.header.relativistic,
        AtomicRelativisticFormalism::ScalarRelativistic
    );
}

#[test]
fn relativistic_no_alias_deserializes_and_serializes_canonically() {
    let xml = HEADER_ENUMS_UPF.replace(r#"relativistic="scalar""#, r#"relativistic="no""#);

    let doc = from_str(&xml).unwrap();
    assert_eq!(
        doc.header.relativistic,
        AtomicRelativisticFormalism::NonRelativistic
    );

    let serialized = to_string(&doc).unwrap();
    assert!(serialized.contains(r#"relativistic="no""#));
    assert!(!serialized.contains(r#"relativistic="nonrelativistic""#));
}

#[test]
fn header_enums_reject_unknown_wire_values() {
    let invalid_pseudo_type =
        HEADER_ENUMS_UPF.replace(r#"pseudo_type="NC""#, r#"pseudo_type="bogus""#);
    let invalid_relativistic =
        HEADER_ENUMS_UPF.replace(r#"relativistic="scalar""#, r#"relativistic="bogus""#);

    assert!(from_str(&invalid_pseudo_type).is_err());
    assert!(from_str(&invalid_relativistic).is_err());
}
