# solana-anchor-tutorial

## Setup and CLI usage

### 1. Prerequisites

- `mocha` and `@project-serum/anchor` must be globally installed via `npm i -g mocha @project-serum/anchor`. Local installation doesn't work.
- Run a local Solana node using `solana-test-validator`. Kill the validator and reuse the terminal for anchor commands.

### 2. Intializing project

1. Create project using `anchor init t1_basics`
2. Build project and generate IDL using `anchor build`
3. Deploy using `anchor deploy`
4. Add new program to workspace using `anchor new program_name`

### 3. Testing

A test script is automatically created in tests folder.

- Run all tests using `anchor test`
- Run specific test using `anchor test tests/t2_accounts.js`

