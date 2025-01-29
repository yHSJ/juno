use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, row, scrollable, text,
    text_input, Column, Container,
};
use iced::{alignment, Alignment, Element, Length, Task};
use std::path::PathBuf;

use crate::hydra::run_options::{
    ChainConfig, DirectChainConfig, NetworkId, NodeId, OfflineChainConfig, RunOptions, Verbosity,
};

#[derive(Debug, Clone)]
pub enum Message {
    VerbosityToggled(bool),
    NodeIdChanged(String),
    HostChanged(String),
    PortChanged(String),

    ApiHostChanged(String),
    ApiPortChanged(String),
    MonitoringPortChanged(String),

    TlsCertPathChanged(String),
    TlsKeyPathChanged(String),
    BrowseTlsCert,
    BrowseTlsKey,

    HydraSigningKeyChanged(String),
    AddHydraVerificationKey,
    RemoveHydraVerificationKey(usize),
    HydraVerificationKeyChanged(usize, String),
    BrowseHydraSigningKey,
    BrowseHydraVerificationKey(usize),

    ChainConfigTypeChanged(ChainConfigType),
    NetworkIdChanged(String),
    NodeSocketChanged(String),

    InitialUtxoChanged(String),
    BrowseInitialUtxo,
    PersistenceDirChanged(String),
    BrowsePersistenceDir,

    SaveSettings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainConfigType {
    Offline,
    Direct,
}
impl Default for ChainConfigType {
    fn default() -> Self {
        ChainConfigType::Offline
    }
}
impl std::fmt::Display for ChainConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainConfigType::Offline => write!(f, "Offline"),
            ChainConfigType::Direct => write!(f, "Online"),
        }
    }
}
#[derive(Default)]
pub struct HydraSettings {
    run_options: RunOptions,
    chain_config_type: ChainConfigType,
}

impl HydraSettings {
    pub fn new() -> Self {
        Self {
            run_options: RunOptions::default(),
            chain_config_type: ChainConfigType::Direct,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(self.general_settings_section())
            .push(horizontal_rule(1))
            .push(self.api_settings_section())
            .push(horizontal_rule(1))
            .push(self.tls_settings_section())
            .push(horizontal_rule(1))
            .push(self.hydra_keys_section())
            .push(horizontal_rule(1))
            .push(self.chain_config_section())
            .push(horizontal_rule(1))
            .push(self.persistence_section())
            .push(horizontal_rule(1))
            .push(button("Save Settings").on_press(Message::SaveSettings));

        let scrollable_content = scrollable(content).height(Length::Fill).width(Length::Fill);

        Container::new(scrollable_content)
            .center_x(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn general_settings_section(&self) -> Element<Message> {
        let title = text("General Settings").size(24);

        let quiet_mode = checkbox(
            "Quiet Mode",
            matches!(self.run_options.verbosity, Verbosity::Quiet),
        )
        .on_toggle(Message::VerbosityToggled);

        let node_id = row![
            text("Node ID:").width(Length::Fixed(150.0)),
            text_input(
                "Enter node ID...",
                match &self.run_options.node_id {
                    NodeId(id) => id,
                },
            )
            .on_input(Message::NodeIdChanged)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let host = row![
            text("Host:").width(Length::Fixed(150.0)),
            text_input("Enter host...", &self.run_options.host.to_string(),)
                .on_input(Message::HostChanged)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let port = row![
            text("Port:").width(Length::Fixed(150.0)),
            text_input("Enter port...", &self.run_options.port.to_string(),)
                .on_input(Message::PortChanged)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        Column::new()
            .push(title)
            .push(quiet_mode)
            .push(node_id)
            .push(host)
            .push(port)
            .spacing(10)
            .into()
    }

    fn api_settings_section(&self) -> Element<Message> {
        let title = text("API Settings").size(24);

        let api_host = row![
            text("API Host:").width(Length::Fixed(150.0)),
            text_input("Enter API host...", &self.run_options.api_host.to_string(),)
                .on_input(Message::ApiHostChanged)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        let api_port = row![
            text("API Port:").width(Length::Fixed(150.0)),
            text_input("Enter API port...", &self.run_options.api_port.to_string(),)
                .on_input(Message::ApiPortChanged)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        let monitoring_port = row![
            text("Monitoring Port:").width(Length::Fixed(150.0)),
            text_input(
                "Enter monitoring port...",
                self.run_options
                    .monitoring_port
                    .map_or("".to_string(), |p| p.to_string())
                    .as_str(),
            )
            .on_input(Message::MonitoringPortChanged)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        column![title, api_host, api_port, monitoring_port]
            .spacing(10)
            .into()
    }

    fn tls_settings_section(&self) -> Element<Message> {
        let title = text("TLS Settings").size(24);

        let cert_path = row![
            text("TLS Certificate:").width(Length::Fixed(150.0)),
            text_input(
                "Enter certificate path...",
                &self
                    .run_options
                    .tls_cert_path
                    .as_ref()
                    .map_or("".to_string(), |p| p.to_string_lossy().to_string()),
            )
            .on_input(Message::TlsCertPathChanged),
            button("Browse").on_press(Message::BrowseTlsCert)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        let key_path = row![
            text("TLS Key:").width(Length::Fixed(150.0)),
            text_input(
                "Enter key path...",
                &self
                    .run_options
                    .tls_key_path
                    .as_ref()
                    .map_or("".to_string(), |p| p.to_string_lossy().to_string()),
            )
            .on_input(Message::TlsKeyPathChanged),
            button("Browse").on_press(Message::BrowseTlsKey)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        column![title, cert_path, key_path].spacing(10).into()
    }

    fn hydra_keys_section(&self) -> Element<Message> {
        let title = text("Hydra Keys").size(24);

        let signing_key = row![
            text("Signing Key:").width(Length::Fixed(150.0)),
            text_input(
                "Enter signing key path...",
                &self.run_options.hydra_signing_key.to_string_lossy(),
            )
            .on_input(Message::HydraSigningKeyChanged),
            button("Browse").on_press(Message::BrowseHydraSigningKey)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        let verification_keys = self
            .run_options
            .hydra_verification_keys
            .iter()
            .enumerate()
            .fold(Column::new().spacing(10), |column, (idx, key)| {
                column.push(
                    row![
                        text(format!("Verification Key {}:", idx + 1)).width(Length::Fixed(150.0)),
                        text_input("Enter verification key path...", &key.to_string_lossy(),)
                            .on_input(move |value| Message::HydraVerificationKeyChanged(
                                idx, value
                            )),
                        button("Browse").on_press(Message::BrowseHydraVerificationKey(idx)),
                        button("Remove").on_press(Message::RemoveHydraVerificationKey(idx))
                    ]
                    .spacing(10)
                    .align_y(alignment::Alignment::Center),
                )
            });

        let add_key_button =
            button("Add Verification Key").on_press(Message::AddHydraVerificationKey);

        column![title, signing_key, verification_keys, add_key_button]
            .spacing(10)
            .into()
    }

    fn chain_config_section(&self) -> Element<Message> {
        let title = text("Chain Configuration").size(24);

        let config_type = row![
            text("Configuration Type:").width(Length::Fixed(150.0)),
            pick_list(
                vec![ChainConfigType::Direct, ChainConfigType::Offline],
                Some(self.chain_config_type),
                Message::ChainConfigTypeChanged
            )
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        let config_fields = match self.chain_config_type {
            ChainConfigType::Direct => {
                if let ChainConfig::Direct(direct_config) = &self.run_options.chain_config {
                    column![
                        row![
                            text("Network ID:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter network ID...",
                                &direct_config.network_id.to_string(),
                            )
                            .on_input(Message::NetworkIdChanged)
                        ],
                        row![
                            text("Node Socket:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter node socket path...",
                                &direct_config.node_socket.to_string_lossy(),
                            )
                            .on_input(Message::NodeSocketChanged)
                        ]
                    ]
                    .spacing(10)
                } else {
                    column![]
                }
            }
            ChainConfigType::Offline => {
                if let ChainConfig::Offline(offline_config) = &self.run_options.chain_config {
                    column![row![
                        text("Initial UTxO File:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter UTxO file path...",
                            &offline_config.initial_utxo_file.to_string_lossy(),
                        )
                        .on_input(Message::InitialUtxoChanged),
                        button("Browse").on_press(Message::BrowseInitialUtxo)
                    ]]
                    .spacing(10)
                } else {
                    column![]
                }
            }
        };

        column![title, config_type, config_fields]
            .spacing(10)
            .into()
    }

    fn persistence_section(&self) -> Element<Message> {
        let title = text("Persistence Settings").size(24);

        let dir_path = row![
            text("Persistence Directory:").width(Length::Fixed(150.0)),
            text_input(
                "Enter persistence directory path...",
                &self.run_options.persistence_dir.to_string_lossy(),
            )
            .on_input(Message::PersistenceDirChanged),
            button("Browse").on_press(Message::BrowsePersistenceDir)
        ]
        .spacing(10)
        .align_y(alignment::Alignment::Center);

        column![title, dir_path].spacing(10).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::VerbosityToggled(quiet) => {
                self.run_options.verbosity = if quiet {
                    Verbosity::Quiet
                } else {
                    Verbosity::Verbose
                };
            }
            Message::NodeIdChanged(id) => {
                self.run_options.node_id = NodeId(id);
            }
            Message::HostChanged(host) => {
                if let Ok(ip) = host.parse() {
                    self.run_options.host = ip;
                }
            }
            Message::PortChanged(port) => {
                if let Ok(p) = port.parse() {
                    self.run_options.port = p;
                }
            }
            Message::ApiHostChanged(host) => {
                if let Ok(ip) = host.parse() {
                    self.run_options.api_host = ip;
                }
            }
            Message::ApiPortChanged(port) => {
                if let Ok(p) = port.parse() {
                    self.run_options.api_port = p;
                }
            }
            Message::MonitoringPortChanged(port) => {
                self.run_options.monitoring_port = port.parse().ok();
            }
            Message::TlsCertPathChanged(path) => {
                self.run_options.tls_cert_path = Some(PathBuf::from(path));
            }
            Message::TlsKeyPathChanged(path) => {
                self.run_options.tls_key_path = Some(PathBuf::from(path));
            }
            Message::HydraSigningKeyChanged(path) => {
                self.run_options.hydra_signing_key = PathBuf::from(path);
            }
            Message::AddHydraVerificationKey => {
                self.run_options
                    .hydra_verification_keys
                    .push(PathBuf::new());
            }
            Message::RemoveHydraVerificationKey(idx) => {
                self.run_options.hydra_verification_keys.remove(idx);
            }
            Message::HydraVerificationKeyChanged(idx, path) => {
                if let Some(key) = self.run_options.hydra_verification_keys.get_mut(idx) {
                    *key = PathBuf::from(path);
                }
            }
            Message::ChainConfigTypeChanged(config_type) => {
                self.chain_config_type = config_type;
                self.run_options.chain_config = match config_type {
                    ChainConfigType::Direct => ChainConfig::Direct(DirectChainConfig {
                        network_id: NetworkId::Testnet(42),
                        node_socket: "node.socket".into(),
                        hydra_scripts_tx_id: "".into(),
                        cardano_signing_key: "cardano.sk".into(),
                        cardano_verification_keys: Vec::new(),
                        start_chain_from: None,
                        contestation_period: 60,
                        deposit_deadline: 60,
                    }),
                    ChainConfigType::Offline => ChainConfig::Offline(OfflineChainConfig {
                        initial_utxo_file: "utxo.json".into(),
                        ledger_genesis_file: None,
                    }),
                };
            }
            // TODO: handle network ID correctly
            Message::NetworkIdChanged(_) => {
                if let ChainConfig::Direct(config) = &mut self.run_options.chain_config {
                    config.network_id = NetworkId::Mainnet
                }
            }
            Message::NodeSocketChanged(socket) => {
                if let ChainConfig::Direct(config) = &mut self.run_options.chain_config {
                    config.node_socket = PathBuf::from(socket);
                }
            }
            Message::PersistenceDirChanged(dir) => {
                self.run_options.persistence_dir = PathBuf::from(dir);
            }

            Message::BrowseTlsCert => {
                // TODO: Implement file dialog for TLS cert
            }
            Message::BrowseTlsKey => {
                // TODO: Implement file dialog for TLS key
            }
            Message::BrowseHydraSigningKey => {
                // TODO: Implement file dialog for Hydra signing key
            }
            Message::BrowseHydraVerificationKey(_idx) => {
                // TODO: Implement file dialog for Hydra verification key
            }
            Message::BrowsePersistenceDir => {
                // TODO: Implement directory dialog
            }
            Message::SaveSettings => {
                // TODO: Implement saving settings
                println!("Saving settings: {:?}", self.run_options);
            }
            Message::InitialUtxoChanged(_) => todo!(),
            Message::BrowseInitialUtxo => todo!(),
        }
    }
}

pub struct HydraSettingsApp {
    settings: HydraSettings,
}

impl HydraSettingsApp {
    pub fn new() -> Self {
        Self {
            settings: HydraSettings::new(),
        }
    }
}

impl HydraSettingsApp {
    fn title(&self) -> String {
        String::from("Hydra Node Settings")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        self.settings.update(message);
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        container(self.settings.view())
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
