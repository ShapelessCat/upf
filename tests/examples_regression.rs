use std::path::PathBuf;

use upf::from_file;

fn example_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/SSSP_1.3.0_PBE_efficiency/UPF_2.x")
}

#[test]
fn reads_upf_2x_examples() {
    let mut attempted = 0usize;
    let mut failures = Vec::new();

    let mut entries = std::fs::read_dir(example_dir())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for path in entries {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(first_char) = name.chars().next() else {
            continue;
        };
        let first_char = first_char.to_ascii_uppercase();
        if !('A'..='Y').contains(&first_char) {
            continue;
        }

        attempted += 1;
        if let Err(err) = from_file(&path) {
            failures.push(format!("{name}: {err}"));
        }
    }

    assert!(attempted > 0, "no target files matched the A-N filter");
    assert!(
        failures.is_empty(),
        "failed to read {} of {} files:\n{}",
        failures.len(),
        attempted,
        failures.join("\n")
    );
}
