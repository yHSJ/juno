use std::net::IpAddr;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RunOptions {
    pub verbosity: Verbosity,
    pub node_id: NodeId,
    pub host: IpAddr,
    pub port: u16,
    pub peers: Vec<Host>,
    pub api_host: IpAddr,
    pub api_port: u16,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
    pub monitoring_port: Option<u16>,
    pub hydra_signing_key: PathBuf,
    pub hydra_verification_keys: Vec<PathBuf>,
    pub persistence_dir: PathBuf,
    pub chain_config: ChainConfig,
    pub ledger_config: LedgerConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Verbosity {
    Quiet,
    Verbose,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Host(String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChainConfig {
    Offline(OfflineChainConfig),
    Direct(DirectChainConfig),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OfflineChainConfig {
    pub initial_utxo_file: PathBuf,

    pub ledger_genesis_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectChainConfig {
    pub network_id: NetworkId,
    pub node_socket: PathBuf,
    pub hydra_scripts_tx_id: TxId,
    pub cardano_signing_key: PathBuf,
    pub cardano_verification_keys: Vec<PathBuf>,
    pub start_chain_from: Option<ChainPoint>,
    pub contestation_period: ContestationPeriod,
    pub deposit_deadline: DepositDeadline,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LedgerConfig {
    pub cardano_ledger_protocol_parameters_file: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NetworkId {
    Mainnet,
    Testnet(u8),
}

impl ToString for NetworkId {
    fn to_string(&self) -> String {
        match self {
            NetworkId::Mainnet => "0".to_string(),
            NetworkId::Testnet(magic) => magic.to_string(),
        }
    }
}
pub type TxId = String;
pub type ChainPoint = String;
pub type ContestationPeriod = u64;
pub type DepositDeadline = u64;

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            verbosity: Verbosity::Verbose,
            node_id: NodeId("hydra-node-1".to_string()),
            host: "127.0.0.1".parse().unwrap(),
            port: 5001,
            peers: Vec::new(),
            api_host: "127.0.0.1".parse().unwrap(),
            api_port: 4001,
            tls_cert_path: None,
            tls_key_path: None,
            monitoring_port: None,
            hydra_signing_key: "hydra.sk".into(),
            hydra_verification_keys: Vec::new(),
            persistence_dir: "./".into(),
            chain_config: ChainConfig::Direct(DirectChainConfig {
                network_id: NetworkId::Testnet(42),
                node_socket: "node.socket".into(),
                hydra_scripts_tx_id: "TxId".to_string(), // TODO: fix
                cardano_signing_key: "cardano.sk".into(),
                cardano_verification_keys: Vec::new(),
                start_chain_from: None,
                contestation_period: 60,
                deposit_deadline: 60,
            }),
            ledger_config: LedgerConfig {
                cardano_ledger_protocol_parameters_file: "protocol-parameters.json".into(),
            },
        }
    }
}
