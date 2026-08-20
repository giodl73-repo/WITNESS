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
