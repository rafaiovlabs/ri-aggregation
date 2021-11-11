/// External uses
use serde::Deserialize;
/// Built-in uses
use std::time::Duration;
// Local uses
use zksync_types::network::Network;
use zksync_types::Address;

use crate::envy_load;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChainConfig {
    /// Proving / circuit data configuration.
    pub circuit: Circuit,
    /// L1 parameters configuration.
    pub eth: Eth,
    /// State keeper / block generating configuration.
    pub state_keeper: StateKeeper,
}

impl ChainConfig {
    pub fn from_env() -> Self {
        Self {
            circuit: envy_load!("circuit", "CHAIN_CIRCUIT_"),
            eth: envy_load!("eth", "CHAIN_ETH_"),
            state_keeper: envy_load!("state_keeper", "CHAIN_STATE_KEEPER_"),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Circuit {
    /// Path to the directory with the cryptographical keys. Relative to `$ZKSYNC_HOME`.
    pub key_dir: String,
    /// Actual supported block chunks sizes by verifier contract (determined by circuit size on setup boundaries).
    pub supported_block_chunks_sizes: Vec<usize>,
    /// Setup power needed to proof block of certain size (goes in the same order as the previous field,
    /// so both arrays can be `zip`ped together).
    pub supported_block_chunks_sizes_setup_powers: Vec<usize>,
    /// Sizes of blocks for aggregated proofs.
    pub supported_aggregated_proof_sizes: Vec<usize>,
    /// Setup power needed to create an aggregated proof for blocks of certain size (goes in the same order as the
    /// previous field, so both arrays can be `zip`ped together).
    pub supported_aggregated_proof_sizes_setup_power2: Vec<u32>,
    /// Depth of the Account Merkle tree.
    pub account_tree_depth: usize,
    /// Depth of the Balance Merkle tree.
    pub balance_tree_depth: usize,
}

impl Circuit {
    pub fn supported_aggregated_proof_sizes_with_setup_pow(&self) -> Vec<(usize, u32)> {
        self.supported_aggregated_proof_sizes
            .iter()
            .cloned()
            .zip(
                self.supported_aggregated_proof_sizes_setup_power2
                    .iter()
                    .cloned(),
            )
            .collect()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Eth {
    /// Name of the used Ethereum network, e.g. `localhost` or `rinkeby`.
    pub network: Network,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StateKeeper {
    /// Block sizes to be generated by server. Has to contain only values set in the `supported_block_chunks_sizes`,
    /// otherwise block will never be proven. This list can contain not all the values though: e.g. for local
    /// development usually a couple of smallest block sizes is enough.
    pub block_chunk_sizes: Vec<usize>,
    /// Time between two miniblocks created by mempool / block_proposer.
    pub miniblock_iteration_interval: u64,
    /// Maximum amount of miniblock iterations before sealing the block.
    pub miniblock_iterations: u64,
    /// Maximum amount of miniblock iterations in case of block containing a fast withdrawal request.
    pub fast_block_miniblock_iterations: u64,
    pub fee_account_addr: Address,
    pub aggregated_proof_sizes: Vec<usize>,
    pub max_aggregated_blocks_to_commit: usize,
    pub max_aggregated_blocks_to_execute: usize,
    pub block_commit_deadline: u64,
    pub block_prove_deadline: u64,
    pub block_execute_deadline: u64,
    pub max_aggregated_tx_gas: usize,
}

impl StateKeeper {
    /// Converts `self.miniblock_iteration_interval` into `Duration`.
    pub fn miniblock_iteration_interval(&self) -> Duration {
        Duration::from_millis(self.miniblock_iteration_interval)
    }

    pub fn block_commit_deadline(&self) -> Duration {
        Duration::from_secs(self.block_commit_deadline)
    }

    pub fn block_prove_deadline(&self) -> Duration {
        Duration::from_secs(self.block_prove_deadline)
    }

    pub fn block_execute_deadline(&self) -> Duration {
        Duration::from_secs(self.block_execute_deadline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::test_utils::{addr, set_env};

    fn expected_config() -> ChainConfig {
        ChainConfig {
            circuit: Circuit {
                key_dir: "keys/plonk-975ae851".into(),
                supported_block_chunks_sizes: vec![6, 30, 74, 150, 320, 630],
                supported_block_chunks_sizes_setup_powers: vec![21, 22, 23, 24, 25, 26],
                supported_aggregated_proof_sizes: vec![1, 5, 10, 20],
                supported_aggregated_proof_sizes_setup_power2: vec![22, 24, 25, 26],
                account_tree_depth: 32,
                balance_tree_depth: 11,
            },
            eth: Eth {
                network: "localhost".parse().unwrap(),
            },
            state_keeper: StateKeeper {
                block_chunk_sizes: vec![6, 30],
                miniblock_iteration_interval: 200,
                miniblock_iterations: 10,
                fast_block_miniblock_iterations: 5,
                fee_account_addr: addr("7857288e171c6159c5576d1bd9ac40c0c48a771c"),
                aggregated_proof_sizes: vec![1, 5],
                max_aggregated_blocks_to_commit: 3,
                max_aggregated_blocks_to_execute: 4,
                block_commit_deadline: 300,
                block_prove_deadline: 3_000,
                block_execute_deadline: 4_000,
                max_aggregated_tx_gas: 4_000_000,
            },
        }
    }

    #[test]
    fn from_env() {
        let config = r#"
CHAIN_CIRCUIT_KEY_DIR="keys/plonk-975ae851"
CHAIN_CIRCUIT_SUPPORTED_BLOCK_CHUNKS_SIZES="6,30,74,150,320,630"
CHAIN_CIRCUIT_SUPPORTED_BLOCK_CHUNKS_SIZES_SETUP_POWERS="21,22,23,24,25,26"
CHAIN_CIRCUIT_SUPPORTED_AGGREGATED_PROOF_SIZES="1,5,10,20"
CHAIN_CIRCUIT_SUPPORTED_AGGREGATED_PROOF_SIZES_SETUP_POWER2="22,24,25,26"
CHAIN_CIRCUIT_ACCOUNT_TREE_DEPTH="32"
CHAIN_CIRCUIT_BALANCE_TREE_DEPTH="11"
CHAIN_ETH_MAX_NUMBER_OF_WITHDRAWALS_PER_BLOCK="10"
CHAIN_ETH_NETWORK="localhost"
CHAIN_STATE_KEEPER_BLOCK_CHUNK_SIZES="6,30"
CHAIN_STATE_KEEPER_MINIBLOCK_ITERATION_INTERVAL="200"
CHAIN_STATE_KEEPER_MINIBLOCK_ITERATIONS="10"
CHAIN_STATE_KEEPER_FAST_BLOCK_MINIBLOCK_ITERATIONS="5"
CHAIN_STATE_KEEPER_FEE_ACCOUNT_ADDR="0x7857288e171c6159c5576d1bd9ac40c0c48a771c"
CHAIN_STATE_KEEPER_AGGREGATED_PROOF_SIZES="1,5"
CHAIN_STATE_KEEPER_MAX_AGGREGATED_BLOCKS_TO_COMMIT="3"
CHAIN_STATE_KEEPER_MAX_AGGREGATED_BLOCKS_TO_EXECUTE="4"
CHAIN_STATE_KEEPER_BLOCK_COMMIT_DEADLINE="300"
CHAIN_STATE_KEEPER_BLOCK_PROVE_DEADLINE="3000"
CHAIN_STATE_KEEPER_BLOCK_EXECUTE_DEADLINE="4000"
CHAIN_STATE_KEEPER_MAX_AGGREGATED_TX_GAS="4000000"
        "#;
        set_env(config);

        let actual = ChainConfig::from_env();
        assert_eq!(actual, expected_config());
    }

    /// Checks the correctness of the config helper methods.
    #[test]
    fn methods() {
        let config = expected_config();

        assert_eq!(
            config.state_keeper.miniblock_iteration_interval(),
            Duration::from_millis(config.state_keeper.miniblock_iteration_interval)
        );
    }
}
