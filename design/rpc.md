# RPC Server API Documentation

This document provides detailed information about the RPC methods available in the `rpc.rs` file of the `ritcoin` application.

## Overview

The RPC server is set up to handle various blockchain-related methods. These methods are organized into different categories: block methods, transaction methods, wallet methods, and blockchain methods. Below is a comprehensive list of the available RPC methods, their parameters, and their responses.

---

## Block Methods

### 1. `insert_new_block`

- **Description**: Inserts a new block into the blockchain.
- **Method**: `insert_new_block`
- **Parameters**: None
- **Response**:
	```json
	{
		"jsonrpc": "2.0",
		"result": "Block inserted.",
		"id": 1
	}
	```
---

### 2. `get_last_block_hash`

- **Description**: Retrieves the hash of the last block in the blockchain.
- **Method**: `get_last_block_hash`
- **Parameters**: None
- **Response**:
	```json
	{
		"jsonrpc": "2.0",
		"result": "<hash>",
		"id": 1
	}
	```
---

### 3. `show_block_info`

- **Description**: Displays information about a specific block.
- **Method**: `show_block_info`
- **Parameters**:
  - `index` (usize): The index of the block to display.
- **Response**:
	```json
	{
		"jsonrpc": "2.0",
		"result": "Info for block <index> displayed.",
		"id": 1
	}
	```
---

### 4. `show_transactions_in_a_block`

- **Description**: Shows all transactions in a specified block.
- **Method**: `show_transactions_in_a_block`
- **Parameters**:
  - `index` (usize): The index of the block to display transactions for.
- **Response**:
	```json
	{
	  "jsonrpc": "2.0",
	  "result": "<transactions_info>",
	  "id": 1
	}
	```
  - `<transactions_info>`: Detailed information about each transaction in the block.

---

## Transaction Methods

### 1. `insert_transaction_in_pool`

- **Description**: Inserts a new transaction into the transaction pool.
- **Method**: `insert_transaction_in_pool`
- **Parameters**: None (Transaction details are hardcoded in the example)
- **Response**:
	```json
	{
	  "jsonrpc": "2.0",
	  "result": "Transaction inserted into pool.",
	  "id": 1
	}
	```
	If an error occurs:
	```json
	{
	  "jsonrpc": "2.0",
	  "result": "Error creating transaction: <error_message>",
	  "id": 1
	}
	```
---

## Wallet Methods

### 1. `create_wallet`

- **Description**: Creates a new wallet and saves it to a specified filename.
- **Method**: `create_wallet`
- **Parameters**:
  - `filename` (String): The filename to save the wallet as.
- **Response**:
  ```json
  {
    "jsonrpc": "2.0",
    "result": "Wallet created and saved as <filename>",
    "id": 1
  }
---

## Blockchain Methods

### 1. `get_blockchain_data`

- **Description**: Retrieves detailed information about the blockchain, including the transaction pool and UTXOs.
- **Method**: `get_blockchain_data`
- **Parameters**: None
- **Response**:
  ```json
	{
		"jsonrpc": "2.0",
		"result": "<blockchain_data>",
		"id": 1
	}
  ```
  - `<blockchain_data>`: Detailed information about the transaction pool and UTXOs.
---

### 2. `show_all_block_hashes`

- **Description**: Displays the hashes of all blocks in the blockchain.
- **Method**: `show_all_block_hashes`
- **Parameters**: None
- **Response**:
	```json
	{
	  "jsonrpc": "2.0",
	  "result": "<blocks_info>",
	  "id": 1
	}
	```
  - `<blocks_info>`: A list of all block hashes in the blockchain.

---

