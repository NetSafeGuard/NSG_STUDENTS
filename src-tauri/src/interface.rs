use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use pnet::util::MacAddr;

use std::net::IpAddr;
use std::thread;

use crate::common::parse_tls;

fn handle_tcp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8], window: &tauri::Window) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        parse_tls(source, destination, &tcp, Some(window))
    } else {
        println!("[{}]: Malformed TCP Packet", interface_name);
    }
}

fn handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    window: &tauri::Window
) {
    match protocol {
        IpNextHeaderProtocols::Tcp => {
            handle_tcp_packet(interface_name, source, destination, packet, window)
        }
        _ => (),
    }
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket, window: &tauri::Window) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
            window
        );
    } else {
        println!("[{}]: Malformed IPv4 Packet", interface_name);
    }
}

fn handle_ipv6_packet(interface_name: &str, ethernet: &EthernetPacket, window: &tauri::Window) {
    let header = Ipv6Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
            window
        );
    } else {
        println!("[{}]: Malformed IPv6 Packet", interface_name);
    }
}

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket, window: &tauri::Window) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet, &window),
        EtherTypes::Ipv6 => handle_ipv6_packet(interface_name, ethernet, &window),
        _ => (),
    }
}

fn process_interface(interface: NetworkInterface, window: tauri::Window) {
    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    loop {
        let mut buf: [u8; 1600] = [0u8; 1600];
        let mut fake_ethernet_frame = MutableEthernetPacket::new(&mut buf[..]).unwrap();
        match rx.next() {
            Ok(packet) => {
                if cfg!(target_os = "macos")
                    && interface.is_up()
                    && !interface.is_broadcast()
                    && !interface.is_loopback()
                    && interface.is_point_to_point()
                {
                    // Maybe is TUN interface
                    let version = Ipv4Packet::new(&packet).unwrap().get_version();
                    if version == 4 {
                        fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                        fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                        fake_ethernet_frame.set_ethertype(EtherTypes::Ipv4);
                        fake_ethernet_frame.set_payload(&packet);
                        handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable(), &window);
                        continue;
                    } else if version == 6 {
                        fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                        fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                        fake_ethernet_frame.set_ethertype(EtherTypes::Ipv6);
                        fake_ethernet_frame.set_payload(&packet);
                        handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable(), &window);
                        continue;
                    }
                }
                handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap(), &window);
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    }
}

pub fn process_all_interfaces(window: tauri::Window) {
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        let interface = interface.clone();

        let window = window.clone();
        let handle = thread::spawn(move || {
            process_interface(interface, window);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
