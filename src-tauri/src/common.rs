use crate::tls::{self, TlsParser, STREAM_TOSERVER};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use std::{net::IpAddr, thread};

/// Print SNI if exists
pub fn parse_tls(src: IpAddr, dst: IpAddr, tcp: &TcpPacket, window: Option<&tauri::Window>) {
    let payload = tcp.payload();
    if payload.len() == 0 {
        return;
    }

    if tls::tls_probe(&payload) {
        let mut parser = TlsParser::new(&payload);

        let allow_domains = vec!["github.com", "www.youtube.com"];

        parser.parse(&payload, STREAM_TOSERVER);

        if parser.sni.len() > 0 {
            println!(
                "TCP {:?}:{} -> {:?}:{}",
                src,
                tcp.get_source(),
                dst,
                tcp.get_destination()
            );

            if let Some(w) = window {
                for domain in parser.sni {
                    if !allow_domains.contains(&domain.as_str()) {
                        println!("SNI: {}", domain);
                        w.emit("suspicious", "test").expect("failed to emit alert");
                    }
                }
            }
        }
    } else if let Ok(http) = String::from_utf8(payload.to_vec()) {
        if let Some(host) = http.lines().find(|line| line.starts_with("Host: ")) {
            let host = host.trim_start_matches("Host: ");
            println!("HTTP Host: {}", host);
        }
    }
}
