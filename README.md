# DAO Voting System

## Installation

### Prerequisites

1. **Rust:** Install the latest stable version from [rust-lang.org](https://www.rust-lang.org/).
2. **Solana CLI:** Follow instructions on [Solana's documentation](https://docs.solana.com/cli/install-solana-cli-tools) to install the CLI tool.
3. **Anchor:** Install Anchor CLI using `cargo install anchor-cli`.
4. **Node.js and npm/yarn**: Install from [nodejs.org](https://nodejs.org/).

### Steps

1. **Clone the repository:**

    ```sh
    git clone https://github.com/your-repo/dao_voting.git
    cd dao_voting
    ```

2. **Install dependencies:**

    ```sh
    npm install
    ```

3. **Build and deploy the program:**

    ```sh
    anchor build
    anchor deploy
    ```

4. **Run tests:**

    ```sh
    anchor test
    ```

## Usage

### Initialize State

```typescript
await program.rpc.initialize({
  accounts: {
    state: state.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  },
  signers: [state],
});

