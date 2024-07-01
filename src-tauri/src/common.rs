use std::{net::IpAddr, thread};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use crate::tls::{self,TlsParser,STREAM_TOSERVER};

/// Print SNI if exists
pub fn parse_tls(src: IpAddr, dst: IpAddr, tcp: &TcpPacket, window: Option<&tauri::Window>) {
    let payload = tcp.payload();
    if payload.len() == 0 { return; }

    if tls::tls_probe(&payload) {
        let mut parser = TlsParser::new(&payload);

        let allow_domains = vec!["github.com", "www.youtube.com"];

        parser.parse(&payload, STREAM_TOSERVER);
        
        if parser.sni.len() > 0 {
            println!("TCP {:?}:{} -> {:?}:{}",
                src, tcp.get_source(),
                dst, tcp.get_destination());

                if let Some(w) = window {
                    for domain in parser.sni {
                        w.emit("test", String::from(format!("Domain: {}, src: {:?}, dst: {:?}", domain, src, dst))).unwrap();
                    }
                }
        }
    }
    else if let Ok(http) = String::from_utf8(payload.to_vec()) {
        if let Some(host) = http.lines().find(|line| line.starts_with("Host: ")) {
            let host = host.trim_start_matches("Host: ");
            println!("HTTP Host: {}", host);
        }
    }
}