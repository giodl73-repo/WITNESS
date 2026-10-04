//! Provider-neutral public fixture projection; no live session capture.
use witness_core::{HarnessEvent, HarnessEventKind as Kind, HarnessReplay};
fn fixture() -> HarnessReplay {
    let events = [
        (
            "bootstrap",
            Kind::BootstrapContext,
            "Load the public sample instructions",
            Some("sample:instructions"),
        ),
        (
            "intent",
            Kind::UserTurn,
            "Improve the sample search interface",
            None,
        ),
        (
            "read",
            Kind::FileRead,
            "Read the sample interface source",
            Some("sample:interface.rs"),
        ),
        (
            "edit",
            Kind::FileEdit,
            "Record a proposed interface edit and validation obligation",
            Some("sample:interface.rs"),
        ),
        (
            "validate",
            Kind::Validation,
            "Attach passing fixture validation evidence",
            None,
        ),
        (
            "checkpoint",
            Kind::Checkpoint,
            "Save the sample checkpoint and its receipts",
            None,
        ),
        (
            "rehydrate",
            Kind::Rehydrate,
            "Resume from the sample checkpoint",
            None,
        ),
    ];
    HarnessReplay {
        fixture: "public-browser-sample",
        substrate: "typed public event projection",
        active_cut: "sample:active",
        checkpoint: "sample:checkpoint",
        rehydrated_cut: "sample:resumed",
        frontier_count: 0,
        events: events
            .into_iter()
            .map(|(id, kind, summary, source_pointer)| HarnessEvent {
                id,
                kind,
                summary,
                source_pointer,
                receipt: id,
            })
            .collect(),
    }
}
pub fn frame(step: usize) -> Result<String, String> {
    let mut replay = fixture();
    let total = replay.event_count();
    if step > total {
        return Err("Choose a replay step between 0 and 7".into());
    }
    replay.events.truncate(step);
    if step < 6 {
        replay.checkpoint = "not yet checkpointed";
    }
    if step < 7 {
        replay.rehydrated_cut = "not yet resumed";
    }
    if step == 0 {
        replay.active_cut = "not yet started";
    }
    let mut value: serde_json::Value =
        serde_json::from_str(&replay.to_json()).map_err(|e| e.to_string())?;
    value["step"] = step.into();
    value["total_steps"] = total.into();
    value["validation_attached"] = (step >= 5).into();
    value["validation_owed"] = (step == 4).into();
    value["scope"] =
        "synthetic fixture projection; no closure execution or live provider data".into();
    serde_json::to_string(&value).map_err(|e| e.to_string())
}
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn frame_json(step: usize) -> Result<String, wasm_bindgen::JsValue> {
    frame(step).map_err(|e| wasm_bindgen::JsValue::from_str(&e))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_counts_and_obligations_follow_events() {
        for step in 0..=7 {
            let v: serde_json::Value = serde_json::from_str(&frame(step).unwrap()).unwrap();
            assert_eq!(v["event_count"], step);
            assert_eq!(v["receipt_count"], step);
            assert_eq!(v["events"].as_array().unwrap().len(), step);
        }
        let edited: serde_json::Value = serde_json::from_str(&frame(4).unwrap()).unwrap();
        assert_eq!(edited["validation_owed"], true);
        assert_eq!(edited["source_pointer_count"], 3);
        assert_eq!(edited["checkpoint"], "not yet checkpointed");
        let done: serde_json::Value = serde_json::from_str(&frame(7).unwrap()).unwrap();
        assert_eq!(done["validation_attached"], true);
        assert_eq!(done["rehydrated_cut"], "sample:resumed");
        assert!(frame(8).is_err());
    }
}
