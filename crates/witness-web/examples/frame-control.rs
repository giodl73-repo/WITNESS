//! Native control transport for the unchanged public fixture projection.
use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("read a control");
        let step: u64 = serde_json::from_str(&line).expect("typed unsigned control");
        let step = usize::try_from(step).expect("control fits this native target");
        let (tag, value) = match witness_web::frame(step) {
            Ok(value) => ("Ok", value),
            Err(value) => ("Err", value),
        };
        println!("{{\"{tag}\":{}}}", serde_json::to_string(&value).unwrap());
    }
}
