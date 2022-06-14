# ERC-1155 implementation in rust

## Installation
```
cargo install --path cli
erc1155-rs --help
```
Successful installation should print something like this
```
erc115-rs 
Application to interact with rust implementation of erc1155, contract state is stored locally
instead of on the blockchain

USAGE:
    erc1155-rs [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    account     Commands related to accounts in use
    approval    Commands related to setting approvals for an account, approved accounts can
                    control the assets of the account their approved for
    balances    Command allows to check the balance of a given token on a given account
    help        Print this message or the help of the given subcommand(s)
    transfer    Command that allows transferring tokens from one account to another

```