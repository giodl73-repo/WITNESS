use std::collections::BTreeMap;
use std::process::Command;

fn proof() -> BTreeMap<&'static str, &'static str> {
    include_str!("fixtures/replay_cli_proof.txt")
        .lines()
        .map(|line| line.split_once('=').expect("proof fixture uses key=value"))
        .collect()
}

#[test]
fn replay_cli_proof_records_acceptance_and_structured_failure() {
    let proof = proof();
    let accepted = Command::new(env!("CARGO_BIN_EXE_witness-cli"))
        .arg(proof["accepted.command"])
        .output()
        .expect("run accepted replay");
    assert!(accepted.status.success());
    let stdout = String::from_utf8(accepted.stdout).unwrap();
    assert!(stdout.contains(proof["accepted.stdout"]));
    assert!(stdout.contains(proof["accepted.events"]));

    let accepted_json = Command::new(env!("CARGO_BIN_EXE_witness-cli"))
        .arg(proof["accepted.command"])
        .arg("--json")
        .output()
        .expect("run accepted JSON replay");
    assert!(accepted_json.status.success());
    let json_stdout = String::from_utf8(accepted_json.stdout).unwrap();
    assert!(json_stdout.contains(proof["accepted.json_schema"]));
    assert!(json_stdout.contains(proof["accepted.json_events"]));

    let status_json = Command::new(env!("CARGO_BIN_EXE_witness-cli"))
        .arg("status")
        .arg("--json")
        .output()
        .expect("run JSON status");
    assert!(status_json.status.success());
    assert!(String::from_utf8(status_json.stdout)
        .unwrap()
        .contains(proof["status.json_schema"]));

    let projection = Command::new(env!("CARGO_BIN_EXE_witness-cli"))
        .arg(proof["projection.command"])
        .output()
        .expect("run provider projection report");
    assert!(projection.status.success());
    assert!(String::from_utf8(projection.stdout)
        .unwrap()
        .contains(proof["projection.stdout"]));

    let projection_json = Command::new(env!("CARGO_BIN_EXE_witness-cli"))
        .arg(proof["projection.command"])
        .arg("--json")
        .output()
        .expect("run provider projection JSON report");
    assert!(projection_json.status.success());
    let projection_json_stdout = String::from_utf8(projection_json.stdout).unwrap();
    assert!(projection_json_stdout.contains(proof["projection.json_schema"]));
    assert!(projection_json_stdout.contains(proof["projection.json_loss"]));

    let rejected = Command::new(env!("CARGO_BIN_EXE_witness-cli"))
        .arg(proof["rejected.command"])
        .output()
        .expect("run rejected command");
    assert_eq!(
        rejected.status.code(),
        Some(proof["rejected.exit_code"].parse().unwrap())
    );
    assert!(String::from_utf8(rejected.stderr)
        .unwrap()
        .contains(proof["rejected.stderr"]));
}
