### startTime

Source: https://docs.surfpool.run/rpc/admin

Returns the system start time.

````APIDOC
## startTime

### Description
Returns the system start time.

### Method
GET

### Endpoint
/startTime

### Parameters
No parameters required

### Request Example
(No request body needed)

### Response
#### Success Response (200)
- **startTime** (integer) - The system start time as a Unix timestamp.

#### Response Example
```json
{
  "startTime": 1678886400
}
````

````

--------------------------------

### Deploy Program Action Example

Source: https://docs.surfpool.run/iac/svm/actions

Example of deploying a program using the `svm::deploy_program` action. It specifies the program to deploy and the authority/payer for the deployment. The action is part of the `svm` module and requires a program identifier obtained from an Anchor project.

```yaml
action "deploy" "svm::deploy_program" {
    description = "Deploy hello world program"
    program = svm::get_program_from_anchor_project("hello_world")
    authority = signer.authority
    payer = signer.payer  # Optional, defaults to authority
}

````

---

### Deploy Subgraph Actions in HCL

Source: https://docs.surfpool.run/iac/svm/actions

These examples show how to configure svm::deploy_subgraph actions for event indexing and PDA setup in Surfpool. They require a prior deploy action to provide program_id, program_idl, and slot. Inputs are the subgraph configurations, outputs are the deployed indexing subgraphs. Limitations include dependency on upstream deploy outputs and no error handling shown.

```hcl
action "transfer_event_subgraph" "svm::deploy_subgraph" {
    program_id = action.deploy.program_id
    program_idl = action.deploy.program_idl
    slot = action.deploy.slot
    event {
        name = "TransferEvent"
    }
}
action "account_index" "svm::deploy_subgraph" {
    program_id = action.deploy.program_id
    program_idl = action.deploy.program_idl
    slot = action.deploy.slot
    pda {
        type = "CustomAccount"
        instruction {
            name = "<instruction-using-this-account>"
            account_name = "<name-of-account-in-instruction>"
        }
        instruction {
            name = "<another-instruction-using-this-account>"
            account_name = "<name-of-account-in-instruction>"
        }
    }
}

```

---

### GET /getVersion

Source: https://docs.surfpool.run/rpc/network

Retrieves the version of the cluster.

```APIDOC
## GET /getVersion

### Description
Returns the version of the cluster.

### Method
GET

### Endpoint
/getVersion

### Parameters
No parameters required

### Response
#### Success Response (200)
- **version** (string) - The version string of the cluster.

#### Response Example
{
  "version": "1.16.0"
}
```

---

### Send SOL Action Example

Source: https://docs.surfpool.run/iac/svm/actions

Example of sending SOL using the `svm::send_sol` action. This action requires the amount, recipient, and signer details. It also allows specifying optional parameters like description and commitment level. The amount is converted from SOL to lamports using `svm::sol_to_lamports`.

```yaml
action "send_sol" "svm::send_sol" {
description = "Send some SOL"
amount = svm::sol_to_lamports(1)
signer = signer.caller
recipient = "zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV"
}
```

---

### GET /getBlocksWithLimit

Source: https://docs.surfpool.run/rpc/network

Retrieves a limited number of blocks starting from a given slot. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getBlocksWithLimit

### Description
Returns the blocks for a given range of slots with a limit. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/getBlocksWithLimit

### Parameters
#### Query Parameters
- **startSlot** (integer) - Required - The starting slot to query for blocks.
- **limit** (integer) - Required - The maximum number of blocks to return.
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **blocks** (array) - An array of block objects.

#### Response Example
{
  "blocks": [
    {
      "slot": 12345,
      "blockhash": "exampleBlockhash1"
    },
    {
      "slot": 12346,
      "blockhash": "exampleBlockhash2"
    }
  ]
}
```

---

### GET /getRecentPerformanceSamples

Source: https://docs.surfpool.run/rpc/network

Returns recent performance samples for validators on the network.

````APIDOC
## GET /getRecentPerformanceSamples

### Description
Returns the recent performance samples.

### Method
GET

### Endpoint
/getRecentPerformanceSamples

### Parameters
#### Query Parameters
- **limit** (integer) - Optional - The maximum number of samples to return.

### Request Example
```json
{
  "limit": 10
}
````

### Response

#### Success Response (200)

- **samples** (array[object]) - An array of performance sample objects.
  - **slot** (integer) - The slot number.
  - **leaderSlots** (integer) - Number of slots produced by the leader.
  - **totalSlotsProcessed** (integer) - Total slots processed.
  - **averageRcentage** (number) - Average percentage of slots produced.

#### Response Example

```json
{
  "samples": [
    {
      "slot": 1000,
      "leaderSlots": 1,
      "totalSlotsProcessed": 1,
      "averageRcentage": 1.0
    },
    {
      "slot": 1001,
      "leaderSlots": 1,
      "totalSlotsProcessed": 1,
      "averageRcentage": 1.0
    }
  ]
}
```

````

--------------------------------

### txtx Manifest Configuration YAML

Source: https://docs.surfpool.run/iac/language

Example of a txtx manifest file that defines runbook metadata, locations, and environment-specific configurations. Environments group related settings such as network identifiers and RPC URLs for deployment flexibility.

```YAML
---
name: protocol-deployment
runbooks:
  - name: Deploy Protocol
    description: This runbook deploys the protocol.
    location: ./deployment
environments:
  development:
    network_id: localnet
    rpc_api_url: http://localhost:8899
  devnet:
    network_id: devnet
    rpc_api_url: https://api.devnet.solana.com
  mainnet:
    network_id: mainnet
    rpc_api_url: https://api.mainnet-beta.solana.com
````

---

### GET /getInflationGovernor

Source: https://docs.surfpool.run/rpc/network

Retrieves the current inflation governor settings for the network.

````APIDOC
## GET /getInflationGovernor

### Description
Retrieves the inflation governor settings.

### Method
GET

### Endpoint
/getInflationGovernor

### Parameters
#### Query Parameters
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.

### Request Example
```json
{
  "commitment": "confirmed"
}
````

### Response

#### Success Response (200)

- **info** (object) - Inflation governor settings.
  - **tapsPerSlot** (integer)
  - **foundationTax** (number)
  - **maxAnnualInflation** (number)
  - **taxLamportsPerByteYear** (integer)

#### Response Example

```json
{
  "info": {
    "tapsPerSlot": 1000,
    "foundationTax": 0.05,
    "maxAnnualInflation": 0.03,
    "taxLamportsPerByteYear": 1000
  }
}
```

````

--------------------------------

### Configure SVM Setup Surfnet Action in TX Format

Source: https://docs.surfpool.run/iac/svm/actions

Demonstrates configuration of the svm::setup_surfnet action for initializing a Solana test environment. Sets up accounts with lamports and token balances, clones program accounts, configures program authorities, and deploys programs. Requires surfpool_run framework with signer configuration and variable definitions.

```tx
action "setup" "svm::setup_surfnet" {
    set_account {
        public_key = signer.caller.public_key
        lamports = 999999999
    }
    set_token_account {
        public_key = signer.caller.public_key
        token = "usdc"
        amount = 1000000
    }
    clone_program_account {
        source_program_id = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" // USDC program id
        destination_program_id = variable.my_program_id
    }
    set_program_authority {
        program_id = variable.my_program_id
        authority = signer.caller.public_key
    }
    deploy_program {
        program_id = variable.my_program_id
        binary_path = "./path/to/my_program.so"
    }
    reset_account {
        public_key = variable.some_pubkey
    }
}
````

---

### GET /getEpochSchedule

Source: https://docs.surfpool.run/rpc/network

Retrieves the epoch schedule, detailing the timing and duration of epochs.

````APIDOC
## GET /getEpochSchedule

### Description
Retrieves the epoch schedule.

### Method
GET

### Endpoint
/getEpochSchedule

### Parameters
No parameters required

### Request Example
(No request body or parameters needed)

### Response
#### Success Response (200)
- **epochSchedule** (object)
  - **firstNormalEpoch** (integer) - The epoch number that marks the start of normal epochs.
  - **firstReducedEpoch** (integer) - The epoch number that marks the start of reduced epochs.
  - **leaderScheduleSlotOffset** (integer) - The offset for the leader schedule slot.
  - **slotsPerEpoch** (integer) - The number of slots per epoch.

#### Response Example
```json
{
  "epochSchedule": {
    "firstNormalEpoch": 0,
    "firstReducedEpoch": 10,
    "leaderScheduleSlotOffset": 0,
    "slotsPerEpoch": 432000
  }
}
````

````

--------------------------------

### GET /getInflationRate

Source: https://docs.surfpool.run/rpc/network

Retrieves the current inflation rate for the network.

```APIDOC
## GET /getInflationRate

### Description
Retrieves the current inflation rate.

### Method
GET

### Endpoint
/getInflationRate

### Parameters
No parameters required

### Request Example
(No request body or parameters needed)

### Response
#### Success Response (200)
- **inflationRate** (object)
  - **total** (number) - Total inflation rate.
  - **validator** (number) - Validator inflation rate.
  - **foundation** (number) - Foundation inflation rate.

#### Response Example
```json
{
  "inflationRate": {
    "total": 0.035,
    "validator": 0.03,
    "foundation": 0.005
  }
}
````

````

--------------------------------

### Define and Use Signers in Surfpool Actions

Source: https://docs.surfpool.run/iac/language

Provides an example of defining a signer, in this case, using the 'svm::web_wallet' signer. The defined signer ('alice') can then be referenced in actions to handle transaction signing, integrating with the Surfpool Web UI for wallet interactions.

```surfpool-config
signer "alice" "svm::web_wallet" {
  expected_address = input.expected_address
}

action "my_tx" "svm::process_instructions" {
  ... instruction data
  signers = [signer.alice]
}

````

---

### Define Outputs in Surfpool

Source: https://docs.surfpool.run/iac/language

Shows how to use the 'output' command to display a specific value at the end of a runbook execution. This example calculates and outputs the sum of 4 + 4.

```surfpool-config
output "my_output" {
  description = "An example output. I hope it equals 8."
  value = 4 + 4
}

```

---

### GET /getEpochInfo

Source: https://docs.surfpool.run/rpc/network

Retrieves information about the current epoch. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getEpochInfo

### Description
Returns the epoch info. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/getEpochInfo

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **epoch** (integer) - The current epoch number.
- **slotIndex** (integer) - The index of the current slot within the epoch.
- **slotsInEpoch** (integer) - The total number of slots in the epoch.

#### Response Example
{
  "epoch": 10,
  "slotIndex": 500,
  "slotsInEpoch": 1000
}
```

---

### GET /getFirstAvailableBlock

Source: https://docs.surfpool.run/rpc/network

Retrieves the first available block in the chain.

```APIDOC
## GET /getFirstAvailableBlock

### Description
Returns the first available block.

### Method
GET

### Endpoint
/getFirstAvailableBlock

### Parameters
No parameters required

### Response
#### Success Response (200)
- **slot** (integer) - The slot number of the first available block.
- **blockhash** (string) - The blockhash of the first available block.

#### Response Example
{
  "slot": 1,
  "blockhash": "exampleGenesisBlockhash"
}
```

---

### Configure Addons with Default Values in Surfpool

Source: https://docs.surfpool.run/iac/language

Shows how to define addon configurations, specifying default values for fields that can be omitted in subsequent actions. This example sets defaults for 'network_id' and 'rpc_api_url' for the 'svm' addon.

```surfpool-config
addon "svm" {
  network_id = input.network_id
  rpc_api_url = input.rpc_api_url
}

```

---

### POST /svm/deploy_subgraph

Source: https://docs.surfpool.run/iac/svm/actions

Creates a live GraphQL database that indexes blockchain events for a specific program. The request includes the program ID, starting block height, and events to index, and returns deployment status.

```APIDOC
## POST /svm/deploy_subgraph

### Description
Creates a live GraphQL database that indexes blockchain events for a specific program. This command encodes the request and sends it to the Surfnet or Surfpool cloud services depending on the environment.

### Method
POST

### Endpoint
/svm/deploy_subgraph

### Parameters
#### Path Parameters
_None_

#### Query Parameters
_None_

#### Request Body
- **program_id** (string) - Required - The on‑chain program ID to be indexed.
- **start_block** (integer) - Required - Block height from which indexing should begin.
- **events** (array[string]) - Required - List of event names or signatures to index.

### Request Example
{
  "program_id": "ExampleProgram1111111111111111111111111111111",
  "start_block": 12345678,
  "events": ["Transfer", "Initialize"]
}

### Response
#### Success Response (200)
- **deployment_id** (string) - Identifier for the created subgraph deployment.
- **status** (string) - Deployment status, e.g., "queued", "running".

#### Response Example
{
  "deployment_id": "subgraph-abc123",
  "status": "queued"
}
```

---

### GET /getMinimumBalanceForRentExemption

Source: https://docs.surfpool.run/rpc/network

Retrieves the minimum balance required for rent exemption based on the provided account data length.

````APIDOC
## GET /getMinimumBalanceForRentExemption

### Description
Returns the minimum balance required for rent exemption.

### Method
GET

### Endpoint
/getMinimumBalanceForRentExemption

### Parameters
#### Query Parameters
- **dataLen** (integer) - Required - The account data length in bytes.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.

### Request Example
```json
{
  "dataLen": 100,
  "commitment": "finalized"
}
````

### Response

#### Success Response (200)

- **minimumBalance** (integer) - The minimum balance required for rent exemption.

#### Response Example

```json
{
  "minimumBalance": 5000000
}
```

````

--------------------------------

### GET /getSlotLeaders

Source: https://docs.surfpool.run/rpc/network

Retrieves the leaders for a specified range of slots.

```APIDOC
## GET /getSlotLeaders

### Description
Retrieves the leaders for a specified range of slots.

### Method
GET

### Endpoint
/getSlotLeaders

### Parameters
#### Query Parameters
- **startSlot** (integer) - Required - The starting slot to query for leaders.
- **limit** (integer) - Required - The maximum number of leaders to return.

### Request Example
```json
{
  "startSlot": 1000,
  "limit": 50
}
````

### Response

#### Success Response (200)

- **slotLeaders** (array[object]) - An array of objects, where each object contains a slot and its leader.
  - **slot** (integer) - The slot number.
  - **leader** (string) - The public key of the slot leader.

#### Response Example

```json
{
  "slotLeaders": [
    {
      "slot": 1000,
      "leader": "2831e487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f99"
    },
    {
      "slot": 1001,
      "leader": "abcde487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f00"
    }
  ]
}
```

````

--------------------------------

### GET /getTokenSupply

Source: https://docs.surfpool.run/rpc/accounts

Retrieves the total supply of a token, identified by its mint address.

```APIDOC
## GET /getTokenSupply

### Description
Returns the total supply of a token, given its mint address.

### Method
GET

### Endpoint
/websites/surfpool_run/getTokenSupply

### Parameters
#### Query Parameters
- **mint** (string) - Required - The public key of the token mint, as a base-58 encoded string.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.

### Request Example
```json
{
  "mint": "token_mint_pubkey",
  "commitment": "finalized"
}
````

### Response

#### Success Response (200)

- **supply** (string) - The total supply of the token.
- **decimals** (integer) - The number of decimals for the token.
- **uiAmount** (number) - The total supply as a floating-point number.
- **uiAmountString** (string) - The total supply as a string.

#### Response Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "apiVersion": "1.18.0",
      "slot": 123456789
    },
    "value": {
      "supply": "1000000000000",
      "decimals": 9,
      "uiAmount": 1000000.0,
      "uiAmountString": "1000000.0"
    }
  },
  "id": 1
}
```

````

--------------------------------

### Output Result Definition in txtx

Source: https://docs.surfpool.run/iac/language

Shows how to define output blocks that display results in the web UI. This example captures the status code from a previous HTTP action. Outputs help track execution results and debug workflows.

```txtx
output "status_code" {
  description = "This output will be displayed in the Outputs section of the web UI!"
  value = action.http_query.status_code
}
````

---

### GET /getTransaction

Source: https://docs.surfpool.run/rpc/transactions

Retrieves a transaction by its signature.

````APIDOC
## GET /getTransaction

### Description
Returns the transaction for a given signature.

### Method
GET

### Endpoint
/getTransaction

### Parameters
#### Query Parameters
- **signature** (string) - Required - The transaction signature to query, as a base-58 encoded string.
- **config.commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
- **config.maxSupportedTransactionVersion** (integer) - Optional - The maximum transaction version to support.

### Response
#### Success Response (200)
- **transaction** (object) - The transaction object.
  - **signatures** (array[string]) - An array of transaction signatures.
  - **message** (object) - The transaction message.
    - **recentBlockhash** (string) - The recent blockhash used in the transaction.
    - **instructions** (array[object]) - An array of instructions in the transaction.
    - **payer** (string) - The public key of the transaction payer.
  - **version** (integer) - The transaction version.

#### Response Example
```json
{
  "transaction": {
    "signatures": [
      "sig1...",
      "sig2..."
    ],
    "message": {
      "recentBlockhash": "bh1...",
      "instructions": [
        {
          "programIdIndex": 0,
          "accounts": [0, 1],
          "data": "base64data..."
        }
      ],
      "payer": "payerpubkey..."
    },
    "version": 0
  }
}
````

````

--------------------------------

### GET /getGenesisHash

Source: https://docs.surfpool.run/rpc/network

Retrieves the genesis hash of the blockchain.

```APIDOC
## GET /getGenesisHash

### Description
Returns the genesis hash.

### Method
GET

### Endpoint
/getGenesisHash

### Parameters
No parameters required

### Response
#### Success Response (200)
- **genesisHash** (string) - The genesis hash of the blockchain.

#### Response Example
{
  "genesisHash": "exampleGenesisHash"
}
````

---

### GET /getSupply

Source: https://docs.surfpool.run/rpc/accounts

Retrieves information about the current token supply. Supports excluding non-circulating accounts.

````APIDOC
## GET /getSupply

### Description
Returns information about the current token supply.

### Method
GET

### Endpoint
/getSupply

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
  - **excludeNonCirculatingAccountsList** (boolean) - Optional - Whether to exclude non-circulating accounts.

### Request Example
```json
{
  "config": {
    "commitment": "finalized",
    "excludeNonCirculatingAccountsList": true
  }
}
````

### Response

#### Success Response (200)

- **circulating** (integer) - The number of circulating supply tokens.
- **maximum** (integer) - The maximum supply of tokens.
- **total** (integer) - The total supply of tokens.

#### Response Example

```json
{
  "circulating": 1000000000,
  "maximum": 10000000000,
  "total": 5000000000
}
```

````

--------------------------------

### GET /getStakeMinimumDelegation

Source: https://docs.surfpool.run/rpc/transactions

Retrieves the minimum amount of SOL required to delegate to a validator.

```APIDOC
## GET /getStakeMinimumDelegation

### Description
Returns the stake minimum delegation.

### Method
GET

### Endpoint
/getStakeMinimumDelegation

### Parameters
#### Query Parameters
- **config.commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
- **config.minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **minLength** (integer) - The minimum number of lamports required for delegation.

#### Response Example
```json
{
  "minLength": 1000000000
}
````

````

--------------------------------

### Get Program from Native Project (Terraform)

Source: https://docs.surfpool.run/iac/svm/functions

Retrieves native program artifacts from a project. It takes the program name as input and outputs an object containing the program's IDL.

```terraform
variable "contract" {
    value = svm::get_program_from_native_project("my_program")
}
output "idl" {
    value = variable.contract.idl
}

````

---

### GET /getRecentPrioritizationFees

Source: https://docs.surfpool.run/rpc/network

Retrieves recent prioritization fees, optionally filtered by account public keys.

```APIDOC
## GET /getRecentPrioritizationFees

### Description
Returns the recent prioritization fees. Optionally takes an array of account public keys to filter the results.

### Method
GET

### Endpoint
/getRecentPrioritizationFees

### Parameters
#### Query Parameters
- **pubkeys** (array) - Optional - An array of account public keys to query for prioritization fees, as base-58 encoded strings.

### Response
#### Success Response (200)
- **fees** (object) - An object mapping account public keys to their recent prioritization fees.

#### Response Example
{
  "fees": {
    "account1": 1000,
    "account2": 1500
  }
}
```

---

### GET /getSlotLeader

Source: https://docs.surfpool.run/rpc/network

Retrieves the leader for the current or a specified slot.

````APIDOC
## GET /getSlotLeader

### Description
Retrieves the leader of the current slot or a specified slot.

### Method
GET

### Endpoint
/getSlotLeader

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
  - **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Request Example
```json
{
  "config": {
    "commitment": "finalized",
    "minContextSlot": 10000
  }
}
````

### Response

#### Success Response (200)

- **slotLeader** (string) - The public key of the slot leader.

#### Response Example

```json
{
  "slotLeader": "2831e487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f99"
}
```

````

--------------------------------

### GET /getBlockTime

Source: https://docs.surfpool.run/rpc/network

Retrieves the block time for a given slot.

```APIDOC
## GET /getBlockTime

### Description
Returns the block time for a given slot.

### Method
GET

### Endpoint
/getBlockTime

### Parameters
#### Query Parameters
- **slot** (integer) - Required - The slot to query for the block time.

### Request Example
```json
{
  "slot": 100000000
}
````

### Response

#### Success Response (200)

- **blockTime** (integer) - The block time in Unix timestamp format.

#### Response Example

```json
{
  "blockTime": 1678886400
}
```

````

--------------------------------

### GET /getBlock

Source: https://docs.surfpool.run/rpc/network

Retrieves the block for a given slot, with options for transaction details, encoding, and rewards.

```APIDOC
## GET /getBlock

### Description
Returns the block for a given slot.

### Method
GET

### Endpoint
/getBlock

### Parameters
#### Query Parameters
- **slot** (integer) - Required - The slot to query for the block.
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
  - **encoding** (string) - Optional - Encoding for transaction data.
  - **maxSupportedTransactionVersion** (integer) - Optional - The maximum transaction version to support.
  - **rewards** (boolean) - Optional - Whether to return rewards.
  - **transactionDetails** (string) - Optional - Level of transaction detail to return.

### Request Example
```json
{
  "slot": 100000000,
  "config": {
    "commitment": "finalized",
    "encoding": "json",
    "maxSupportedTransactionVersion": 0,
    "rewards": true,
    "transactionDetails": "full"
  }
}
````

### Response

#### Success Response (200)

- **block** (object) - The block details.
  - **previousBlockhash** (string)
  - **blockhash** (string)
  - **parentRoot** (string)
  - **stateRoot** (string)
  - **rewards** (array) - Array of reward objects if requested.
  - **transactions** (array) - Array of transaction objects.
  - **signatures** (array)
  - **blockTime** (integer)
  - **blockHeight** (integer)
  - **previousBlockHeight** (integer)
  - **status** (string)

#### Response Example

```json
{
  "block": {
    "previousBlockhash": "somehash",
    "blockhash": "anotherhash",
    "parentRoot": "root1",
    "stateRoot": "root2",
    "rewards": [
      {
        "pubkey": "validatorpubkey",
        "lamports": 1000,
        "postBalance": 5000000
      }
    ],
    "transactions": [
      {
        "signatures": ["sig1"],
        "message": {
          "recentBlockhash": "somehash",
          "accountKeys": ["pubkey1", "pubkey2"],
          "instructions": []
        }
      }
    ],
    "signatures": ["sig1"],
    "blockTime": 1678886400,
    "blockHeight": 100000000,
    "previousBlockHeight": 99999999,
    "status": "finalized"
  }
}
```

````

--------------------------------

### Create Token Account Instruction in SVM Config

Source: https://docs.surfpool.run/iac/svm/functions

This function generates serialized instruction bytes for creating a token account in SVM. It takes inputs like funding address, wallet address, token mint address, and token program ID. Output is the raw bytes for the instruction. Example demonstrates usage in a configuration file.

```config
action "call" "svm::process_instructions" {
    signers = [signer.caller]

    instruction {
        raw_bytes = svm::create_token_account_instruction(
            signer.caller.address, // funding address
            signer.caller.address, // wallet address
            variable.token_mint, // token mint address
            variable.token_program // token program id
        )
    }
}

````

---

### GET /getBlocks

Source: https://docs.surfpool.run/rpc/network

Retrieves blocks for a specified range of slots. Supports optional wrappers, commitment levels, and minimum context slots.

```APIDOC
## GET /getBlocks

### Description
Returns the blocks for a given range of slots. Supports optional wrapper for end slot or context configuration, commitment levels, and minimum context slot.

### Method
GET

### Endpoint
/getBlocks

### Parameters
#### Query Parameters
- **startSlot** (integer) - Required - The starting slot to query for blocks.
- **wrapper** (string) - Optional - Wrapper for end slot or context configuration.
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **blocks** (array) - An array of block objects.

#### Response Example
{
  "blocks": [
    {
      "slot": 12345,
      "blockhash": "exampleBlockhash1"
    }
  ]
}
```

---

### GET /getFeeForMessage

Source: https://docs.surfpool.run/rpc/transactions

Calculates the estimated transaction fee for a given message.

````APIDOC
## GET /getFeeForMessage

### Description
Returns the fee for a given message.

### Method
GET

### Endpoint
/getFeeForMessage

### Parameters
#### Query Parameters
- **message** (string) - Required - The message to calculate the fee for, as a base-64 encoded string.
- **config.commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
- **config.minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **fee** (integer) - The estimated transaction fee in lamports.

#### Response Example
```json
{
  "fee": 5000
}
````

````

--------------------------------

### GET /getSignaturesForAddress

Source: https://docs.surfpool.run/rpc/transactions

Retrieves a list of transaction signatures for a given address.

```APIDOC
## GET /getSignaturesForAddress

### Description
Returns the signatures for a given address.

### Method
GET

### Endpoint
/getSignaturesForAddress

### Parameters
#### Query Parameters
- **address** (string) - Required - The address to query for transaction signatures, as a base-58 encoded string.
- **before** (string) - Optional - Start searching backwards from this transaction signature.
- **limit** (integer) - Optional - The maximum number of signatures to return.
- **until** (string) - Optional - Search until this transaction signature.

### Response
#### Success Response (200)
- **signatures** (array[string]) - An array of transaction signatures.

#### Response Example
```json
{
  "signatures": [
    "sig1...",
    "sig2...",
    "sig3..."
  ]
}
````

````

--------------------------------

### Get Program from Anchor Project in SVM HCL

Source: https://docs.surfpool.run/iac/svm/functions

This function retrieves deployment artifacts (keypair, IDL, and binary) for an Anchor project program. It requires the program_name input, with optional paths for keypair, IDL, and binary files defaulting to standard Anchor locations. The output is an object containing the artifacts, dependent on proper Anchor project structure and file availability.

```hcl
variable "contract" {
    value = svm::get_program_from_anchor_project("my_program")
}
output "idl" {
    value = variable.contract.idl
}

````

---

### GET /getHighestSnapshotSlot

Source: https://docs.surfpool.run/rpc/network

Retrieves the slot number of the highest available snapshot.

```APIDOC
## GET /getHighestSnapshotSlot

### Description
Returns the highest snapshot slot.

### Method
GET

### Endpoint
/getHighestSnapshotSlot

### Parameters
No parameters required

### Response
#### Success Response (200)
- **highestSnapshotSlot** (integer) - The slot number of the highest snapshot.

#### Response Example
{
  "highestSnapshotSlot": 900
}
```

---

### GET /minimumLedgerSlot

Source: https://docs.surfpool.run/rpc/network

Returns the minimum ledger slot, representing the earliest slot for which ledger data is available.

````APIDOC
## GET /minimumLedgerSlot

### Description
Returns the minimum ledger slot.

### Method
GET

### Endpoint
/minimumLedgerSlot

### Parameters
No parameters required

### Request Example
(No request body or parameters needed)

### Response
#### Success Response (200)
- **minimumLedgerSlot** (integer) - The minimum ledger slot.

#### Response Example
```json
{
  "minimumLedgerSlot": 1
}
````

````

--------------------------------

### GET /getTokenAccountBalance

Source: https://docs.surfpool.run/rpc/accounts

Retrieves the balance of a specific token account, identified by its public key.

```APIDOC
## GET /getTokenAccountBalance

### Description
Returns the balance of a token account, given its public key.

### Method
GET

### Endpoint
/websites/surfpool_run/getTokenAccountBalance

### Parameters
#### Query Parameters
- **pubkey** (string) - Required - The public key of the token account to query, as a base-58 encoded string.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.

### Request Example
```json
{
  "pubkey": "token_account_pubkey",
  "commitment": "confirmed"
}
````

### Response

#### Success Response (200)

- **balance** (object) - The balance of the token account.
  - **amount** (string) - The amount of tokens in the account.
  - **decimals** (integer) - The number of decimals for the token.
  - **uiAmount** (number) - The amount of tokens as a floating-point number.
  - **uiAmountString** (string) - The amount of tokens as a string.

#### Response Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "context": {
      "apiVersion": "1.18.0",
      "slot": 123456789
    },
    "value": {
      "amount": "1000000",
      "decimals": 6,
      "uiAmount": 1.0,
      "uiAmountString": "1.0"
    }
  },
  "id": 1
}
```

````

--------------------------------

### GET /getLargestAccounts

Source: https://docs.surfpool.run/rpc/accounts

Returns the 20 largest accounts by lamport balance. Supports optional filtering.

```APIDOC
## GET /getLargestAccounts

### Description
Returns the 20 largest accounts by lamport balance.

### Method
GET

### Endpoint
/getLargestAccounts

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
  - **filter** (string) - Optional - The filter to apply to the largest accounts.

### Request Example
```json
{
  "config": {
    "commitment": "confirmed"
  }
}
````

### Response

#### Success Response (200)

- **accounts** (array) - An array of account objects, sorted by lamport balance in descending order.
  - **account** (string) - The public key of the account.
  - **lamports** (integer) - The number of lamports in the account.

#### Response Example

```json
{
  "accounts": [
    {
      "account": "<account_public_key>",
      "lamports": 1000000000
    }
  ]
}
```

````

--------------------------------

### GET /getMultipleAccounts

Source: https://docs.surfpool.run/rpc/accounts

Retrieves account information for multiple public keys in a single call. Supports filtering and pagination.

```APIDOC
## GET /getMultipleAccounts

### Description
Returns account information for multiple public keys in a single call.

### Method
GET

### Endpoint
/websites/surfpool_run/getMultipleAccounts

### Parameters
#### Query Parameters
- **pubkeys** (array[string]) - Required - An array of public keys to query, as base-58 encoded strings.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **length** (integer) - Optional - The length of the data slice.
- **offset** (integer) - Optional - The offset of the data slice.
- **encoding** (string) - Optional - The encoding for the account data.
- **minContextSlot** (integer) - Optional - The minimum context slot for the account info.

### Request Example
```json
{
  "pubkeys": ["pubkey1", "pubkey2"],
  "commitment": "finalized",
  "length": 100,
  "offset": 0,
  "encoding": "json",
  "minContextSlot": 12345
}
````

### Response

#### Success Response (200)

- **accounts** (array) - Array of account information objects.
  - **account.lamports** (integer) - The balance of the account in lamports.
  - **account.owner** (string) - The program that owns the account.
  - **account.executable** (boolean) - Whether the account is executable.
  - **account.rentEpoch** (integer) - The epoch at which the account will be rent-exempt.
  - **account.data** (any) - The data in the account, either as a string (for JSON and BSON) or a base-64 encoded string (for other encodings).

#### Response Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "value": [
      {
        "pubkey": "account_pubkey_1",
        "account": {
          "lamports": 1000000,
          "owner": "owner_program_id",
          "executable": false,
          "rentEpoch": 100,
          "data": {}
        }
      },
      {
        "pubkey": "account_pubkey_2",
        "account": {
          "lamports": 500000,
          "owner": "owner_program_id",
          "executable": false,
          "rentEpoch": 100,
          "data": {}
        }
      }
    ]
  },
  "id": 1
}
```

````

--------------------------------

### GET /getSlot

Source: https://docs.surfpool.run/rpc/network

Retrieves the current slot number. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getSlot

### Description
Returns the current slot. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/getSlot

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **slot** (integer) - The current slot number.

#### Response Example
{
  "slot": 123456
}
````

---

### GET /getSignatureStatuses

Source: https://docs.surfpool.run/rpc/transactions

Retrieves the signature statuses for a given array of transaction signatures.

````APIDOC
## GET /getSignatureStatuses

### Description
Returns the signature statuses for a given signature.

### Method
GET

### Endpoint
/getSignatureStatuses

### Parameters
#### Query Parameters
- **signatures** (array[string]) - Required - An array of transaction signatures to query, as base-58 encoded strings.
- **config.searchTransactionHistory** (boolean) - Optional - Whether to search the transaction history.

### Response
#### Success Response (200)
- **signatureStatus** (object) - An array of objects, each containing the status of a signature.
  - **signature** (string) - The transaction signature.
  - **slot** (integer) - The slot in which the transaction was processed.
  - **err** (object | null) - An error object if the transaction failed, otherwise null.
  - **confirmations** (integer | null) - The number of confirmations for the transaction.
  - **confirmationStatus** (string) - The confirmation status of the transaction ('processed', 'confirmed', 'finalized').

#### Response Example
```json
{
  "signature": "string",
  "slot": 12345,
  "err": null,
  "confirmations": 10,
  "confirmationStatus": "finalized"
}
````

````

--------------------------------

### GET /getBlockProduction

Source: https://docs.surfpool.run/rpc/network

Retrieves block production information, optionally filtered by validator identity and slot range.

```APIDOC
## GET /getBlockProduction

### Description
Retrieves block production information.

### Method
GET

### Endpoint
/getBlockProduction

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
  - **identity** (string) - Optional - Filter by validator identity, as a base-58 encoded string.
  - **firstSlot** (integer) - Optional - The first slot to include in the range.
  - **lastSlot** (integer) - Optional - The last slot to include in the range.

### Request Example
```json
{
  "config": {
    "commitment": "finalized",
    "identity": "2831e487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f99",
    "firstSlot": 1000,
    "lastSlot": 2000
  }
}
````

### Response

#### Success Response (200)

- **blockProduction** (object)
  - ** Slot ** (integer) - The slot number.
  - **leaderSlots** (integer) - Number of slots produced by the leader.
  - **totalSlotsProcessed** (integer) - Total slots processed.

#### Response Example

```json
{
  "blockProduction": {
    "1000": {
      "leaderSlots": 1,
      "totalSlotsProcessed": 1
    },
    "1001": {
      "leaderSlots": 1,
      "totalSlotsProcessed": 1
    }
  }
}
```

````

--------------------------------

### GET /getHealth

Source: https://docs.surfpool.run/rpc/node

Checks the overall health status of the cluster, indicating its operational readiness and stability.

```APIDOC
## GET /getHealth

### Description
Returns the health of the cluster.

### Method
GET

### Endpoint
/getHealth

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
None

### Request Example
None

### Response
#### Success Response (200)
- **health** (string) - The health status of the cluster (e.g., 'ok', 'degraded').

#### Response Example
{
  "health": "ok"
}
````

---

### GET /getBlockHeight

Source: https://docs.surfpool.run/rpc/network

Retrieves the current block height. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getBlockHeight

### Description
Returns the block height. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/getBlockHeight

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **blockHeight** (integer) - The current block height.

#### Response Example
{
  "blockHeight": 1000
}
```

---

### GET /getLeaderSchedule

Source: https://docs.surfpool.run/rpc/network

Retrieves the leader schedule for a given range or validator. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getLeaderSchedule

### Description
Returns the leader schedule. Supports optional options for slot or configuration, commitment levels, minimum context slots, and filtering by validator identity.

### Method
GET

### Endpoint
/getLeaderSchedule

### Parameters
#### Query Parameters
- **options** (string) - Optional - Wrapper for slot or configuration.
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.
- **identity** (string) - Optional - Filter by validator identity.

### Response
#### Success Response (200)
- **leaderSchedule** (object) - An object mapping slots to leader public keys.

#### Response Example
{
  "leaderSchedule": {
    "12345": "validatorPublicKey1",
    "12346": "validatorPublicKey2"
  }
}
```

---

### GET /getMaxRetransmitSlot

Source: https://docs.surfpool.run/rpc/network

Returns the maximum retransmit slot, indicating the furthest slot that has been retransmitted.

````APIDOC
## GET /getMaxRetransmitSlot

### Description
Returns the maximum retransmit slot.

### Method
GET

### Endpoint
/getMaxRetransmitSlot

### Parameters
No parameters required

### Request Example
(No request body or parameters needed)

### Response
#### Success Response (200)
- **maxRetransmitSlot** (integer) - The maximum retransmit slot.

#### Response Example
```json
{
  "maxRetransmitSlot": 150000000
}
````

````

--------------------------------

### GET /getAccountInfo

Source: https://docs.surfpool.run/rpc/accounts

Retrieves detailed information about a specific account using its public key. Supports configuration options for data retrieval.

```APIDOC
## GET /getAccountInfo

### Description
Returns detailed information about an account given its public key.

### Method
GET

### Endpoint
/getAccountInfo

### Parameters
#### Query Parameters
- **pubkey** (string) - Required - The public key of the account to query, as a base-58 encoded string.
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
  - **length** (integer) - Optional - The length of the data slice.
  - **offset** (integer) - Optional - The offset of the data slice.
  - **encoding** (string) - Optional - The encoding for the account data.
  - **minContextSlot** (integer) - Optional - The minimum context slot for the account info.

### Request Example
```json
{
  "pubkey": "<account_public_key_base58>",
  "config": {
    "encoding": "jsonParsed"
  }
}
````

### Response

#### Success Response (200)

- **context** (object) - Context information for the response.
  - **slot** (integer) - The slot number.
- **value** (object)
  - **data** (any) - The account data.
  - **executable** (boolean) - Whether the account is executable.
  - **lamports** (integer) - The number of lamports in the account.
  - **owner** (string) - The public key of the account's owner.
  - **rentEpoch** (integer) - The rent epoch of the account.
  - **space** (integer) - The size of the account's data in bytes.

#### Response Example

```json
{
  "context": {
    "slot": 1000
  },
  "value": {
    "data": {},
    "executable": false,
    "lamports": 1000000,
    "owner": "<owner_public_key>",
    "rentEpoch": 100,
    "space": 100
  }
}
```

````

--------------------------------

### GET /getTransactionCount

Source: https://docs.surfpool.run/rpc/network

Retrieves the total number of transactions. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getTransactionCount

### Description
Returns the transaction count. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/getTransactionCount

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **transactionCount** (integer) - The total number of transactions.

#### Response Example
{
  "transactionCount": 1000000
}
````

---

### Retrieve list entry with index function in Terraform

Source: https://docs.surfpool.run/iac/std/functions/list

Demonstrates the index function for retrieving elements from lists by position. Accepts a list (string, integer, or buffer arrays) and an integer index as inputs, returning the element at the specified zero-based index. This example retrieves the second element from a list using index 1 in Terraform.

```terraform
output "entry" {
    value = index(['a', 'b', 'c'], 1)
}
```

---

### Define Variable Block in txtx

Source: https://docs.surfpool.run/iac/language

This snippet shows how to define a variable block in the txtx language. Variables can hold static values or dynamically computed results based on other commands or functions. The example includes an editable field for UI adjustments.

```txtx
variable "query_path" {
  description = "An input that can be edited in the web UI!"
  value = "details"
  editable = true
}
```

---

### Define and Execute Actions in Surfpool

Source: https://docs.surfpool.run/iac/language

Demonstrates the definition of an action, 'deploy_hello_world', which uses the 'svm::deploy_program' capability. It includes inputs like 'program', 'authority', and 'payer', and defines an output for the resulting 'signature'.

```surfpool-config
action "deploy_hello_world" "svm::deploy_program" {
    description = "Deploy the hello_world program"
    program = svm::get_program_from_anchor_project("hello_world")
    authority = signer.authority
    payer = signer.payer
}
output "signature" {
  value = action.deploy_hello_world.signature
}

```

---

### GET /getLatestBlockhash

Source: https://docs.surfpool.run/rpc/network

Retrieves the hash of the latest block. Supports commitment levels and minimum context slots.

```APIDOC
## GET /getLatestBlockhash

### Description
Returns the latest blockhash. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/getLatestBlockhash

### Parameters
#### Query Parameters
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **blockhash** (string) - The blockhash of the latest block.

#### Response Example
{
  "blockhash": "latestExampleBlockhash"
}
```

---

### Get Associated Token Account (Terraform)

Source: https://docs.surfpool.run/iac/svm/functions

Computes the address of a wallet's associated token account for a given token mint. It requires the wallet address and optionally accepts the token mint address.

```terraform
variable "token_account" {
    value = svm::get_associated_token_account(signer.caller.address, "So11111111111111111111111111111111111111112")
}

```

---

### GET /getIdentity

Source: https://docs.surfpool.run/rpc/node

Retrieves the unique identity information of the cluster, which may include identifiers or cryptographic keys.

```APIDOC
## GET /getIdentity

### Description
Returns the identity of the cluster.

### Method
GET

### Endpoint
/getIdentity

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
None

### Request Example
None

### Response
#### Success Response (200)
- **identity** (object) - An object containing the cluster's identity details.

#### Response Example
{
  "identity": {
    "publicKey": "cluster_public_key_here",
    "clusterName": "Surfpool Cluster"
  }
}
```

---

### Send HTTP Request

Source: https://docs.surfpool.run/iac/std/actions/http

Makes an HTTP request to a specified URL and exports the response. Supports GET, HEAD, and POST methods.

````APIDOC
## POST /send_http_request

### Description
Makes an HTTP request to the given URL and exports the response. Supports GET, HEAD, and POST methods.

### Method
POST

### Endpoint
/send_http_request

### Parameters
#### Query Parameters
- **url** (string) - Required - The URL for the request. Supported schemes are http and https.
- **body** (string) - Optional - The request body as a string or json object.
- **method** (string) - Optional - The HTTP Method for the request. Allowed methods are GET, HEAD, and POST.
- **timeout_ms** (integer) - Optional - The request timeout in milliseconds.
- **headers** (object) - Optional - A map of request header field names and values.
- **pre_condition** (map) - Optional - Assertions evaluated before execution.
  - **behavior** (string) - Optional - Behavior if assertion fails: 'halt' (default), 'log', 'skip'.
  - **assertion** (any) - Required - The assertion to check.
- **post_condition** (map) - Optional - Assertions evaluated after execution.
  - **retries** (integer) - Optional - Number of retries if assertion fails.
  - **backoff_ms** (integer) - Optional - Wait time in milliseconds before retrying.
  - **behavior** (string) - Optional - Behavior if assertion fails: 'halt' (default), 'log', 'skip', 'continue'.
  - **assertion** (any) - Required - The assertion to check.

### Request Example
```json
{
  "action": "example",
  "action_type": "std::send_http_request",
  "inputs": {
    "url": "https://example.com"
  }
}
````

### Response

#### Success Response (200)

- **response_body** (string) - The response body returned as a string.
- **status_code** (integer) - The HTTP response status code.

#### Response Example

```json
{
  "status_code": 200,
  "response_body": "<html><body><h1>Example Domain</h1></body></html>"
}
```

````

--------------------------------

### Send HTTP Request using std::send_http_request

Source: https://docs.surfpool.run/iac/std/actions/http

The `std::send_http_request` function makes an HTTP request to a specified URL. It supports GET, HEAD, and POST methods, with optional body, timeout, and headers. Pre-conditions can assert conditions before execution, and post-conditions can handle retries and behaviors after execution. The output includes the response body and status code.

```tx
action "example" "std::send_http_request" {
  url = "https://example.com"
}

output "status" {
  value = action.example.status_code
}
// > status: 200

````

---

### GET /getMaxShredInsertSlot

Source: https://docs.surfpool.run/rpc/network

Returns the maximum shred insert slot, indicating the furthest slot for which shreds have been inserted.

````APIDOC
## GET /getMaxShredInsertSlot

### Description
Returns the maximum shred insert slot.

### Method
GET

### Endpoint
/getMaxShredInsertSlot

### Parameters
No parameters required

### Request Example
(No request body or parameters needed)

### Response
#### Success Response (200)
- **maxShredInsertSlot** (integer) - The maximum shred insert slot.

#### Response Example
```json
{
  "maxShredInsertSlot": 150000005
}
````

````

--------------------------------

### GET /getClusterNodes

Source: https://docs.surfpool.run/rpc/node

Retrieves a list of all nodes within the cluster, providing insights into the network's distributed components.

```APIDOC
## GET /getClusterNodes

### Description
Returns the cluster nodes.

### Method
GET

### Endpoint
/getClusterNodes

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
None

### Request Example
None

### Response
#### Success Response (200)
- **nodes** (array) - An array of cluster node objects.

#### Response Example
{
  "nodes": [
    {
      "nodeId": "node123",
      "ipAddress": "192.168.1.100",
      "status": "active"
    }
  ]
}
````

---

### GET /getTokenLargestAccounts

Source: https://docs.surfpool.run/rpc/accounts

Fetches the largest accounts for a given token mint. Requires the token mint's public key.

````APIDOC
## GET /getTokenLargestAccounts

### Description
Returns the largest accounts for a given token mint.

### Method
GET

### Endpoint
/getTokenLargestAccounts

### Parameters
#### Query Parameters
- **mint** (string) - Required - The public key of the token mint, as a base-58 encoded string.
- **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'

### Request Example
```json
{
  "mint": "<mint_public_key_base58>",
  "commitment": "confirmed"
}
````

### Response

#### Success Response (200)

- **accounts** (array) - An array of account objects, sorted by lamport balance in descending order.
  - **account** (string) - The public key of the account.
  - **amount** (string) - The token amount in the account.
  - **uiAmount** (number) - The token amount in UI format.
  - **uiAmountString** (string) - The token amount as a string.

#### Response Example

```json
{
  "accounts": [
    {
      "account": "<account_public_key>",
      "amount": "1000000000",
      "uiAmount": 1000000000.0,
      "uiAmountString": "1000000000"
    }
  ]
}
```

````

--------------------------------

### GET /isBlockhashValid

Source: https://docs.surfpool.run/rpc/network

Checks the validity of a given blockhash. Supports commitment levels and minimum context slots.

```APIDOC
## GET /isBlockhashValid

### Description
Returns the blockhash validity. Supports optional configuration, commitment levels, and minimum context slots.

### Method
GET

### Endpoint
/isBlockhashValid

### Parameters
#### Query Parameters
- **blockhash** (string) - Required - The blockhash to check, as a base-58 encoded string.
- **config** (object) - Optional - Configuration object for the query.
- **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
- **minContextSlot** (integer) - Optional - The minimum context slot for the context.

### Response
#### Success Response (200)
- **isValid** (boolean) - True if the blockhash is valid, false otherwise.

#### Response Example
{
  "isValid": true
}
````

---

### State Management Configuration in txtx

Source: https://docs.surfpool.run/iac/language

Configuration enabling state persistence for runbook executions. State tracking helps avoid unnecessary re-executions when there are no code or input changes. Includes specification of a storage directory for state files.

```YAML
runbooks:
  - name: Deploy Protocol
    location: ./deployment
    state:
      location: states
```

---

### GET /getVoteAccounts

Source: https://docs.surfpool.run/rpc/node

Fetches information about the vote accounts within the network, including details about their stake, uptime, and delinquent status.

```APIDOC
## GET /getVoteAccounts

### Description
Returns the vote accounts.

### Method
GET

### Endpoint
/getVoteAccounts

### Parameters
#### Path Parameters
None

#### Query Parameters
- **config** (object) - Configuration object for the query.
  - **commitment** (string) - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized' (optional)
  - **delinquentSlotDistance** (integer) - The distance in slots to consider a vote account delinquent (optional)
  - **keepUnstakedDelinquents** (boolean) - Whether to keep unstaked delinquent vote accounts (optional)
  - **votePubkey** (string) - Filter by vote account public key (optional)

#### Request Body
None

### Request Example
None

### Response
#### Success Response (200)
- **voteAccounts** (array) - An array of vote account objects.

#### Response Example
{
  "voteAccounts": [
    {
      "votePubkey": "vote_account_pubkey_1",
      "activatedStake": 1000000,
      "commission": 5,
      "lastTimestamp": 1678886400,
      "stakedTo": "staker_pubkey_1"
    }
  ]
}
```

---

### Create HTTP Request Action in txtx

Source: https://docs.surfpool.run/iac/language

Demonstrates creating an HTTP request action using the standard library send_http_request method. The URL is constructed using previously defined variables. Outputs from this action can be referenced elsewhere in the runbook.

```txtx
action "http_query" "std::send_http_request" {
  description = "This action will make a GET request to the specified URL!"
  url = "https://example.com/${variable.my_var.query_path}"
}
```

---

### GET /getProgramAccounts

Source: https://docs.surfpool.run/rpc/accounts

Retrieves program accounts owned by a specific program ID. Supports various configuration options for filtering and data retrieval.

````APIDOC
## GET /getProgramAccounts

### Description
Get program accounts owned by a specific program ID.

### Method
GET

### Endpoint
/getProgramAccounts

### Parameters
#### Query Parameters
- **programId** (string) - Required - The public key of the program, as a base-58 encoded string.
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
  - **length** (integer) - Optional - The length of the data slice.
  - **offset** (integer) - Optional - The offset of the data slice.
  - **encoding** (string) - Optional - The encoding for the account data.
  - **filters** (array) - Optional - Filters to apply to the program accounts. Each filter is a base58-encoded string representing an address or a specific filter type.
  - **minContextSlot** (integer) - Optional - The minimum context slot for the account info.
  - **sortResults** (boolean) - Optional - Whether to sort the results.
  - **withContext** (boolean) - Optional - Whether to include the context in the response.

### Request Example
```json
{
  "programId": "<program_id_base58>",
  "config": {
    "commitment": "finalized",
    "encoding": "jsonParsed",
    "filters": [
      {
        "memcmp": {
          "offset": 0,
          "bytes": "<bytes_base58>"
        }
      }
    ]
  }
}
````

### Response

#### Success Response (200)

- **accounts** (array) - An array of program account objects.
  - **account** (object)
    - **data** (any) - The account data.
    - **executable** (boolean) - Whether the account is executable.
    - **lamports** (integer) - The number of lamports in the account.
    - **owner** (string) - The public key of the account's owner.
    - **rentEpoch** (integer) - The rent epoch of the account.
    - **space** (integer) - The size of the account's data in bytes.
  - **context** (object) - Context information for the response (if `withContext` is true).
    - **slot** (integer) - The slot number.

#### Response Example

```json
{
  "accounts": [
    {
      "account": {
        "data": {},
        "executable": false,
        "lamports": 1000000,
        "owner": "<owner_public_key>",
        "rentEpoch": 100,
        "space": 100
      },
      "context": {
        "slot": 1000
      }
    }
  ]
}
```

````

--------------------------------

### GET /getBlockCommitment

Source: https://docs.surfpool.run/rpc/accounts

Retrieves commitment levels for a given block (slot). Requires the block slot number.

```APIDOC
## GET /getBlockCommitment

### Description
Returns commitment levels for a given block (slot).

### Method
GET

### Endpoint
/getBlockCommitment

### Parameters
#### Query Parameters
- **block** (integer) - Required - The slot to query for block commitment.

### Request Example
```json
{
  "block": 1000
}
````

### Response

#### Success Response (200)

- **commitment** (array) - An array of commitment levels for the block.
  - **slot** (integer) - The slot number.
  - **commitment** (string) - The commitment level ('processed', 'confirmed', or 'finalized').

#### Response Example

```json
{
  "commitment": [
    {
      "slot": 1000,
      "commitment": "finalized"
    }
  ]
}
```

````

--------------------------------

### Use jq function to query JSON in Terraform

Source: https://docs.surfpool.run/iac/std/functions/json

The example demonstrates using the jq function to extract a value from a JSON string. It shows how to query the 'message' field from a JSON object containing 'Hello world!'. The function takes a JSON object and a jq query string as inputs, returning the matching values.

```terraform
output "message" {
    value = jq("{ \"message\": \"Hello world!\" }", ".message")
}
````

---

### Convert U64 to Byte Array in SVM Config

Source: https://docs.surfpool.run/iac/svm/functions

The svm::u64 function converts a u64 integer to its byte array representation, ideal for PDA derivation seeds. It requires a single integer input. Output is a buffer containing the byte array. Example shows variable assignment in configuration syntax.

```config
variable "u64" {
    value = svm::u64(1000000000)
}

```

---

### Create Token Account Instruction (Terraform)

Source: https://docs.surfpool.run/iac/svm/functions

Generates raw instruction bytes for creating an associated token account. It requires funding and wallet addresses, and optionally accepts token mint and program IDs.

```terraform
svm::create_token_account_instruction(
  funding_address,
  wallet_address,
  token_mint_address,
  token_program_id
)

```

---

### GET /getTokenAccountsByDelegate

Source: https://docs.surfpool.run/rpc/accounts

Retrieves all SPL Token accounts associated with a given delegate's public key. Supports configuration options for data retrieval.

````APIDOC
## GET /getTokenAccountsByDelegate

### Description
Returns all SPL Token accounts by delegate.

### Method
GET

### Endpoint
/getTokenAccountsByDelegate

### Parameters
#### Query Parameters
- **delegate** (string) - Required - The public key of the delegate, as a base-58 encoded string.
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
  - **length** (integer) - Optional - The length of the data slice.
  - **offset** (integer) - Optional - The offset of the data slice.
  - **encoding** (string) - Optional - The encoding for the account data.
  - **minContextSlot** (integer) - Optional - The minimum context slot for the account info.

### Request Example
```json
{
  "delegate": "<delegate_public_key_base58>",
  "config": {
    "encoding": "jsonParsed"
  }
}
````

### Response

#### Success Response (200)

- **value** (array) - An array of token account objects.
  - **account** (object)
    - **data** (object) - Token account data.
    - **executable** (boolean) - Whether the account is executable.
    - **lamports** (integer) - The number of lamports in the account.
    - **owner** (string) - The public key of the account's owner.
    - **rentEpoch** (integer) - The rent epoch of the account.
    - **space** (integer) - The size of the account's data in bytes.
  - **mint** (string) - The public key of the token mint.
  - **pubkey** (string) - The public key of the token account.

#### Response Example

```json
{
  "value": [
    {
      "account": {
        "data": {
          "programId": "<program_id_base58>",
          "tokenAmount": {
            "amount": "1000000",
            "decimals": 6,
            "uiAmount": 1.0,
            "uiAmountString": "1.0"
          }
        },
        "executable": false,
        "lamports": 2039280,
        "owner": "<owner_public_key_base58>",
        "rentEpoch": 300,
        "space": 165
      },
      "mint": "<mint_public_key_base58>",
      "pubkey": "<token_account_public_key_base58>"
    }
  ]
}
```

````

--------------------------------

### Convert I64 to Byte Array in SVM Config

Source: https://docs.surfpool.run/iac/svm/functions

The svm::i64 function converts an i64 integer to its byte array representation, suitable for PDA seeds. It accepts a required integer value, including negatives. Returns a buffer of the byte array. Example illustrates variable declaration in config format.

```config
variable "i64" {
    value = svm::i64(-1000000000)
}

````

---

### Define Flows and Integrate with Addons in Surfpool

Source: https://docs.surfpool.run/iac/language

Illustrates how to define multiple 'flow' blocks, each with specific parameters like 'rpc_api_url'. These flows can then be used to configure addons, allowing actions within the addon to inherit flow-specific configurations.

```surfpool-config
// declare some flows
flow "solana" {
  rpc_api_url "https://api.mainnet-beta.solana.com"
}
flow "eclipse" {
  rpc_api_url "https://mainnetbeta-rpc.eclipse.xyz"
}
// declare the evm addon with
addon "svm" {
  network_id = "mainnet"
  rpc_api_url = flow.rpc_api_url
}
// the rest of the runbook can now use the svm addon without specifying chain_id or rpc_api_url,
// and will be executed once for each flow
...

```

---

### listPlugins

Source: https://docs.surfpool.run/rpc/admin

Returns a list of all currently loaded plugin names.

````APIDOC
## listPlugins

### Description
Returns a list of all currently loaded plugin names.

### Method
GET

### Endpoint
/listPlugins

### Parameters
No parameters required

### Request Example
(No request body needed)

### Response
#### Success Response (200)
- **plugins** (array[string]) - A list of loaded plugin names.

#### Response Example
```json
{
  "plugins": ["plugin1", "plugin2"]
}
````

````

--------------------------------

### GET /getTokenAccountsByOwner

Source: https://docs.surfpool.run/rpc/accounts

Retrieves all SPL Token accounts associated with a given owner's public key. Supports configuration options for data retrieval.

```APIDOC
## GET /getTokenAccountsByOwner

### Description
Returns all SPL Token accounts by owner.

### Method
GET

### Endpoint
/getTokenAccountsByOwner

### Parameters
#### Query Parameters
- **owner** (string) - Required - The public key of the account owner, as a base-58 encoded string.
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string | string | string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
  - **length** (integer) - Optional - The length of the data slice.
  - **offset** (integer) - Optional - The offset of the data slice.
  - **encoding** (string) - Optional - The encoding for the account data.
  - **minContextSlot** (integer) - Optional - The minimum context slot for the account info.

### Request Example
```json
{
  "owner": "<owner_public_key_base58>",
  "config": {
    "encoding": "jsonParsed"
  }
}
````

### Response

#### Success Response (200)

- **value** (array) - An array of token account objects.
  - **account** (object)
    - **data** (object) - Token account data.
    - **executable** (boolean) - Whether the account is executable.
    - **lamports** (integer) - The number of lamports in the account.
    - **owner** (string) - The public key of the account's owner.
    - **rentEpoch** (integer) - The rent epoch of the account.
    - **space** (integer) - The size of the account's data in bytes.
  - **mint** (string) - The public key of the token mint.
  - **pubkey** (string) - The public key of the token account.

#### Response Example

```json
{
  "value": [
    {
      "account": {
        "data": {
          "programId": "<program_id_base58>",
          "tokenAmount": {
            "amount": "1000000",
            "decimals": 6,
            "uiAmount": 1.0,
            "uiAmountString": "1.0"
          }
        },
        "executable": false,
        "lamports": 2039280,
        "owner": "<owner_public_key_base58>",
        "rentEpoch": 300,
        "space": 165
      },
      "mint": "<mint_public_key_base58>",
      "pubkey": "<token_account_public_key_base58>"
    }
  ]
}
```

````

--------------------------------

### Recover secp256k1 Public Key from Signature

Source: https://docs.surfpool.run/iac/std/functions/crypto

This example demonstrates how to use the secp256k1_recover function to recover a public key from a signature in a Terraform configuration. The function takes a message hash and a signature as inputs and returns the recovered public key as a string value that can be used in other parts of the configuration.

```hcl
output "recovered_public_key" {
    value = secp256k1_recover("0x6a2ce4b8aab1ef79aa1aa617cf6b72d7146857b83055e203b67c5177faef212c", "0x0165a85a1e64d7157d678d177bc8a9e6bfb8d750458d52a31c34abe1e56475b5eb62f183a5e6ddbced38fca93a8ff1c73b4ce66231e39392572af916b5303fbe12")
}

// > recovered_public_key: 0x03b3e0a76b292b2c83fc0ac14ae6160d0438ebe94e14bbb5b7755153628886e08e
````

---

### Perform logical AND using and_bool in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The and_bool function returns the binary AND of two boolean arguments. It accepts optional lhs and rhs boolean inputs and outputs a boolean result. Example shown demonstrates usage within a Terraform output block.

```HCL
output "my_bool" {
  value = false && true
}
// > my_bool: false
```

---

### Create Record Action Configuration

Source: https://docs.surfpool.run/iac/svm/actions

Shows how to configure the create_record action using HCL (main.tx) and YAML (main.yaml). The action requires a record name, data string, class public key, and owner. Outputs include signature, name, data, class, and public_key.

```HCL
action "my_record" "svm::create_record" {
    name = "my_record"
    data = "data string"
    class = action.create_my_class.public_key
    owner = signer.owner
}

```

```YAML
action "my_record" "svm::create_record" {
    name = "my_record"
    data = "data string"
    class = action.create_my_class.public_key
    owner = signer.owner
}

```

---

### Execute integer division using div in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The div function performs integer division of the left-hand operand by the right-hand operand, rounding any remainder down. It takes optional integer lhs (dividend) and rhs (divisor) inputs and returns an integer result. Example demonstrates division with negative divisor.

```HCL
output "my_int" {
  value = 11 / -3
}
// > my_int: -3
```

---

### Perform logical OR using or_bool in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The or_bool function returns the binary OR of two boolean arguments. It accepts optional lhs and rhs boolean inputs and outputs a boolean result. Example shows its usage within a Terraform output block.

```HCL
output "my_bool" {
  value = false || true
}
// > my_bool: true
```

---

### Evaluate less-than using lt in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The lt function returns true when the left-hand argument is less than the right-hand argument. It accepts null, integer, float, string, and boolean types for both inputs. Example illustrates a numeric less-than comparison in a Terraform output.

```HCL
output "is_lt" {
  value = 1 < 2
}
// > is_lt: true
```

---

### loadPlugin

Source: https://docs.surfpool.run/rpc/admin

Dynamically loads a new plugin into the runtime from a configuration file.

````APIDOC
## loadPlugin

### Description
Dynamically loads a new plugin into the runtime from a configuration file.

### Method
POST

### Endpoint
/loadPlugin

### Parameters
#### Request Body
- **configFile** (string) - Required - The path to the configuration file for the new plugin.

### Request Example
```json
{
  "configFile": "/path/to/new/plugin/config.json"
}
````

### Response

#### Success Response (200)

Indicates the plugin was successfully loaded.

#### Response Example

(No content)

````

--------------------------------

### Evaluate greater-than-or-equal using gte in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The gte function returns true when the left-hand argument is greater than or equal to the right-hand argument. It handles null, integer, float, string, and boolean inputs. Example demonstrates equality comparison in a Terraform output.

```HCL
output "is_gte" {
  value = 2 >= 2
}
// > is_gte: true
````

---

### Command Reference Usage in txtx

Source: https://docs.surfpool.run/iac/language

Illustrates referencing outputs from other commands using the command_type.reference_name.output_name pattern. This enables chaining operations where one command depends on another's result, ensuring proper execution order.

```txtx
variable "my_var" {
  description = variable.another_var.value // references the `value` output of a `variable` named `another_var`
  value = action.my_action.data // references the `data` output of an `action` named `my_action`
}
```

---

### Evaluate greater-than using gt in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The gt function returns true when the left-hand argument is greater than the right-hand argument. It supports null, integer, float, string, and boolean types for both inputs. Example shows a numeric comparison in a Terraform output.

```HCL
output "is_gt" {
  value = 2 > 1
}
// > is_gt: true
```

---

### Check equality using eq in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The eq function returns true if the two provided arguments are equal, otherwise false. It accepts a wide range of types (null, integer, float, string, bool, arrays, objects, etc.) for both lhs and rhs inputs. Example illustrates string equality within a Terraform output.

```HCL
output "is_eq" {
  value = "arg" == "arg"
}
// > is_eq: true
```

---

### Process Instructions for SVM Transactions

Source: https://docs.surfpool.run/iac/svm/actions

Encodes, signs, and broadcasts instructions to an SVM-compatible blockchain. It requires instruction details, signers, and RPC details. Optional pre- and post-conditions can be defined for execution control and error handling.

```tx
action "program_call" "svm::process_instructions" {
    description = "Invoke instructions"
    instruction {
        program_idl = variable.program.idl
        instruction_name = "initialize"
        instruction_args = [1]
        payer {
            public_key = signer.payer.public_key
        }
    }
    signers = [signer.caller]
}

```

---

### setIdentity

Source: https://docs.surfpool.run/rpc/admin

Sets the identity for the system using the provided keypair.

````APIDOC
## setIdentity

### Description
Sets the identity for the system using the provided keypair.

### Method
POST

### Endpoint
/setIdentity

### Parameters
#### Request Body
- **keypairFile** (string) - Required - Path to the keypair file to be used as the node's identity.
- **requireTower** (boolean) - Required - Boolean indicating if a tower is required for this identity.

### Request Example
```json
{
  "keypairFile": "/path/to/identity.json",
  "requireTower": true
}
````

### Response

#### Success Response (200)

Indicates the identity was successfully set.

#### Response Example

(No content)

````

--------------------------------

### Create Temporary Keypair File for Account Closure

Source: https://docs.surfpool.run/iac/svm/deployment-failure-recovery

Saves the ephemeral authority's base-58 encoded secret key to a JSON file for use in Solana CLI commands. This is required before closing accounts.

```JSON
[<byte0>, <byte1>, ..., <byte63>]
````

---

### Define and Use Variables in Surfpool

Source: https://docs.surfpool.run/iac/language

Demonstrates how to declare variables with descriptions, default values, and editability settings for user interaction in the Surfpool Web UI. Unspecified or false 'editable' fields make variables read-only.

```surfpool-config
variable "my_var" {
  description = "Enter your birthday"
  value = "MM/DD/YYYY"
  editable = true
}

```

---

### reloadPlugin

Source: https://docs.surfpool.run/rpc/admin

Reloads a runtime plugin with new configuration.

````APIDOC
## reloadPlugin

### Description
Reloads a runtime plugin with new configuration.

### Method
POST

### Endpoint
/reloadPlugin

### Parameters
#### Request Body
- **name** (string) - Required - The name of the plugin to reload.
- **configFile** (string) - Required - The path to the new configuration file for the plugin.

### Request Example
```json
{
  "name": "plugin_name",
  "configFile": "/path/to/new/config.json"
}
````

### Response

#### Success Response (200)

Indicates the plugin was successfully reloaded.

#### Response Example

(No content)

````

--------------------------------

### Retry SVM Program Deployment with Existing Accounts

Source: https://docs.surfpool.run/iac/svm/deployment-failure-recovery

Continues a failed deployment by providing the buffer account pubkey and ephemeral authority secret key. Requires prior execution logs containing account details.

```HCL
action "deploy" "svm::deploy_program" {
    description = "Deploy hello world program"
    program = svm::get_program_from_anchor_project("hello_world")
    authority = signer.authority
    payer = signer.payer  # Optional, defaults to authority
    buffer_account_pubkey = "<buffer-account-pubkey>"
    ephemeral_authority_secret_key = "<ephemeral-authority-secret-key>"
}
````

---

### exit

Source: https://docs.surfpool.run/rpc/admin

Immediately shuts down the RPC server.

```APIDOC
## exit

### Description
Immediately shuts down the RPC server.

### Method
POST

### Endpoint
/exit

### Parameters
No parameters required

### Request Example
No request body needed.

### Response
#### Success Response (200)
An empty response indicates successful shutdown.

#### Response Example
(No content)
```

---

### svm::send_token

Source: https://docs.surfpool.run/iac/svm/actions

This endpoint details the input parameters required for the svm::send_token action.

```APIDOC
## POST /api/svm/send_token

### Description
This endpoint describes the inputs for the svm::send_token action, used to encode, sign, and broadcast a token transaction.

### Method
POST

### Endpoint
/api/svm/send_token

### Parameters
#### Path Parameters
- None

#### Query Parameters
- None

#### Request Body
- `description` (string) - Optional - A description of the transaction.
- `amount` (integer) - Required - The amount of tokens to send, in base unit.
- `token` (string) - Required - The program address for the token being sent (token mint account).
- `recipient` (string) - Required - The SVM address of the recipient.
- `authority` (string) - Optional - The pubkey of the authority account for the token source. If omitted, the first signer will be used.
- `fund_recipient` (bool) - Optional - If true and the recipient token account does not exist, the action will create and fund it.
- `signers` (array[string]) - Required - A set of references to signer constructs used to sign the transaction.
- `commitment_level` (string) - Optional - The commitment level expected (processed, confirmed, finalized). Default is 'confirmed'.
- `rpc_api_url` (string) - Required - The URL to use when making API requests.
- `rpc_api_auth_token` (string) - Optional - The HTTP authentication token to include in the headers.
- `pre_condition` (map) - Optional - Pre-conditions are assertions evaluated before command execution.
- `post_condition` (map) - Optional - Post-conditions are assertions evaluated after command execution.

### Request Example
{
  "description": "Example transaction",
  "amount": 1000,
  "token": "token_address",
  "recipient": "recipient_address",
  "authority": "authority_pubkey",
  "fund_recipient": true,
  "signers": ["signer1", "signer2"],
  "commitment_level": "confirmed",
  "rpc_api_url": "https://rpc.example.com",
  "rpc_api_auth_token": "auth_token",
  "pre_condition": {
    "behavior": "log",
    "assertion": true
  },
  "post_condition": {
    "retries": 2,
    "backoff_ms": 500,
    "behavior": "halt",
    "assertion": true
  }
}

### Response
#### Success Response (200)
- `signature` (string) - The transaction computed signature.
- `recipient_token_address` (addon(svm::pubkey)) - The recipient token account address.
- `source_token_address` (addon(svm::pubkey)) - The source token account address.
- `token_mint_address` (addon(svm::pubkey)) - The token mint address.

```

---

### unloadPlugin

Source: https://docs.surfpool.run/rpc/admin

Unloads a runtime plugin.

````APIDOC
## unloadPlugin

### Description
Unloads a runtime plugin.

### Method
POST

### Endpoint
/unloadPlugin

### Parameters
#### Request Body
- **name** (string) - Required - The name of the plugin to unload.

### Request Example
```json
{
  "name": "plugin_name"
}
````

### Response

#### Success Response (200)

Indicates the plugin was successfully unloaded.

#### Response Example

(No content)

````

--------------------------------

### Action: svm::create_record

Source: https://docs.surfpool.run/iac/svm/actions

The create_record action is used to create a new record on the SVM. It requires parameters for name, data, class public key, and owner. Upon successful execution, it attaches outputs including the transaction signature, record name, data, class public key, and the record's public key.

```APIDOC
## Action svm::create_record

### Description
Creates a new record with the specified name, data, associated class, and owner. This action is part of the surfpool_run project and executes on the SVM.

### Method
Action

### Endpoint
svm::create_record

### Parameters
#### Request Body
- **name** (string) - Required - The name of the record to create.
- **data** (string) - Required - The data associated with the record.
- **class** (string) - Required - The public key of the associated class.
- **owner** (string) - Required - The owner of the record, typically the signer.

### Request Example
````

action "my_record" "svm::create_record" {
name = "my_record"
data = "data string"
class = action.create_my_class.public_key
owner = signer.owner
}

```

### Response
#### Success Response
- **signature** (string) - The transaction computed signature.
- **name** (string) - The name of the record.
- **data** (string) - The data of the record.
- **class** (string) - The public key of the associated class.
- **public_key** (string) - The public key of the created record.

#### Response Example
```

{
"signature": "transaction_signature_here",
"name": "my_record",
"data": "data string",
"class": "class_public_key_here",
"public_key": "record_public_key_here"
}

```

```

---

### POST /requestAirdrop

Source: https://docs.surfpool.run/rpc/transactions

Requests an airdrop of lamports to a specified public key.

````APIDOC
## POST /requestAirdrop

### Description
Requests an airdrop to a given address.

### Method
POST

### Endpoint
/requestAirdrop

### Parameters
#### Query Parameters
- **pubkey** (string) - Required - The public key of the account to receive the airdrop, as a base-58 encoded string.
- **lamports** (integer) - Required - The amount of lamports to airdrop.
- **config.commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'

### Response
#### Success Response (200)
- **transactionSignature** (string) - The signature of the airdrop transaction.

#### Response Example
```json
{
  "transactionSignature": "Fp44mE44k839e9dD18s411d1111111111111111111111111111111111111111111"
}
````

````

--------------------------------

### Inline Function Use in txtx

Source: https://docs.surfpool.run/iac/language

Shows usage of inline functions such as arithmetic operations and explicit function calls within variable definitions. Functions can reference command outputs and support modular logic without requiring separate command blocks.

```txtx
variable "one" {
  value = 1
}
variable "two" {
  value = 2
}
variable "addem_up" {
  value = variable.one + variable.two
}
output "add_some_more" {
  value = add_uint(variable.addem_up + variable.one, variable.two)
}
````

---

### Close Buffer Account and Transfer Funds

Source: https://docs.surfpool.run/iac/svm/deployment-failure-recovery

Closes the buffer account and transfers all lamports to a specified recipient address using the ephemeral authority keypair.

```Shell
solana program close --buffers --recipient <recipient_address> --authority temp-keypair.json --keypair temp-keypair.json
```

---

### setIdentityFromBytes

Source: https://docs.surfpool.run/rpc/admin

Sets the identity for the system using a keypair provided as a byte array.

````APIDOC
## setIdentityFromBytes

### Description
Sets the identity for the system using a keypair provided as a byte array.

### Method
POST

### Endpoint
/setIdentityFromBytes

### Parameters
#### Request Body
- **identityKeypair** (array[integer]) - Required - Byte array representing the identity keypair.
- **requireTower** (boolean) - Required - Boolean indicating if a tower is required for this identity.

### Request Example
```json
{
  "identityKeypair": [123, 45, 67, 89, ...],
  "requireTower": false
}
````

### Response

#### Success Response (200)

Indicates the identity was successfully set.

#### Response Example

(No content)

````

--------------------------------

### Configure signer blocks for squads deployment using SVM DSL

Source: https://docs.surfpool.run/iac/svm/signers

This snippet shows how to define initiator and deployer signers in the SVM configuration language for a squads deployment. It sets the expected address for the initiator and links the deployer to the multisig account public key, referencing input variables. No external dependencies are required beyond the SVM runtime.

```svm
signer \"initiator\" \"svm::web_wallet\" {\n    expected_address = input.initiator_address\n}\nsigner \"deployer\" \"svm::squads\" {\n    multisig_account_public_key = input.squads_multisig_address\n    initiator = signer.initiator\n}
````

---

### Create Class Action in YAML

Source: https://docs.surfpool.run/iac/svm/actions

Defines a class with specified attributes such as name, metadata, and permissions. The action requires a signer and outputs a signature, name, metadata, and public key.

```yaml
action "my_class" "svm::create_class" {
name = "my_class"
metadata = "metadata string"
is_permissioned = true
is_frozen = false
signer = signer.creator
}
```

---

### Convert Lamports to SOL (Terraform)

Source: https://docs.surfpool.run/iac/svm/functions

Converts a given amount of lamports to its equivalent in SOL. Accepts an integer lamport amount and returns the equivalent floating-point SOL value.

```terraform
output "sol" {
    value = svm::lamports_to_sol(1100000000)
}
// sol: 1.1

```

---

### Secret Key Signer Configuration

Source: https://docs.surfpool.run/iac/svm/signers

Configures a secret key signer for synchronous transaction signing. Supports multiple input methods including direct secret key, mnemonic phrase, derivation path, or keypair JSON file. Provides public key and address outputs for transaction verification.

```Configuration
signer "deployer" "svm::secret_key" {
    secret_key = input.secret_key
}
```

---

### Web Wallet Signer Configuration

Source: https://docs.surfpool.run/iac/svm/signers

Configures a web wallet signer for browser-based transaction signing. Allows connection with a specific expected address or permits any address to connect. Returns address and public key outputs for transaction processing.

```Configuration
signer "alice" "svm::web_wallet" {
    expected_address = "zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV"
}
```

---

### Convert SOL to Lamports (Terraform)

Source: https://docs.surfpool.run/iac/svm/functions

Converts a given amount of SOL to its equivalent in lamports. Accepts a floating-point or integer SOL amount and returns the equivalent lamport value.

```terraform
output "lamports" {
    value = svm::sol_to_lamports(1.1)
}
// lamports: 1100000000

```

---

### surfnet_cloneProgramAccount

Source: https://docs.surfpool.run/rpc/cheatcodes

Clones a program account from one program ID to another, including its associated program data.

````APIDOC
## surfnet_cloneProgramAccount

### Description
Clones a program account from one program ID to another, including its associated program data.

### Method
POST

### Endpoint
/websites/surfpool_run

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
- **sourceProgramId** (string) - Required - The public key of the source program to clone, as a base-58 encoded string.
- **destinationProgramId** (string) - Required - The public key of the destination program, as a base-58 encoded string.

### Request Example
```json
{
  "method": "surfnet_cloneProgramAccount",
  "params": {
    "sourceProgramId": "SOURCE_PROGRAM_ID",
    "destinationProgramId": "DESTINATION_PROGRAM_ID"
  }
}
````

### Response

#### Success Response (200)

- **result** (boolean) - Indicates if the program account cloning was successful.

#### Response Example

```json
{
  "result": true
}
```

````

--------------------------------

### rpcAddress

Source: https://docs.surfpool.run/rpc/admin

Returns the address of the RPC server.

```APIDOC
## rpcAddress

### Description
Returns the address of the RPC server.

### Method
GET

### Endpoint
/rpcAddress

### Parameters
No parameters required

### Request Example
(No request body needed)

### Response
#### Success Response (200)
- **address** (string) - The address of the RPC server.

#### Response Example
```json
{
  "address": "127.0.0.1:8080"
}
````

````

--------------------------------

### surfnet_setAccount

Source: https://docs.surfpool.run/rpc/cheatcodes

Sets or updates an account's properties including lamports, data, owner, and executable status.

```APIDOC
## surfnet_setAccount

### Description
Sets or updates an account's properties including lamports, data, owner, and executable status.

### Method
POST

### Endpoint
/websites/surfpool_run

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
- **pubkey** (string) - Required - The public key of the account to update, as a base-58 encoded string.
- **update** (object) - Required - The account data to update. Contains the new values for lamports, owner, executable status, rent epoch, and data.
  - **data** (string) - Optional - The new account data, as a hex encoded string.
  - **executable** (boolean) - Optional - Whether the account should be executable.
  - **lamports** (integer) - Optional - The new balance in lamports.
  - **owner** (string) - Optional - The new owner program ID, as a base-58 encoded string.
  - **rentEpoch** (integer) - Optional - The new rent epoch.

### Request Example
```json
{
  "method": "surfnet_setAccount",
  "params": {
    "pubkey": "ACCOUNT_PUBKEY",
    "update": {
      "lamports": 1000000,
      "executable": false,
      "owner": "OWNER_PROGRAM_ID",
      "rentEpoch": 1,
      "data": "HEX_ENCODED_DATA"
    }
  }
}
````

### Response

#### Success Response (200)

- **result** (boolean) - Indicates if the account update was successful.

#### Response Example

```json
{
  "result": true
}
```

````

--------------------------------

### POST /getInflationReward

Source: https://docs.surfpool.run/rpc/network

Returns the inflation reward for a given address or addresses, optionally for a specific epoch.

```APIDOC
## POST /getInflationReward

### Description
Returns the inflation reward for a given address.

### Method
POST

### Endpoint
/getInflationReward

### Parameters
#### Request Body
- **addresses** (array[string]) - Required - An array of public keys to query, as base-58 encoded strings.
- **config** (object) - Optional - Configuration object for the query.
  - **commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'.
  - **epoch** (integer) - Optional - The epoch number to query.
  - **minContextSlot** (integer) - Optional - The minimum context slot for the epoch.

### Request Example
```json
{
  "addresses": [
    "2831e487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f99",
    "abcde487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f00"
  ],
  "config": {
    "commitment": "finalized",
    "epoch": 10
  }
}
````

### Response

#### Success Response (200)

- **inflationReward** (array[object]) - An array of inflation reward objects for each address.
  - **address** (string) - The public key of the address.
  - **reward** (integer) - The inflation reward amount.
  - **postBalance** (integer) - The balance after reward.
  - **pubkey** (string) - The public key of the address.

#### Response Example

```json
{
  "inflationReward": [
    {
      "address": "2831e487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f99",
      "reward": 1000,
      "postBalance": 5000000,
      "pubkey": "2831e487860172e30f811777385c918a71929d4b52735b321143511f1a0d4f99"
    }
  ]
}
```

````

--------------------------------

### repairShredFromPeer

Source: https://docs.surfpool.run/rpc/admin

Repairs a shred from a peer node in the network.

```APIDOC
## repairShredFromPeer

### Description
Repairs a shred from a peer node in the network.

### Method
POST

### Endpoint
/repairShredFromPeer

### Parameters
#### Request Body
- **pubkey** (string) - Optional - The public key of the peer to repair from, as a base-58 encoded string.
- **slot** (integer) - Required - The slot of the shred to repair.
- **shredIndex** (integer) - Required - The index of the shred to repair.

### Request Example
```json
{
  "pubkey": "some_base58_pubkey",
  "slot": 12345,
  "shredIndex": 67890
}
````

### Response

#### Success Response (200)

Indicates the shred repair process was initiated.

#### Response Example

(No content)

````

--------------------------------

### Find PDA (Terraform)

Source: https://docs.surfpool.run/iac/svm/functions

Derives a Program Derived Address (PDA) using a program ID and an optional set of seeds. It returns an object containing the PDA address and its associated bump seed.

```terraform
variable "pda" {
    value = svm::find_pda("3bv3j4GvMPjvvBX9QdoX27pVoWhDSXpwKZipFF1QiVr6", ["data"])
}
output "pda" {
    value = std::encode_base58(variable.pda.pda)
}
output "bump" {
    value = variable.pda.bump_seed
}
// > pda: 4amHoWMBgLkPfM8Nq9ZP33Liq9FCuqrLoU1feejkdsUJ
// > bump: 252

````

---

### setLogFilter

Source: https://docs.surfpool.run/rpc/admin

Sets a filter for log messages in the system.

````APIDOC
## setLogFilter

### Description
Sets a filter for log messages in the system.

### Method
POST

### Endpoint
/setLogFilter

### Parameters
#### Request Body
- **filter** (string) - Required - The log filter string to apply.

### Request Example
```json
{
  "filter": "error"
}
````

### Response

#### Success Response (200)

Indicates the log filter was successfully set.

#### Response Example

(No content)

````

--------------------------------

### setStakedNodesOverrides

Source: https://docs.surfpool.run/rpc/admin

Sets the overrides for staked nodes using a specified path.

```APIDOC
## setStakedNodesOverrides

### Description
Sets the overrides for staked nodes using a specified path.

### Method
POST

### Endpoint
/setStakedNodesOverrides

### Parameters
#### Request Body
- **path** (string) - Required - Path to the file containing staked nodes overrides.

### Request Example
```json
{
  "path": "/path/to/staked_nodes.json"
}
````

### Response

#### Success Response (200)

Indicates the staked nodes overrides were successfully set.

#### Response Example

(No content)

````

--------------------------------

### Send SOL Token Action Configuration

Source: https://docs.surfpool.run/iac/svm/actions

Configures a SOL token transfer action using SVM blockchain functionality. The action defines transfer amount, signers, recipient address, and token details with automatic recipient funding. This configuration is used within the Surfpool project for handling SOL transactions.

```DSL/Configuration
action "send_sol" "svm::send_token" {
    description = "Send some SOL"
    amount = svm::sol_to_lamports(1)
    signers = [signer.caller]
    recipient = "zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV"
    token = "3bv3j4GvMPjvvBX9QdoX27pVoWhDSXpwKZipFF1QiVr6"
    fund_recipient = true
}
````

---

### Close Ephemeral Authority Account and Transfer Funds

Source: https://docs.surfpool.run/iac/svm/deployment-failure-recovery

Transfers all remaining lamports from the ephemeral authority account to a recipient address. Allows unfunded recipients and uses the same keypair as fee payer.

```Shell
solana transfer --from temp-keypair.json <recipient_address> ALL --allow-unfunded-recipient --fee-payer temp-keypair.json
```

---

### Generate Instruction Data from IDL Path in SVM HCL

Source: https://docs.surfpool.run/iac/svm/functions

This function creates encoded instruction data for program invocations using an IDL file path, ensuring type checking and serialization. It requires idl_path and instruction_name as inputs, with an optional arguments array; it depends on the IDL file being accessible. The output is a buffer of serialized data, limited by the accuracy and availability of the IDL file.

```hcl
output "data" {
    value = svm::get_instruction_data_from_idl_path("/path/to/idl.json", "my_instruction", ["arg1", "arg2"])
}
// > data: 0x95763bdcc47fa1b305000000776f726c64

```

---

### Retrieve System Program ID in SVM HCL

Source: https://docs.surfpool.run/iac/svm/functions

This function returns the public key identifier of the system program on SVM-compatible blockchains, which is a constant value. It requires no inputs and has no dependencies. The output is a pubkey type, essential for referencing the system program in blockchain transactions.

```hcl
output "system_program_id" {
    value = svm::system_program_id()
}
// > 11111111111111111111111111111111

```

---

### Addition in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The add function sums two integers and returns the integer result. Inputs are integers. Outputs an integer value; arithmetic is basic without overflow checks.

```HCL
output "my_sum" {
    value = 10 + 5
}
// > my_sum: 15
```

---

### surfnet_profileTransaction

Source: https://docs.surfpool.run/rpc/cheatcodes

Profiles a transaction to analyze compute units, account changes, and execution details.

````APIDOC
## surfnet_profileTransaction

### Description
Profiles a transaction to analyze compute units, account changes, and execution details.

### Method
POST

### Endpoint
/websites/surfpool_run

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
- **transactionData** (string) - Required - The transaction data to profile, as a base-64 encoded string.
- **tag** (string) - Optional - An optional tag to identify the profiling results.
- **config** (object) - Optional - Configuration for the profile result.
  - **depth** (string) - Optional - The depth of profiling ('transaction' or 'instruction').
  - **encoding** (string) - Optional - The encoding format for returned account data (e.g., 'base64', 'base58', 'jsonParsed').

### Request Example
```json
{
  "method": "surfnet_profileTransaction",
  "params": {
    "transactionData": "BASE64_ENCODED_TRANSACTION",
    "tag": "my_transaction_tag",
    "config": {
      "depth": "instruction",
      "encoding": "jsonParsed"
    }
  }
}
````

### Response

#### Success Response (200)

- **result** (object) - The profiling results for the transaction.

#### Response Example

```json
{
  "result": {
    "computeUnits": 10000,
    "accounts": [...],
    "instructions": [...]
  }
}
```

````

--------------------------------

### setRepairWhitelist

Source: https://docs.surfpool.run/rpc/admin

Sets the whitelist of nodes allowed to repair shreds.

```APIDOC
## setRepairWhitelist

### Description
Sets the whitelist of nodes allowed to repair shreds.

### Method
POST

### Endpoint
/setRepairWhitelist

### Parameters
#### Request Body
- **whitelist** (array[string]) - Required - A list of public keys (base-58 encoded strings) to set as the repair whitelist.

### Request Example
```json
{
  "whitelist": ["pubkey1", "pubkey2"]
}
````

### Response

#### Success Response (200)

Indicates the repair whitelist was successfully set.

#### Response Example

(No content)

````

--------------------------------

### getSecondaryIndexKeySize

Source: https://docs.surfpool.run/rpc/admin

Retrieves the size of the secondary index key for a given account.

```APIDOC
## getSecondaryIndexKeySize

### Description
Retrieves the size of the secondary index key for a given account.

### Method
GET

### Endpoint
/getSecondaryIndexKeySize

### Parameters
#### Query Parameters
- **pubkeyStr** (string) - Required - The public key of the account to get the secondary index key size for, as a base-58 encoded string.

### Request Example
````

GET /getSecondaryIndexKeySize?pubkeyStr=some_base58_pubkey

````

### Response
#### Success Response (200)
- **keySize** (integer) - The size of the secondary index key.

#### Response Example
```json
{
  "keySize": 32
}
````

````

--------------------------------

### surfnet_setTokenAccount

Source: https://docs.surfpool.run/rpc/cheatcodes

Sets or updates a token account's properties including balance, delegate, state, and authorities.

```APIDOC
## surfnet_setTokenAccount

### Description
Sets or updates a token account's properties including balance, delegate, state, and authorities.

### Method
POST

### Endpoint
/websites/surfpool_run

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
- **owner** (string) - Required - The public key of the token account owner, as a base-58 encoded string.
- **mint** (string) - Required - The public key of the token mint, as a base-58 encoded string.
- **update** (object) - Required - The token account data to update.
  - **amount** (integer) - Optional - The new token balance amount.
  - **closeAuthority** (string) - Optional - The new close authority.
  - **delegate** (string) - Optional - The new delegate account.
  - **delegatedAmount** (integer) - Optional - The new delegated amount.
  - **state** (string) - Optional - The new account state (e.g., 'initialized', 'frozen', 'closed').
  - **tokenProgram** (string) - Optional - The token program ID, as a base-58 encoded string.

### Request Example
```json
{
  "method": "surfnet_setTokenAccount",
  "params": {
    "owner": "OWNER_ADDRESS",
    "mint": "MINT_ADDRESS",
    "update": {
      "amount": 1000,
      "state": "initialized",
      "delegate": "DELEGATE_ADDRESS",
      "delegatedAmount": 500,
      "closeAuthority": "CLOSE_AUTHORITY_ADDRESS",
      "tokenProgram": "TOKEN_PROGRAM_ID"
    }
  }
}
````

### Response

#### Success Response (200)

- **result** (boolean) - Indicates if the token account update was successful.

#### Response Example

```json
{
  "result": true
}
```

````

--------------------------------

### POST /sendTransaction

Source: https://docs.surfpool.run/rpc/transactions

Sends a signed transaction to the cluster for processing.

```APIDOC
## POST /sendTransaction

### Description
Sends a transaction to the cluster.

### Method
POST

### Endpoint
/sendTransaction

### Parameters
#### Query Parameters
- **transaction** (string) - Required - The signed transaction, as a base-64 encoded string.
- **config.encoding** (string) - Optional - The encoding for the transaction.
- **config.maxRetries** (integer) - Optional - The maximum number of retries for the transaction.
- **config.minContextSlot** (integer) - Optional - The minimum context slot for the transaction.
- **config.preflightCommitment** (string) - Optional - The commitment level for the preflight check.
- **config.skipPreflight** (boolean) - Optional - Whether to skip the preflight check.

### Response
#### Success Response (200)
- **transactionSignature** (string) - The signature of the sent transaction.

#### Response Example
```json
{
  "transactionSignature": "Fp44mE44k839e9dD18s411d1111111111111111111111111111111111111111111"
}
````

````

--------------------------------

### Generate Instruction Data from IDL in SVM HCL

Source: https://docs.surfpool.run/iac/svm/functions

This function encodes instruction data for program invocations directly from provided IDL data, offering type checking and serialization. Inputs include the required IDL (as object or string) and instruction_name, with optional arguments array; no external dependencies beyond the IDL availability. The output is a buffer of encoded data, constrained by the IDL's completeness and correctness.

```hcl
output "data" {
    value = svm::get_instruction_data_from_idl(variable.idl, "my_instruction", ["arg1", "arg2"])
}
// > data: 0x95763bdcc47fa1b305000000776f726c64

````

---

### addAuthorizedVoterFromBytes

Source: https://docs.surfpool.run/rpc/admin

Adds an authorized voter to the system using a byte-encoded keypair.

````APIDOC
## addAuthorizedVoterFromBytes

### Description
Adds an authorized voter to the system using a byte-encoded keypair.

### Method
POST

### Endpoint
/addAuthorizedVoterFromBytes

### Parameters
#### Request Body
- **keypair** (array[integer]) - Required - Byte array representing the keypair for the authorized voter.

### Request Example
```json
{
  "keypair": [255, 128, 0, 1, ...]
}
````

### Response

#### Success Response (200)

Indicates the voter was successfully added.

#### Response Example

(No content)

````

--------------------------------

### Retrieve Default Pubkey in SVM HCL

Source: https://docs.surfpool.run/iac/svm/functions

This function generates and returns a default public key on SVM-compatible blockchains, represented as a constant value. It takes no inputs and has no dependencies. The output is a pubkey type, useful for default or placeholder key usage in deployments.

```hcl
output "default_pubkey" {
    value = svm::default_pubkey()
}
// > 11111111111111111111111111111111

````

---

### Signer web_wallet

Source: https://docs.surfpool.run/iac/svm/signers

The svm::web_wallet signer allows Runbook operators to sign transactions using their preferred browser-based wallet. It optionally verifies the expected address for security during connection.

```APIDOC
## Signer web_wallet

### Description
The `svm::web_wallet` signer facilitates transaction signing via a browser wallet connected to the Runbook execution.

### Method
Configuration Block

### Endpoint
signer [name] "svm::web_wallet"

### Parameters
#### Inputs
- **expected_address** (string) - Optional - The SVM address expected to connect. Omitting allows any address.

#### Outputs
- **address** (string) - The address of the account. Alias for public_key.
- **public_key** (string) - The address of the account.

### Request Example
```

signer "alice" "svm::web_wallet" {
expected_address = "zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV"
}

```

### Response
#### Success Response
- **address** (string) - Signer address.
- **public_key** (string) - Signer public key.

#### Response Example
{
  "address": "example_address",
  "public_key": "example_public_key"
}
```

---

### setPublicTpuAddress

Source: https://docs.surfpool.run/rpc/admin

Sets the public TPU (Transaction Processing Unit) address.

````APIDOC
## setPublicTpuAddress

### Description
Sets the public TPU (Transaction Processing Unit) address.

### Method
POST

### Endpoint
/setPublicTpuAddress

### Parameters
#### Request Body
- **publicTpuAddr** (string) - Required - The public TPU address as a string.

### Request Example
```json
{
  "publicTpuAddr": "tpu.example.com"
}
````

### Response

#### Success Response (200)

Indicates the public TPU address was successfully set.

#### Response Example

(No content)

````

--------------------------------

### FUNCTION eq

Source: https://docs.surfpool.run/iac/std/functions/operators

Compares two values for equality and returns a boolean result.

```APIDOC
## FUNCTION eq

### Description
Returns true if the left- and right-hand-side arguments are equal and false otherwise.

### Method
FUNCTION

### Endpoint
eq

### Parameters
#### Request Body
- **lhs** (null | integer | float | string | bool | array | object) - Optional - First value to compare.
- **rhs** (null | integer | float | string | bool | array | object) - Optional - Second value to compare.

### Request Example
{
  "lhs": "arg",
  "rhs": "arg"
}

### Response
#### Success Response (200)
- **value** (bool) - Result of the equality check.

### Response Example
{
  "value": true
}
````

---

### Delete Temporary Keypair File

Source: https://docs.surfpool.run/iac/svm/deployment-failure-recovery

Removes the temporary keypair file after completing account closure operations to ensure security.

```Shell
rm temp-keypair.json
```

---

### POST /simulateTransaction

Source: https://docs.surfpool.run/rpc/transactions

Simulates a transaction without sending it to the cluster, useful for estimating fees and checking for errors.

````APIDOC
## POST /simulateTransaction

### Description
Simulates a transaction.

### Method
POST

### Endpoint
/simulateTransaction

### Parameters
#### Query Parameters
- **transaction** (string) - Required - The transaction to simulate, as a base-64 encoded string.
- **config.addresses** (array[string]) - Optional - An array of account addresses to return, as base-58 encoded strings.
- **config.encoding** (string) - Optional - Encoding for the account data.
- **config.commitment** (string) - Optional - The commitment describes how finalized a block is at that point in time. Options are 'processed', 'confirmed', or 'finalized'
- **config.encoding** (string) - Optional - Encoding for the transaction data.
- **config.innerInstructions** (boolean) - Optional - Whether to include inner instructions in the simulation result.
- **config.minContextSlot** (integer) - Optional - Mincontextslot
- **config.replaceRecentBlockhash** (boolean) - Optional - Whether to replace the recent blockhash with a new one.
- **config.sigVerify** (boolean) - Optional - Whether to verify transaction signatures.

### Response
#### Success Response (200)
- **value** (object) - The simulation result.
  - **err** (object | null) - An error object if the simulation failed, otherwise null.
  - **logs** (array[string]) - An array of logs generated during the simulation.
  - **accounts** (array[object] | null) - An array of account data, if requested.
  - **unitsConsumed** (integer) - The number of compute units consumed during the simulation.
  - **signatures** (object) - An object containing signature verification results.

#### Response Example
```json
{
  "value": {
    "err": null,
    "logs": [],
    "accounts": null,
    "unitsConsumed": 1000,
    "signatures": {}
  }
}
````

````

--------------------------------

### Signer secret_key

Source: https://docs.surfpool.run/iac/svm/signers

The svm::secret_key signer enables synchronous transaction signing using a provided secret key, mnemonic, derivation path, or keypair file. It generates and outputs the corresponding public key and SVM address for the signer account.

```APIDOC
## Signer secret_key

### Description
The `svm::secret_key` signer can be used to synchronously sign a transaction using a secret key or derived keypair.

### Method
Configuration Block

### Endpoint
signer [name] "svm::secret_key"

### Parameters
#### Inputs
- **secret_key** (string) - Optional - The secret key used to sign messages and transactions.
- **mnemonic** (string) - Optional - The mnemonic phrase used to generate the secret key. Ignored if secret_key is provided.
- **derivation_path** (string) - Optional - The derivation path used to generate the secret key. Ignored if secret_key is provided.
- **keypair_json** (string) - Optional - A path to a keypair.json file containing the secret key. Ignored if secret_key or mnemonic is provided.
- **is_encrypted** (bool) - Optional - Coming soon.
- **password** (string) - Optional - Coming soon.

#### Outputs
- **public_key** (string) - The public key of the account generated from the secret key, mnemonic, or keypair file.
- **address** (string) - The SVM address generated from the secret key, mnemonic, or keypair file. Alias for public_key.

### Request Example
````

signer "deployer" "svm::secret_key" {
secret_key = input.secret_key
}

```

### Response
#### Success Response
- **public_key** (string) - Public key of the signer.
- **address** (string) - SVM address of the signer.

#### Response Example
{
  "public_key": "example_public_key",
  "address": "example_address"
}
```

---

### setPublicTpuForwardsAddress

Source: https://docs.surfpool.run/rpc/admin

Sets the public TPU forwards address.

````APIDOC
## setPublicTpuForwardsAddress

### Description
Sets the public TPU forwards address.

### Method
POST

### Endpoint
/setPublicTpuForwardsAddress

### Parameters
#### Request Body
- **publicTpuForwardsAddr** (string) - Required - The public TPU forwards address as a string.

### Request Example
```json
{
  "publicTpuForwardsAddr": "tpu-forwards.example.com"
}
````

### Response

#### Success Response (200)

Indicates the public TPU forwards address was successfully set.

#### Response Example

(No content)

````

--------------------------------

### FUNCTION div

Source: https://docs.surfpool.run/iac/std/functions/operators

Performs integer division of two numbers, rounding any remainder down.

```APIDOC
## FUNCTION div

### Description
Returns the integer division of the left-hand-side argument by the right-hand-side argument, rounding any remainder down to the nearest integer.

### Method
FUNCTION

### Endpoint
div

### Parameters
#### Request Body
- **lhs** (integer) - Optional - The dividend.
- **rhs** (integer) - Optional - The divisor.

### Request Example
{
  "lhs": 11,
  "rhs": -3
}

### Response
#### Success Response (200)
- **value** (integer) - Result of the integer division.

### Response Example
{
  "value": -3
}
````

---

### FUNCTION and_bool

Source: https://docs.surfpool.run/iac/std/functions/operators

Executes a logical AND operation between two boolean inputs and returns the result.

```APIDOC
## FUNCTION and_bool

### Description
Executes a logical AND operation between two boolean inputs.

### Method
FUNCTION

### Endpoint
and_bool

### Parameters
#### Request Body
- **lhs** (bool) - Optional - Left-hand side boolean value.
- **rhs** (bool) - Optional - Right-hand side boolean value.

### Request Example
{
  "lhs": false,
  "rhs": true
}

### Response
#### Success Response (200)
- **value** (bool) - Result of logical AND.

### Response Example
{
  "value": false
}
```

---

### Multiplication in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The multiply function multiplies two integers and returns the integer product. Both inputs are required integers. Outputs an integer; potential for overflow not specified in Terraform.

```HCL
output "my_product" {
    value = 10 * 5
}
// > my_product: 50
```

---

### Signer squads

Source: https://docs.surfpool.run/iac/svm/signers

The svm::squads signer supports transaction signing with Squads multisig accounts, deriving vaults and PDAs for proposals. It requires an initiator signer with Initiate permission and optional payer.

```APIDOC
## Signer squads

### Description
The `svm::squads` signer enables signing transactions using a Squads multisig, creating vault transactions and proposals.

### Method
Configuration Block

### Endpoint
signer [name] "svm::squads"

### Parameters
#### Inputs
- **multisig_account_public_key** (addon(svm::pubkey)) - Optional - The Squad multisig account pubkey from the settings page (not vault address).
- **create_key** (addon(svm::pubkey)) - Optional - The create key to derive the Squad multisig address.
- **vault_index** (integer) - Optional - The index of the vault to create (defaults to 0).
- **initiator** (string) - Required - Reference to a signer with Initiate permission in the Squads Multisig.
- **payer** (string) - Optional - Reference to a signer to pay for creation (defaults to initiator).
- **program_id** (addon(svm::pubkey)) - Optional - The Squad program ID (uses default if omitted).
- **squads_frontend_url** (string) - Optional - The Squads frontend URL (defaults to 'https://app.squads.so').

#### Outputs
No specific outputs mentioned beyond successful multisig signing.

### Request Example
No example provided in the documentation.

### Response
#### Success Response
Successful creation and signing of Squads vault transaction and proposal.

#### Response Example
{
  "status": "success",
  "multisig_address": "example_multisig"
}
```

---

### Compute Ripemd160 hash of a value

Source: https://docs.surfpool.run/iac/std/functions/hash

The ripemd160 function computes the Ripemd160 hash of a hex-encoded buffer or array of buffers. It returns a string representation of the hashed result. The input value is optional and should be properly encoded.

```hcl
output "hashed_data" {
    value = ripemd160(encode_hex("hello, world"))
}
// > hashed_data: 0XA3201F82FCA034E46D10CD7B27E174976E241DA2
```

---

### surfnet_getProfileResults

Source: https://docs.surfpool.run/rpc/cheatcodes

Retrieves all profiling results for transactions tagged with a specific identifier.

````APIDOC
## surfnet_getProfileResults

### Description
Retrieves all profiling results for transactions tagged with a specific identifier.

### Method
POST

### Endpoint
/websites/surfpool_run

### Parameters
#### Path Parameters
None

#### Query Parameters
None

#### Request Body
- **tag** (string) - Required - The tag to retrieve profiling results for.
- **config** (object) - Optional - Configuration for the profile result.
  - **depth** (string) - Optional - The depth of profiling ('transaction' or 'instruction').
  - **encoding** (string) - Optional - The encoding format for returned account data (e.g., 'base64', 'base58', 'jsonParsed').

### Request Example
```json
{
  "method": "surfnet_getProfileResults",
  "params": {
    "tag": "my_transaction_tag",
    "config": {
      "depth": "transaction",
      "encoding": "base64"
    }
  }
}
````

### Response

#### Success Response (200)

- **result** (array) - An array of profiling results for transactions matching the tag.

#### Response Example

```json
{
  "result": [
    {
      "transactionHash": "TX_HASH_1",
      "computeUnits": 10000,
      "accounts": [...]
    },
    {
      "transactionHash": "TX_HASH_2",
      "computeUnits": 12000,
      "accounts": [...]
    }
  ]
}
```

````

--------------------------------

### addAuthorizedVoter

Source: https://docs.surfpool.run/rpc/admin

Adds an authorized voter to the system.

```APIDOC
## addAuthorizedVoter

### Description
Adds an authorized voter to the system.

### Method
POST

### Endpoint
/addAuthorizedVoter

### Parameters
#### Request Body
- **keypairFile** (string) - Required - Path to the keypair file for the authorized voter.

### Request Example
```json
{
  "keypairFile": "/path/to/voter.json"
}
````

### Response

#### Success Response (200)

Indicates the voter was successfully added.

#### Response Example

(No content)

````

--------------------------------

### POST /svm/send_token

Source: https://docs.surfpool.run/iac/svm/actions

Transfers SOL tokens to a specified recipient using a token identifier. This endpoint expects a JSON payload with transaction details and returns a transaction identifier upon success.

```APIDOC
## POST /svm/send_token

### Description
Transfers SOL tokens to a specified recipient using a token identifier.

### Method
POST

### Endpoint
/svm/send_token

### Parameters
#### Path Parameters
_None_

#### Query Parameters
_None_

#### Request Body
- **description** (string) - Required - Description of the action.
- **amount** (integer) - Required - Amount in lamports (1 SOL = 1_000_000_000 lamports).
- **signers** (array[string]) - Required - List of signer identifiers, typically the caller.
- **recipient** (string) - Required - Destination wallet address.
- **token** (string) - Required - Token identifier for the SOL token.
- **fund_recipient** (boolean) - Optional - Whether to fund the recipient's account if needed.

### Request Example
{
  "description": "Send some SOL",
  "amount": 1000000000,
  "signers": ["caller"],
  "recipient": "zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV",
  "token": "3bv3j4GvMPjvvBX9QdoX27pVoWhDSXpwKZipFF1QiVr6",
  "fund_recipient": true
}

### Response
#### Success Response (200)
- **tx_id** (string) - Transaction identifier.

#### Response Example
{
  "tx_id": "5N8a...abc"
}
````

---

### FUNCTION or_bool

Source: https://docs.surfpool.run/iac/std/functions/operators

Executes a logical OR operation between two boolean inputs and returns the result.

```APIDOC
## FUNCTION or_bool

### Description
Executes a logical OR operation between two boolean inputs.

### Method
FUNCTION

### Endpoint
or_bool

### Parameters
#### Request Body
- **lhs** (bool) - Optional - Left-hand side boolean value.
- **rhs** (bool) - Optional - Right-hand side boolean value.

### Request Example
{
  "lhs": false,
  "rhs": true
}

### Response
#### Success Response (200)
- **value** (bool) - Result of logical OR.

### Response Example
{
  "value": true
}
```

---

### Less or Equal Comparison in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The lte function checks if the left-hand-side value is less than or equal to the right-hand-side and returns a boolean. Inputs can be integer, float, string, bool, or null. Outputs a boolean value; assumes inputs are comparable, otherwise results may be undefined.

```HCL
output "is_lte" {
  value = 2 <= 2
}
// > is_lte: true
```

---

### Encode String to Hex in Terraform

Source: https://docs.surfpool.run/iac/std/functions/hex

The encode_hex function converts any input string into its hexadecimal representation. It takes a required string input named 'value' and outputs the hex-encoded string. This function is useful in Terraform configurations for data encoding, with no dependencies or limitations mentioned.

```Terraform
output "encoded_hex" {
    value = encode_hex("hello, world")
}
// > encoded_hex: 68656C6C6F2C20776F726C64

```

---

### Modulo Operation in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The modulo function computes the remainder of dividing the left-hand-side integer by the right-hand-side integer and returns an integer. Inputs are required integers. Outputs an integer remainder; assumes positive integers, behavior may vary with negatives.

```HCL
output "my_mod" {
    value = 10 % 3
}
// > my_mod: 1
```

---

### FUNCTION gte

Source: https://docs.surfpool.run/iac/std/functions/operators

Determines if the left value is greater than or equal to the right value.

```APIDOC
## FUNCTION gte

### Description
Returns true if the left-hand-side argument is greater than or equal to the right-hand-side argument.

### Method
FUNCTION

### Endpoint
gte

### Parameters
#### Request Body
- **lhs** (null | integer | float | string | bool) - Optional - Left-hand side value.
- **rhs** (null | integer | float | string | bool) - Optional - Right-hand side value.

### Request Example
{
  "lhs": 2,
  "rhs": 2
}

### Response
#### Success Response (200)
- **value** (bool) - Result of the greater-than-or-equal comparison.

### Response Example
{
  "value": true
}
```

---

### Not Equal Comparison in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The neq function checks if the left- and right-hand-side values are not equal and returns a boolean. Inputs support integer, float, string, bool, or null types. Outputs a boolean; handles type comparisons where possible.

```HCL
output "is_neq" {
  value = "arg" != "arg"
}
// > is_neq: false
```

---

### Assert inequality with std::assert_ne in HCL

Source: https://docs.surfpool.run/iac/std/functions/assertions

The assert_ne function verifies that two values are not equal. It supports multiple data types for comparison including primitives and complex structures. Returns a standardized assertion result for validation purposes.

```hcl
output "assertion" {
    value = std::assert_ne(action.example.result, 1)
}
```

---

### FUNCTION lt

Source: https://docs.surfpool.run/iac/std/functions/operators

Determines if the left value is less than the right value.

```APIDOC
## FUNCTION lt

### Description
Returns true if the left-hand-side argument is less than the right-hand-side argument.

### Method
FUNCTION

### Endpoint
lt

### Parameters
#### Request Body
- **lhs** (null | integer | float | string | bool) - Optional - Left-hand side value.
- **rhs** (null | integer | float | string | bool) - Optional - Right-hand side value.

### Request Example
{
  "lhs": 1,
  "rhs": 2
}

### Response
#### Success Response (200)
- **value** (bool) - Result of the less-than comparison.

### Response Example
{
  "value": true
}
```

---

### Subtraction in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The minus function subtracts the right-hand-side integer from the left-hand-side and returns the integer result. Both inputs are required integers. Outputs an integer value; no handling for overflow or negative results beyond basic arithmetic.

```HCL
output "my_integer" {
  value = 10 - 6
}
// > my_integer: 4
```

---

### Compute SHA256 hash of a value

Source: https://docs.surfpool.run/iac/std/functions/hash

The sha256 function computes the SHA256 hash of a hex-encoded buffer or array of buffers. It returns a string representation of the hashed result. The input value is optional and must be properly encoded.

```hcl
output "hashed_data" {
    value = sha256(encode_hex("hello, world"))
}
// > hashed_data: 0x09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b
```

---

### Less Than Comparison in Terraform

Source: https://docs.surfpool.run/iac/std/functions/operators

The lt function checks if the left-hand-side value is less than the right-hand-side and returns a boolean. It supports types like integer, float, string, bool, or null for inputs. Outputs a boolean value; note that non-compatible types may not be comparable.

```HCL
output "is_lt" {
  value = 2 < 1
}
// > is_lt: false
```

---

### Compute Keccak256 hash of a string

Source: https://docs.surfpool.run/iac/std/functions/hash

The keccak256 function computes the Keccak256 hash of a string value. It returns a string representation of the hashed result. The input value is optional and can be a plain string without encoding.

```hcl
output "hashed_data" {
    value = keccak256("hello, world")
}
// > hashed_data: 0x09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b
```

---

### Assert less than or equal with std::assert_lte in HCL

Source: https://docs.surfpool.run/iac/std/functions/assertions

The assert_lte function validates that the left numeric value is less than or equal to the right. Restricted to integer and float data types. Returns a standardized assertion result for threshold checks.

```hcl
output "assertion" {
    value = std::assert_lte(action.example.result, 1)
}
```

---

### Assert less than with std::assert_lt in HCL

Source: https://docs.surfpool.run/iac/std/functions/assertions

The assert_lt function verifies that the left numeric value is less than the right. Accepts only integer and float types for comparison. Returns a standardized assertion result for range validation.

```hcl
output "assertion" {
    value = std::assert_lt(action.example.result, 1)
}
```

---

### FUNCTION gt

Source: https://docs.surfpool.run/iac/std/functions/operators

Determines if the left value is greater than the right value.

```APIDOC
## FUNCTION gt

### Description
Returns true if the left-hand-side argument is greater than the right-hand-side argument.

### Method
FUNCTION

### Endpoint
gt

### Parameters
#### Request Body
- **lhs** (null | integer | float | string | bool) - Optional - Left-hand side value.
- **rhs** (null | integer | float | string | bool) - Optional - Right-hand side value.

### Request Example
{
  "lhs": 2,
  "rhs": 1
}

### Response
#### Success Response (200)
- **value** (bool) - Result of the greater-than comparison.

### Response Example
{
  "value": true
}
```

---

### Assert equality with std::assert_eq in HCL

Source: https://docs.surfpool.run/iac/std/functions/assertions

The assert_eq function checks if two values are equal. It accepts any data type including null, numeric, string, boolean, or complex objects. Returns a standardized assertion result that can be used for testing validations.

```hcl
output "assertion" {
    value = std::assert_eq(action.example.result, 1)
}
```

---

### Assert greater than or equal with std::assert_gte in HCL

Source: https://docs.surfpool.run/iac/std/functions/assertions

The assert_gte function checks if the left numeric value is greater than or equal to the right. Limited to integer and float data types. Returns a standardized assertion result for boundary testing.

```hcl
output "assertion" {
    value = std::assert_gte(action.example.result, 1)
}
```

---

### removeAllAuthorizedVoters

Source: https://docs.surfpool.run/rpc/admin

Removes all authorized voters from the system.

```APIDOC
## removeAllAuthorizedVoters

### Description
Removes all authorized voters from the system.

### Method
POST

### Endpoint
/removeAllAuthorizedVoters

### Parameters
No parameters required

### Request Example
(No request body needed)

### Response
#### Success Response (200)
Indicates all voters were successfully removed.

#### Response Example
(No content)
```

---

### Assert greater than with std::assert_gt in HCL

Source: https://docs.surfpool.run/iac/std/functions/assertions

The assert_gt function validates that the left numeric value is greater than the right. Restricted to integer and float types only. Returns a standardized assertion result for numeric comparisons.

```hcl
output "assertion" {
    value = std::assert_gt(action.example.result, 1)
}
```

=== COMPLETE CONTENT === This response contains all available snippets from this library. No additional content exists. Do not make further requests.
