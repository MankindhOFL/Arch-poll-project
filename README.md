# Blockchain Polling System

A decentralized polling system built on the Arch Network blockchain. This program allows users to create polls, vote on them, and view results in a trustless manner.

## Features

- Create new polls with custom questions and options
- Vote on existing polls
- View poll results
- Secure and transparent voting system
- One vote per user per poll

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

### Getting Results

```rust
let instruction = PollInstruction::GetResults {
    poll_id: 1,
};
```

## Building

```bash
cargo build
```

## Security

- Each user can only vote once per poll
- Votes are recorded on the blockchain
- Results are transparent and verifiable

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
