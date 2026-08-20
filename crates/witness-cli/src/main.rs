use witness_core::claude_session_fixture;

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("status") => {
            println!("witness public core: ready");
            println!("commands: status, replay");
        }
        Some("replay") => {
            let replay = claude_session_fixture();
            println!("fixture: {}", replay.fixture);
            println!("events: {}", replay.events.len());
            println!("active_cut: {}", replay.active_cut);
            println!("checkpoint: {}", replay.checkpoint);
            println!("frontier_count: {}", replay.frontier_count);
        }
        _ => {
            eprintln!("usage: witness-cli <status|replay>");
            std::process::exit(2);
        }
    }
}
