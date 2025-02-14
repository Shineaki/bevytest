use lightyear::prelude::CompressionConfig;
use lightyear_examples_common::settings::{
    ClientSettings, ClientTransports, Conditioner, ServerSettings, ServerTransports, Settings,
    SharedSettings, WebTransportCertificateSettings,
};
use std::net::Ipv4Addr;
use std::string::ToString;

pub(crate) fn get_settings() -> Settings {
    Settings {
        server: ServerSettings {
            headless: false,
            inspector: true,
            conditioner: None,
            transport: vec![ServerTransports::Udp { local_port: 5000 }],
        },
        client: ClientSettings {
            inspector: true,
            client_id: 0,
            client_port: 0,
            server_addr: Ipv4Addr::LOCALHOST,
            server_port: 5000,
            transport: ClientTransports::Udp {},
            conditioner: None,
        },
        shared: SharedSettings {
            protocol_id: 0,
            private_key: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            compression: CompressionConfig::None,
        },
    }
}
