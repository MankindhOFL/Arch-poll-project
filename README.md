# Blockchain Polling System

A decentralized polling system built on the Arch Network blockchain. This program allows users to create polls, vote on them, and view results in a trustless manner.

## Features

- Create new polls with custom questions and options
- Vote on existing polls (one vote per address)
- View basic poll results (available to all)
- View detailed poll results (creator only)
  - Vote counts and percentages
  - List of all voter addresses
- Secure and transparent voting system
- Creator-specific features and controls

## Project Structure

```
arch_project/
├── Cargo.toml    # Project dependencies and configuration
└── src/
    └── lib.rs    # Main program implementation
```

## Dependencies

- `arch-network`: Blockchain framework
- `borsh`: Serialization library

## Usage

### Creating a Poll

```rust
let instruction = PollInstruction::CreatePoll {
    question: "What's your favorite programming language?".to_string(),
    options: vec!["Rust".to_string(), "Python".to_string(), "JavaScript".to_string()],
};
```

### Voting on a Poll

```rust
let instruction = PollInstruction::Vote {
    poll_id: 1,
    option_index: 0, // Index of the option to vote for
};
```

### Getting Basic Results (Available to All)

```rust
let instruction = PollInstruction::GetResults {
    poll_id: 1,
};
```

### Getting Detailed Results (Creator Only)

```rust
let instruction = PollInstruction::GetDetailedResults {
    poll_id: 1,
};
// Returns:
// - Vote counts for each option
// - Percentage of votes for each option
// - List of all voter addresses
```

## Building

```bash
cargo build
```

## Security

- Each user can only vote once per poll
- Votes are recorded on the blockchain
- The voter must sign the transaction with their private key
- Detailed results are only accessible to the poll creator
- All operations are verified through blockchain signatures

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
