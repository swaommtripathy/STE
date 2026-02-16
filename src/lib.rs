mod pb;

use pb::solana::transfers::v1::{Transfer, Transfers};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

// System Program ID - responsible for SOL transfers
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<Transfers, Error> {
    let mut transfers = Vec::new();

    // Iterate through all transactions in the block
    for confirmed_tx in block.transactions_owned() {
        // Skip failed transactions
        if confirmed_tx.meta.is_none() {
            continue;
        }

        let meta = confirmed_tx.meta.as_ref().unwrap();
        if meta.err.is_some() {
            continue;
        }

        // Get transaction and accounts
        let tx = match confirmed_tx.transaction {
            Some(tx) => tx,
            None => continue,
        };

        let message = match tx.message {
            Some(msg) => msg,
            None => continue,
        };

        // Get account keys (wallet addresses)
        let account_keys = &message.account_keys;

        // Iterate through all instructions
        for instruction in message.instructions.iter() {
            let program_id_index = instruction.program_id_index as usize;
            if program_id_index >= account_keys.len() {
                continue;
            }

            let program_id = &account_keys[program_id_index];

            // Check if this is a System Program instruction
            if program_id == SYSTEM_PROGRAM_ID.as_bytes() {
                // Parse the instruction data to extract transfer details
                if let Some(transfer) = parse_system_instruction(&instruction.data, &instruction.accounts, account_keys) {
                    transfers.push(transfer);
                }
            }
        }

        // Also check inner instructions (these are NOT wrapped in Option)
        for inner_instruction_set in &meta.inner_instructions {
            for inner_instruction in &inner_instruction_set.instructions {
                let program_id_index = inner_instruction.program_id_index as usize;
                if program_id_index >= account_keys.len() {
                    continue;
                }

                let program_id = &account_keys[program_id_index];

                if program_id == SYSTEM_PROGRAM_ID.as_bytes() {
                    if let Some(transfer) = parse_system_instruction(&inner_instruction.data, &inner_instruction.accounts, account_keys) {
                        transfers.push(transfer);
                    }
                }
            }
        }
    }

    Ok(Transfers { transfers })
}

fn parse_system_instruction(
    data: &[u8],
    account_indices: &[u8],
    account_keys: &[Vec<u8>],
) -> Option<Transfer> {
    // System Program Transfer instruction format:
    // - First 4 bytes: instruction discriminator (u32 little-endian)
    // - Next 8 bytes: lamports amount (u64 little-endian)
    
    // Need at least 12 bytes total
    if data.len() < 12 {
        return None;
    }

    // Parse instruction discriminator (u32 little endian)
    let discriminator = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // Transfer instruction has discriminator value of 2
    if discriminator != 2 {
        return None;
    }

    // Parse lamports amount (u64 little endian) - starts at byte 4
    let amount = u64::from_le_bytes([
        data[4], data[5], data[6], data[7],
        data[8], data[9], data[10], data[11],
    ]);

    // Transfer instruction requires exactly 2 accounts:
    // account_indices[0] = funding account (from)
    // account_indices[1] = recipient account (to)
    if account_indices.len() < 2 {
        return None;
    }

    let from_index = account_indices[0] as usize;
    let to_index = account_indices[1] as usize;

    // Validate indices are within bounds
    if from_index >= account_keys.len() || to_index >= account_keys.len() {
        return None;
    }

    // Convert account keys (bytes) to base58 strings
    let from = bs58::encode(&account_keys[from_index]).into_string();
    let to = bs58::encode(&account_keys[to_index]).into_string();

    Some(Transfer {
        from,
        to,
        amount,
    })
}