use upf::{
    from_str,
    model::{AtomicRelativisticFormalism, PseudopotentialType},
};

const MINIMAL_UPF: &str = r#"
<UPF version="2.0.1">
  <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="minimal"
             element="He" pseudo_type="NC" relativistic="scalar"
             is_ultrasoft="F" is_paw="F" is_coulomb="F"
             has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
             functional="PBE" z_valence="2.0"
             l_max="0" mesh_size="3" number_of_wfc="0" number_of_proj="0" />
  <PP_MESH mesh="3">
    <PP_R type="real" size="3">0.0 0.1 0.2</PP_R>
    <PP_RAB type="real" size="3">0.1 0.1 0.1</PP_RAB>
  </PP_MESH>
  <PP_LOCAL type="real" size="3">1.0 2.0 3.0</PP_LOCAL>
  <PP_NONLOCAL />
  <PP_RHOATOM type="real" size="3">0.2 0.3 0.4</PP_RHOATOM>
</UPF>
"#;

#[test]
fn minimal_document_deserializes_core_sections() {
    let doc = from_str(MINIMAL_UPF).unwrap();

    assert_eq!(doc.version, "2.0.1");
    assert_eq!(doc.header.element, "He");
    assert_eq!(doc.header.functional, "PBE");
    assert_eq!(doc.header.pseudo_type, PseudopotentialType::NormConserving);
    assert_eq!(doc.header.total_psenergy, None);
    assert_eq!(doc.header.wfc_cutoff, None);
    assert_eq!(doc.header.rho_cutoff, None);
    assert_eq!(doc.header.l_max_rho, None);
    assert_eq!(doc.header.l_local, None);
    assert_eq!(
        doc.header.relativistic,
        AtomicRelativisticFormalism::ScalarRelativistic
    );
    assert_eq!(doc.mesh.r.values, vec![0.0, 0.1, 0.2]);
    assert_eq!(doc.local.values, vec![1.0, 2.0, 3.0]);
    assert_eq!(doc.rhoatom.values, vec![0.2, 0.3, 0.4]);
}
