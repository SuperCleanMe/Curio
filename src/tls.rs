//use crate::structs::Response;
use rustls::{ClientConfig, ClientSession};
use webpki_roots::TLS_SERVER_ROOTS;
use webpki::*;
use std::sync::Arc;
use std::io::Write;

pub fn get<S: Into<String>>(domain: S, path: S) {
    let formatted_domain = domain.into();
    let formatted_path = path.into();
    let config = Arc::new(build_tls_config());
    let domain_ref = DNSNameRef::try_from_ascii_str(formatted_domain.as_str()).unwrap();
    let mut client = ClientSession::new(&config, domain_ref);
    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", formatted_path, formatted_domain);
    client.write_all(request.as_bytes()).unwrap();
}

fn build_tls_config() -> ClientConfig {
    let mut cfg = ClientConfig::new();
    cfg.root_store.add_server_trust_anchors(&TLS_SERVER_ROOTS);
    cfg
}