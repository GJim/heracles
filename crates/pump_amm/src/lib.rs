use anchor_client::anchor_lang::prelude::*;
use solana_client::rpc_response::{Response, RpcLogsResponse};

declare_program!(pump_amm);

use crate::pump_amm::events;
pub use crate::pump_amm::ID;

#[derive(Debug)]
pub enum Event {
    Buy(events::BuyEvent),
    Sell(events::SellEvent),
    CreatePool(events::CreatePoolEvent),
    Deposit(events::DepositEvent),
    Withdraw(events::WithdrawEvent),
}

#[must_use]
pub fn parse_logs_response(logs: &Response<RpcLogsResponse>, program_id_str: &str) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();
    for log in &logs.value.logs[..] {
        if let Ok((Some(event), _, _)) =
            anchor_client::handle_program_log::<events::BuyEvent>(program_id_str, log)
        {
            events.push(Event::Buy(event));
        }
        if let Ok((Some(event), _, _)) =
            anchor_client::handle_program_log::<events::SellEvent>(program_id_str, log)
        {
            events.push(Event::Sell(event));
        }
        if let Ok((Some(event), _, _)) =
            anchor_client::handle_program_log::<events::CreatePoolEvent>(program_id_str, log)
        {
            events.push(Event::CreatePool(event));
        }
        if let Ok((Some(event), _, _)) =
            anchor_client::handle_program_log::<events::DepositEvent>(program_id_str, log)
        {
            events.push(Event::Deposit(event));
        }
        if let Ok((Some(event), _, _)) =
            anchor_client::handle_program_log::<events::WithdrawEvent>(program_id_str, log)
        {
            events.push(Event::Withdraw(event));
        }
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_response::{RpcLogsResponse, RpcResponseContext};

    #[test]
    fn test_parse_logs_response_buy_event() {
        // Create a mock RPC logs response with the provided logs
        let logs = vec![
            "Program ComputeBudget111111111111111111111111111111 invoke [1]".to_string(),
            "Program ComputeBudget111111111111111111111111111111 success".to_string(),
            "Program ComputeBudget111111111111111111111111111111 invoke [1]".to_string(),
            "Program ComputeBudget111111111111111111111111111111 success".to_string(),
            "Program sattCHvHkM4XHLyadnU4KQtuNWZbWVDKzuPhmJBXCkq invoke [1]".to_string(),
            "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA invoke [2]".to_string(),
            "Program log: Instruction: Buy".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".to_string(),
            "Program log: Instruction: TransferChecked".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 199168 compute units".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".to_string(),
            "Program log: Instruction: TransferChecked".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 190354 compute units".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".to_string(),
            "Program log: Instruction: TransferChecked".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 181464 compute units".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            "Program data: Z/RSHyz1d3d1SOpnAAAAAGb4hZQHAAAAia4KGQAAAAAAAAAAAAAAAITRnPLlBQAAfIUSmXIuAACsgqb2mAAAAOyx+hgAAAAAFAAAAAAAAAAWygwAAAAAAAUAAAAAAAAAhjIDAAAAAAACfAcZAAAAAIiuChkAAAAAs2KoY0K+8wW11Mzg7EPJF5YwXehmJXMsVZt7MEYcSPIU6zbh3Tj7uDLsYRbIHIH8Nxc2EsCB0aH03gYW/tnNAIr01Uaez9OsUAzDGS6qy0F7QYyFfp5dj72V3hxIbzYm0bqMzqITUWXmdMfp+z3UPNRElWs2FFc6IsqBgy9R1gFjg3MADqIssmTTSv9koEte+r+7dN3NBImXsZgVR9fREAe0ZyjFA6fIFZjsUWe5tjKg2nvc6Y8HxZZ7EO1veKHO".to_string(),
            "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA invoke [3]".to_string(),
            "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA consumed 2004 of 169362 compute units".to_string(),
            "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA success".to_string(),
            "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA consumed 58848 of 225667 compute units".to_string(),
            "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA success".to_string(),
            "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo invoke [2]".to_string(),
            "Program log: Instruction: Swap".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".to_string(),
            "Program log: Instruction: TransferChecked".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 131294 compute units".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]".to_string(),
            "Program log: Instruction: TransferChecked".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 121605 compute units".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo invoke [3]".to_string(),
            "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo consumed 2135 of 112112 compute units".to_string(),
            "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success".to_string(),
            "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo consumed 54985 of 163585 compute units".to_string(),
            "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success".to_string(),
            "Program sattCHvHkM4XHLyadnU4KQtuNWZbWVDKzuPhmJBXCkq consumed 159980 of 268485 compute units".to_string(),
            "Program sattCHvHkM4XHLyadnU4KQtuNWZbWVDKzuPhmJBXCkq success".to_string(),
            "Program ComputeBudget111111111111111111111111111111 invoke [1]".to_string(),
            "Program ComputeBudget111111111111111111111111111111 success".to_string(),
        ];

        let response = Response {
            context: RpcResponseContext { slot: 0, api_version: None },
            value: RpcLogsResponse { signature: "test_signature".to_string(), err: None, logs },
        };

        // Test parsing with the pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA program ID
        let program_id_str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
        let events = parse_logs_response(&response, program_id_str);

        // Check that we have at least one event
        assert!(!events.is_empty(), "No events were parsed from the logs");

        // Check that the first event is a Buy event
        match &events[0] {
            Event::Buy(_) => {
                // Test passed - we found a Buy event
                println!("Successfully parsed a Buy event");
            }
            _ => {
                panic!("First event should be a Buy event, but got: {:?}", events[0]);
            }
        }
    }

    #[test]
    fn test_parse_logs_response_empty_logs() {
        // Create a mock RPC logs response with empty logs
        let logs: Vec<String> = Vec::new();

        let response = Response {
            context: RpcResponseContext { slot: 0, api_version: None },
            value: RpcLogsResponse { signature: "test_signature".to_string(), err: None, logs },
        };

        // Test parsing with any program ID
        let program_id_str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
        let events = parse_logs_response(&response, program_id_str);

        // Check that we have no events
        assert!(events.is_empty(), "Should not have parsed any events from empty logs");
    }
}
