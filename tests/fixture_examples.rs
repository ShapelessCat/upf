use std::path::PathBuf;

use upf::{from_file, from_str};

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

fn example_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/SSSP_1.3.0_PBE_efficiency")
}

#[test]
fn parses_oncv_example_with_missing_optional_header_fields() {
    let path = example("Mo_ONCV_PBE-1.0.oncvpsp.upf");
    let doc = from_file(path).unwrap();

    assert_eq!(doc.header.element, "Mo");
    assert_eq!(doc.header.number_of_proj, 6);
    assert_eq!(doc.pswfc.as_ref().unwrap().orbitals.len(), 4);
}

#[test]
fn parses_qe_paw_example_with_full_wfc_and_gipaw() {
    let path = example("Fe.pbe-spn-kjpaw_psl.0.2.1.UPF");
    let doc = from_file(path).unwrap();

    assert_eq!(doc.header.element, "Fe");
    let nonlocal = doc.nonlocal.as_ref().unwrap();
    assert_eq!(nonlocal.betas.len(), 6);
    assert_eq!(nonlocal.betas[0].value.values.len(), doc.header.mesh_size);
    assert_eq!(nonlocal.betas[0].value.label.as_deref(), Some("3S"));
    assert_eq!(nonlocal.betas[0].value.cutoff_radius_index, Some(874));
    assert_eq!(doc.full_wfc.as_ref().unwrap().ae_wfc.len(), 6);
    assert_eq!(doc.full_wfc.as_ref().unwrap().ps_wfc.len(), 6);
    assert!(doc.full_wfc.as_ref().unwrap().ae_wfc_rel.is_none());
    assert_eq!(doc.full_wfc.as_ref().unwrap().number_of_wfc, Some(6));
    assert_eq!(
        doc.full_wfc.as_ref().unwrap().ae_wfc[0].value.index,
        Some(1)
    );
    assert!(doc.gipaw.is_some());
}

#[test]
fn parses_lowercase_boolean_paw_example() {
    let path = example("Pu.paw.z_16.ld1.uni-marburg.v0.upf");
    let doc = from_file(path).unwrap();

    assert!(doc.header.is_paw);
    assert_eq!(doc.nonlocal.as_ref().unwrap().betas.len(), 8);
    assert!(doc.paw.is_some());
}

#[test]
fn numeric_section_attributes_are_ignored_on_read() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="0" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R type="real" size="3" columns="4">0.0 0.1 0.2</PP_R>
        <PP_RAB type="real" size="3" columns="4">0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL type="real" size="3" columns="4">1.0 2.0 3.0</PP_LOCAL>
      <PP_RHOATOM type="real" size="3" columns="4">0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();

    assert_eq!(doc.mesh.r, vec![0.0, 0.1, 0.2]);
    assert_eq!(doc.mesh.rab, vec![0.1, 0.1, 0.1]);
    assert_eq!(doc.local, vec![1.0, 2.0, 3.0]);
    assert_eq!(doc.rhoatom, vec![0.2, 0.3, 0.4]);
}

#[test]
fn missing_header_comment_defaults_to_empty_string() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03"
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
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();

    assert_eq!(doc.header.element, "He");
    assert_eq!(doc.header.comment, "");
}

#[test]
fn parses_beta_without_index() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="1" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R>0.0 0.1 0.2</PP_R>
        <PP_RAB>0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 angular_momentum="0" cutoff_radius="1.0">0.1 0.2 0.3</PP_BETA.1>
        <PP_DIJ>0.0</PP_DIJ>
      </PP_NONLOCAL>
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();

    assert_eq!(doc.header.element, "He");
    let nonlocal = doc.nonlocal.as_ref().unwrap();
    assert_eq!(nonlocal.betas.len(), 1);
    assert_eq!(nonlocal.betas[0].value.index, None);
}

#[test]
fn parses_beta_without_cutoff_radius() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="1" />
      <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
        <PP_R>0.0 0.1 0.2</PP_R>
        <PP_RAB>0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0">0.1 0.2 0.3</PP_BETA.1>
        <PP_DIJ>0.0</PP_DIJ>
      </PP_NONLOCAL>
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();

    assert_eq!(doc.header.element, "He");
    let nonlocal = doc.nonlocal.as_ref().unwrap();
    assert_eq!(nonlocal.betas.len(), 1);
    assert_eq!(nonlocal.betas[0].value.cutoff_radius, None);
}

#[test]
fn parses_fortran_double_precision_attribute_values() {
    let xml = r#"
    <UPF version="2.0.1">
      <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment=""
                 element="He" pseudo_type="NC" relativistic="scalar"
                 is_ultrasoft="F" is_paw="F" is_coulomb="F"
                 has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
                 functional="PBE"
                 z_valence="2.d0" total_psenergy="-1.25"
                 wfc_cutoff="20.0" rho_cutoff="80.0"
                 l_max="0" l_max_rho="0" l_local="0"
                 mesh_size="3" number_of_wfc="0" number_of_proj="1" />
      <PP_MESH dx="0.0125d0" mesh="3" xmin="-7.d0" rmax="0.2" zmesh="65.0">
        <PP_R>0.0 0.1 0.2</PP_R>
        <PP_RAB>0.1 0.1 0.1</PP_RAB>
      </PP_MESH>
      <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
      <PP_NONLOCAL>
        <PP_BETA.1 index="1" angular_momentum="0">0.1 0.2 0.3</PP_BETA.1>
        <PP_DIJ>0.0</PP_DIJ>
        <PP_AUGMENTATION q_with_l="T" nqf="0" nqlc="1" augmentation_epsilon="1.d-12">
          <PP_Q>0.0</PP_Q>
          <PP_QIJL.1.1.0 first_index="1" second_index="1" composite_index="1" angular_momentum="0">
            0.0 0.0 0.0
          </PP_QIJL.1.1.0>
        </PP_AUGMENTATION>
      </PP_NONLOCAL>
      <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
    </UPF>
    "#;

    let doc = from_str(xml).unwrap();

    assert_eq!(doc.header.z_valence, 2.0);
    assert_eq!(doc.mesh.dx, Some(0.0125));
    assert_eq!(doc.mesh.xmin, Some(-7.0));
    assert_eq!(
        doc.nonlocal
            .as_ref()
            .unwrap()
            .augmentation
            .as_ref()
            .unwrap()
            .augmentation_epsilon,
        Some(1.0e-12)
    );
}

#[test]
fn all_example_files_use_a_upf_root_wrapper() {
    let mut invalid = Vec::new();

    for subdir in ["UPF_1.x", "UPF_2.x"] {
        for entry in std::fs::read_dir(example_dir().join(subdir)).unwrap() {
            let path = entry.unwrap().path();
            let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
                continue;
            };

            if extension != "UPF" && extension != "upf" {
                continue;
            }

            let contents = std::fs::read_to_string(&path).unwrap();
            let Some(first_non_empty_line) = contents.lines().find(|line| !line.trim().is_empty())
            else {
                invalid.push(path.file_name().unwrap().to_string_lossy().into_owned());
                continue;
            };

            if first_non_empty_line != r#"<?xml version="1.0" encoding="UTF-8"?>"#
                && first_non_empty_line != r#"<UPF version="2.0.1">"#
                && first_non_empty_line != r#"<UPF>"#
            {
                invalid.push(path.file_name().unwrap().to_string_lossy().into_owned());
                continue;
            }

            if !contents.contains("</UPF>") {
                invalid.push(path.file_name().unwrap().to_string_lossy().into_owned());
            }
        }
    }

    assert!(
        invalid.is_empty(),
        "examples missing UPF root wrapper: {invalid:?}"
    );
}
