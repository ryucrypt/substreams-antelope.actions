use crate::pb;

use serde_json::json;
use substreams::errors::Error;
use substreams::log;
use pb::antelope::action::v1::ActionEvents;
use pb::sf::substreams::sink::files::v1::Lines;

#[substreams::handlers::map]
fn jsonl_out(events: ActionEvents) -> Result<Lines, Error> {
    let mut lines = vec![];

    for event in events.items.into_iter() {
        lines.push(
            json!({
                "trx_id": event.trx_id,
                "timestamp": event.timestamp.unwrap().to_string(),
                "contract": event.contract, 
                "action": event.action,
                "data": event.data,
            }).to_string()
        );
    }
    Ok(Lines { lines })
}
