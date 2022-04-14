# CaperLabs-QuickSwapCore-StakingRewards
Implementation of `Staking Rewards` Contract for the CasperLabs platform.

## Steps

There are 3 contracts in this folder

1. StakingRewards Contract
2. ReentrancyGuard Contract
3. RewardsDistributionRrecipient Contract
## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Install](#install)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Build All Smart Contracts](#build-all-smart-contracts)
    - [Individual Test Cases](#individual-test-cases)
    - [All Test Cases](#all-test-cases)
  - [Known contract hashes](#known-contract-hashes)

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update
# Install the command-line JSON processor
sudo apt install jq -y
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#Install the nightly version (by default stable toolchain is installed)
rustup install nightly
#Check that nightly toolchain version is installed(this will list stable and nightly versions)
rustup toolchain list
#Set rust nightly as default
rustup default nightly
# Install wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
#rust Version
rustup --version
#Install Cmake
 sudo apt-get -y install cmake
Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html
#cmake Version
cmake --version
#Installing the Casper Crates
cargo install cargo-casper
# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.ascr
sudo apt update
# Install the Casper client software
Install Casper-client
cargo +nightly install casper-client
# To check Casper Client Version
Casper-client --version
# Commands for help
casper-client --help
casper-client <command> --help
```

### Creating Keys

```bash
# Create keys
casper-client keygen <TARGET DIRECTORY>
```

### Usage

To run the Contracts make sure you are in the folder of your required contract.

#### Install

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

#### Build Individual Smart Contract

Run this command to build Smart Contract.

```
make build-contract
```

<br>**Note:** User needs to be in the desired project folder to build contracts and User needs to run `make build-contract` in every project to make wasms to avoid errors

#### Build All Smart Contracts

Run this command in main folder to build all Smart Contract.

```
make all
```

#### Individual Test Cases

Run this command to run Test Cases.

```
make test
```

<br>**Note:** User needs to be in the desired project folder to run test cases

#### All Test Cases

Run this command in main folder to run all contract's Test Cases.

```
make test
```

### Deploying StakingRewards contract manually

If you need to deploy the `StakingRewards contract` manually you need to pass the some parameters. Following is the command to deploy the `StakingRewards contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="rewards_distribution:Key='staking_rewards-package-hash'" \
    --session-arg="rewards_token:Key='erc20-package-hash'" \
    --session-arg="staking_token:Key='erc20-package-hash'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="StakingRewards-entry-point-methods"></a>

Following are the StakingReward's entry point methods.

- #### total_supply <a id="StakingRewards-total-supply"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### balance_of <a id="StakingRewards-balance-of"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| account        | Key  |

This method **returns** U256.

- #### last_time_reward_applicable <a id="StakingRewards-last-time-reward-applicable"></a>

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| account         | Key  |

This method **returns** U256.

- #### reward_per_token <a id="StakingRewards-reward-per-token"></a>

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |

This method **returns** U256.

- #### earned <a id="StakingRewards-earned"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| account        | Key  |

This method **returns** U256.

- #### stake_with_permit <a id="StakingRewards-stake-with-permit"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |
| deadline       | U256 |
| public_key     |String|
| signature      |String|

This method **returns** nothing.

- #### stake <a id="StakingRewards-stake"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** nothing.

- #### withdraw <a id="StakingRewards-withdraw"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** nothing.

- #### get_reward <a id="StakingRewards-get-reward"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.
- #### exit <a id="StakingRewards-exit"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### notify_reward_amount <a id="StakingRewards-notify-reward-amount"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| reward         | U256 |
| rewards_duration | U256 |

This method **returns** nothing.