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

## Program structure

```
```

## Solala basics

### Data storage

Data is stored on Solana in two ways:

1. Accounts:
    - They hold persistent arbitary data, similar to files in Linux.
    - Also contain metadata, which controls write access.
    - Accounts need to pay a rent otherwise they are purged. Paying tokens above an amount makes the account rent exempt.
    - Design pattern: Store user data on accounts. Try to keep programs stateless.

2.