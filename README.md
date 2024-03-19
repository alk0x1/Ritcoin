# Ritcoin (The Bitcoin maded in Rust)

Basically I will read the Bitcoin paper and recreate using Rust, while I comment and explain the process.

<b>Motivation: </b> Learn more about foundations of blockchain and sharp my skills in Rust.

**Documented the code here:** https://alk0x1.github.io/blockchain/2022/10/11/recreating-the-bitcoin-with-Rust.html


## CLI
```bash
Usage: ritcoin [OPTIONS] [NAME] [COMMAND]

Commands:
  start   			create a blockchain
  mine			    insert_block in the blockchain
  transaction		create a transaction and insert in the transaction pool
  wallet  			manage wallets
  help    			Print this message or the help of the given subcommand(s)

Arguments:
  [NAME]  Optional name to operate on

Options:
  -c, --config <FILE>  Sets a custom config file
  -d, --debug...       Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version

```

/* options
	- criar blockchain
	- minerar bloco com todas as transações validadas na pool
	- criar carteira
	-	criar transações e assina-las
*/