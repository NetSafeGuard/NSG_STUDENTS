use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::{ipv4::Ipv4Packet, udp::UdpPacket, Packet};
use std::collections::{HashMap, HashSet};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Função para extrair o domínio raiz
fn extract_root_domain(domain: &str) -> String {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() > 2 {
        parts[parts.len() - 2..].join(".")
    } else {
        domain.to_string()
    }
}

pub fn run_packet_capture(domains: Arc<Mutex<Vec<String>>>, window: tauri::Window) {
    thread::sleep(Duration::from_secs(5));

    let interfaces = datalink::interfaces();
    for iface in &interfaces {
        println!("Nome da Interface de Rede: {}", iface.name);
    }

    let interface_name = interfaces[0].name.clone();

    let interface = interfaces
        .iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface não encontrada");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Tipo de canal não tratado"),
        Err(e) => panic!("Falha ao criar canal: {}", e),
    };

    let mut captured_domains: HashSet<String> = HashSet::new();
    let mut domain_timestamps: HashMap<String, Instant> = HashMap::new();
    let ttl = Duration::from_secs(300);

    let ignore_urls = vec![
        "pki.goog",
        "cloudflare.com",
        "tawk.to",
        "doubleclick.net",
        "gstatic.com",
        "windowsupdate.com",
        "msedge.net",
        "microsoft.com",
        "msecnd.net",
        "wpad.home",
        "entrust.net",
        "google-analytics.com",
        "google.com",
        "googleapis.com",
        "googleusercontent.com",
        "googletagmanager.com",
        "mozilla.net",
        "mozaws.net",
        "mozilla.com",
        "mozilla.org",
        "gstatic.com",
        "office.com",
        "scdn.co",
        "azure.com",
        "msftncsi.com",
        "googlevideo.com",
        "ggpht.com",
        "ytimg.com",
        "jquery.com",
        "jquery.org",
        "bootstrapcdn.com",
        "jsdelivr.net",
        "clarify-ms",
        "google.pt",
        "withgoogle.com"
    ];

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(eth) = EthernetPacket::new(packet) {
                    if eth.get_ethertype() == EtherTypes::Ipv4 {
                        if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                            if ipv4.get_next_level_protocol()
                                == pnet::packet::ip::IpNextHeaderProtocols::Udp
                            {
                                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                    if udp.get_destination() == 53 {
                                        let dns_payload = udp.payload();
                                        if dns_payload.len() >= 5 {
                                            let qdcount = (dns_payload[4] as usize) << 8
                                                | (dns_payload[5] as usize);
                                            let mut offset = 12;

                                            for _ in 0..qdcount {
                                                let mut domain_name = String::new();
                                                let mut label_len = dns_payload[offset] as usize;
                                                offset += 1;

                                                while label_len > 0 {
                                                    if domain_name.len() > 0 {
                                                        domain_name.push('.');
                                                    }

                                                    domain_name += str::from_utf8(
                                                        &dns_payload[offset..offset + label_len],
                                                    )
                                                    .unwrap_or("<inválido>");

                                                    offset += label_len;
                                                    label_len = dns_payload[offset] as usize;
                                                    offset += 1;
                                                }

                                                let root_domain = extract_root_domain(&domain_name);

                                                if ignore_urls.contains(&root_domain.as_str()) {
                                                    continue;
                                                }

                                                let now = Instant::now();
                                                domain_timestamps.retain(|_, &mut timestamp| {
                                                    now.duration_since(timestamp) < ttl
                                                });

                                                if !captured_domains.contains(&root_domain) {
                                                    {
                                                        let mut captured_domains_shared =
                                                            domains.lock().unwrap();
                                                        captured_domains_shared
                                                            .push(root_domain.clone());
                                                    }

                                                    captured_domains.insert(root_domain.clone());
                                                    domain_timestamps
                                                        .insert(root_domain.clone(), now);

                                                    window
                                                        .emit("suspicious", "test")
                                                        .expect("failed to emit alert");

                                                    println!(
                                                        "Domínio Raiz Capturado: {}",
                                                        root_domain
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Erro ao capturar pacote: {}", e),
        }
    }
}
