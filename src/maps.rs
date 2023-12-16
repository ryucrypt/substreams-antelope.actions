use crate::utils;
use crate::pb;

use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;
use pb::antelope::action::v1::{ActionEvent, ActionEvents};

#[substreams::handlers::map]
fn map_logactions(params: String, block: Block) -> Result<ActionEvents, Error> {
    let mut response = vec![];

    let filter_contracts = utils::create_filters(params.as_str(), "contracts");
    let filter_actions = utils::create_filters(params.as_str(), "actions");

    for trx in block.all_transaction_traces() {
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if !filter_contracts.is_empty() && filter_contracts.contains(action_trace.account.as_str()) {
                if !filter_actions.is_empty() && filter_actions.contains(action_trace.name.as_str()) {
                    response.push(ActionEvent {
                        trx_id: trx.id.clone(),
                        timestamp: trx.block_time.clone(),
                        contract: action_trace.account.to_string(),
                        action: action_trace.name.to_string(),
                        data: action_trace.json_data.to_string()
                    });
                }
            }
        }
    }

    Ok(ActionEvents { items: response })
}
