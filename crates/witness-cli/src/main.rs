use witness_core::{claude_session_fixture, provider_projection_fixture};

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next();
    let json = args.any(|arg| arg == "--json");

    match command.as_deref() {
        Some("status") => {
            if json {
                println!(
                    "{{\"schema\":\"witness.status.v1\",\"status\":\"ready\",\"commands\":[\"status\",\"replay\",\"provider-projection\"]}}"
                );
            } else {
                println!("witness public core: ready");
                println!("commands: status, replay, provider-projection");
            }
        }
        Some("replay") => {
            let replay = claude_session_fixture();
            if json {
                println!("{}", replay.to_json());
            } else {
                println!("fixture: {}", replay.fixture);
                println!("events: {}", replay.events.len());
                println!("active_cut: {}", replay.active_cut);
                println!("checkpoint: {}", replay.checkpoint);
                println!("frontier_count: {}", replay.frontier_count);
            }
        }
        Some("provider-projection") => {
            let report = provider_projection_fixture();
            if json {
                println!("{}", report.to_json());
            } else {
                println!("fixture: {}", report.fixture);
                println!("provider_adapter: {}", report.provider_adapter);
                println!("compatibility_status: {}", report.compatibility_status);
                println!("losses: {}", report.losses.len());
            }
        }
        _ => {
            eprintln!("usage: witness-cli <status|replay|provider-projection> [--json]");
            std::process::exit(2);
        }
    }
}
