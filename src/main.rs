mod transfr;

fn main() {
    let info = transfr::server::ServerInfo {
        addr: "0.0.0.0".to_string(),
        port: 8081,
    };

    transfr::server::run(info);
}
