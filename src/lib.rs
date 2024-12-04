mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::InitPoll;
use pb::substreams::v1::program::AddCandidate;
use pb::substreams::v1::program::Vote;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "3vQgAS1UoiYFG2gGv7AkFE5Lj1hU9kMr5W8QwiDPaMsF";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut init_poll_list: Vec<InitPoll> = Vec::new();
    let mut add_candidate_list: Vec<AddCandidate> = Vec::new();
    let mut vote_list: Vec<Vote> = Vec::new();

    blk.transactions().for_each(|transaction| {
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();

        transaction
            .walk_instructions()
            .into_iter()
            .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
            .for_each(|inst| {
                let slice_u8: &[u8] = &inst.data()[..];
                
                // Handle InitPoll instruction
                if slice_u8[0..8] == idl::idl::program::client::args::InitPoll::DISCRIMINATOR {
                    if let Ok(instruction) = idl::idl::program::client::args::InitPoll::deserialize(&mut &slice_u8[8..]) {
                        init_poll_list.push(InitPoll {
                            poll_id: instruction.poll_id,
                            description: instruction.description,
                            authority: inst.accounts()[1].0.to_vec(), // Using .0 to access the inner bytes
                            account: inst.accounts()[2].0.to_vec(), // Using .0 to access the inner bytes
                        });
                    }
                }
                
                // Handle AddCandidate instruction
                else if slice_u8[0..8] == idl::idl::program::client::args::AddCandidate::DISCRIMINATOR {
                    let poll_account = &inst.accounts()[1];
                    let candidate_account = &inst.accounts()[0];
                    
                    
                    add_candidate_list.push(AddCandidate {
                        poll_address: poll_account.0.to_vec(),
                        candidate_address: candidate_account.0.to_vec(),
                        candidate_name: format!("CAND"), // Placeholder for now
                        initial_votes: 0, // Placeholder for now
                    });
                    
                }
                
                // Handle Vote instruction
                else if slice_u8[0..8] == idl::idl::program::client::args::Vote::DISCRIMINATOR {
                    let candidate_account = &inst.accounts()[0];
                    let voter_account = &inst.accounts()[1];
                    
                    
                    vote_list.push(Vote {
                        candidate_address: candidate_account.0.to_vec(),
                        voter: voter_account.0.to_vec(),
                        new_vote_count: 0, // Placeholder for now
                    });
                    
                }
            });
    });

    Data {
        init_poll_list,
        add_candidate_list,
        vote_list,
    }
}


#[substreams::handlers::map]
fn all_transactions_without_votes(blk: Block) -> Result<Transactions, substreams::errors::Error> {
    let transactions: Vec<ConfirmedTransaction> = blk.transactions.into_iter().collect();

    substreams::log::println(format!("Number of transactions:{}", transactions.len()));

    Ok(Transactions { transactions })
}


