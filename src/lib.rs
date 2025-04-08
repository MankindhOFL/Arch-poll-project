#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

extern crate alloc;

use arch_network::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};
use alloc::collections::BTreeMap as HashMap;
use alloc::{string::String, vec::Vec};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PollInstruction::try_from_slice(instruction_data)?;

    match instruction {
        PollInstruction::CreatePoll { question, options } => {
            msg!("Instruction: Create Poll");
            process_create_poll(program_id, accounts, question, options)
        }
        PollInstruction::Vote { poll_id, option_index } => {
            msg!("Instruction: Vote");
            process_vote(program_id, accounts, poll_id, option_index)
        }
        PollInstruction::GetResults { poll_id } => {
            msg!("Instruction: Get Results");
            process_get_results(program_id, accounts, poll_id)
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum PollInstruction {
    CreatePoll {
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: u64,
        option_index: u8,
    },
    GetResults {
        poll_id: u64,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Poll {
    pub creator: Pubkey,
    pub question: String,
    pub options: Vec<String>,
    pub votes: HashMap<Pubkey, u8>,
    pub total_votes: u64,
}

impl Poll {
    pub fn new(creator: Pubkey, question: String, options: Vec<String>) -> Self {
        Self {
            creator,
            question,
            options,
            votes: HashMap::new(),
            total_votes: 0,
        }
    }

    pub fn vote(&mut self, voter: Pubkey, option_index: u8) -> Result<(), ProgramError> {
        if option_index as usize >= self.options.len() {
            return Err(ProgramError::InvalidArgument);
        }

        if self.votes.contains_key(&voter) {
            return Err(ProgramError::InvalidArgument);
        }

        self.votes.insert(voter, option_index);
        self.total_votes += 1;
        Ok(())
    }

    pub fn get_results(&self) -> Vec<u64> {
        let mut results = vec![0; self.options.len()];
        for &option_index in self.votes.values() {
            results[option_index as usize] += 1;
        }
        results
    }
}

// Create a new poll
fn process_create_poll(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    question: String,
    options: Vec<String>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let poll_account = next_account_info(account_info_iter)?;
    let creator = next_account_info(account_info_iter)?;

    // Verify the creator signed the transaction
    if !creator.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Create the poll
    let poll = Poll::new(*creator.key, question, options);

    // Serialize and save the poll
    poll.serialize(&mut *poll_account.data.borrow_mut())?;

    msg!("Poll created successfully");
    Ok(())
}

// Process a vote
fn process_vote(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    poll_id: u64,
    option_index: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let poll_account = next_account_info(account_info_iter)?;
    let voter = next_account_info(account_info_iter)?;

    // Verify the voter signed the transaction
    if !voter.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Deserialize the poll
    let mut poll: Poll = Poll::try_from_slice(&poll_account.data.borrow())?;

    // Record the vote
    poll.vote(*voter.key, option_index)?;

    // Serialize and save the updated poll
    poll.serialize(&mut *poll_account.data.borrow_mut())?;

    msg!("Vote recorded successfully");
    Ok(())
}

// Get poll results
fn process_get_results(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    poll_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let poll_account = next_account_info(account_info_iter)?;

    // Deserialize the poll
    let poll: Poll = Poll::try_from_slice(&poll_account.data.borrow())?;

    // Calculate results
    let results = poll.get_results();
    msg!("Results: {:?}", results);

    Ok(())
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
} 
