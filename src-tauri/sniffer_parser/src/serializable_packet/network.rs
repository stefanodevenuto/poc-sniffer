use std::net::{Ipv4Addr, Ipv6Addr};

use pnet::packet::arp::{ArpOperations, ArpPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use pnet::util::MacAddr;
use serde::Serialize;

use super::SerializablePacket;

/// ARP Packet Representation

#[derive(Serialize, Debug)]
pub struct SerializableArpPacket {
    hardware_type: String,
    protocol_type: u16,
    hw_addr_len: u8,
    proto_addr_len: u8,
    operation: String,
    sender_hw_addr: MacAddr,
    sender_proto_addr: Ipv4Addr,
    target_hw_addr: MacAddr,
    target_proto_addr: Ipv4Addr,
    payload: Vec<u8>,
}

impl SerializablePacket for SerializableArpPacket {}

impl<'a> From<&ArpPacket<'a>> for SerializableArpPacket {
    fn from(packet: &ArpPacket<'a>) -> Self {
        SerializableArpPacket {
            hardware_type: format!("{:?}", packet.get_hardware_type()),
            protocol_type: packet.get_protocol_type().0,
            hw_addr_len: packet.get_hw_addr_len(),
            proto_addr_len: packet.get_proto_addr_len(),
            operation: match packet.get_operation() {
                ArpOperations::Reply => format!("ARP Reply ({})", packet.get_operation().0),
                ArpOperations::Request => format!("ARP Request ({})", packet.get_operation().0),
                _ => format!("ARP Unknown ({})", packet.get_operation().0),
            },
            sender_hw_addr: packet.get_sender_hw_addr(),
            sender_proto_addr: packet.get_sender_proto_addr(),
            target_hw_addr: packet.get_target_hw_addr(),
            target_proto_addr: packet.get_target_proto_addr(),
            payload: packet.payload().to_vec(),
        }
    }
}

/// IPv6 Packet Representation

#[derive(Serialize, Debug)]
pub struct SerializableIpv6Packet {
    pub version: u8,
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: String,
    pub hop_limit: u8,
    pub source: Ipv6Addr,
    pub destination: Ipv6Addr,
    pub payload: Vec<u8>,
}

impl SerializablePacket for SerializableIpv6Packet {}

impl<'a> From<&Ipv6Packet<'a>> for SerializableIpv6Packet {
    fn from(packet: &Ipv6Packet<'a>) -> Self {
        SerializableIpv6Packet {
            version: packet.get_version(),
            traffic_class: packet.get_traffic_class(),
            flow_label: packet.get_flow_label(),
            payload_length: packet.get_payload_length(),
            next_header: format!(
                "{} ({})",
                packet.get_next_header(),
                packet.get_next_header().0
            ),
            hop_limit: packet.get_hop_limit(),
            source: packet.get_source(),
            destination: packet.get_destination(),
            payload: packet.payload().to_vec(),
        }
    }
}

/// IPv4 Packet Representation

#[derive(Serialize, Debug)]
pub struct SerializableIpv4Packet {
    version: u8,
    header_length: u8,
    dscp: u8,
    ecn: u8,
    total_length: u16,
    identification: u16,
    flags: u8,
    fragment_offset: u16,
    ttl: u8,
    next_level_protocol: String,
    checksum: u16,
    source: Ipv4Addr,
    destination: Ipv4Addr,
    payload: Vec<u8>,
}

impl SerializablePacket for SerializableIpv4Packet {}

impl<'a> From<&Ipv4Packet<'a>> for SerializableIpv4Packet {
    fn from(packet: &Ipv4Packet<'a>) -> Self {
        SerializableIpv4Packet {
            version: packet.get_version(),
            header_length: packet.get_header_length(),
            dscp: packet.get_dscp(),
            ecn: packet.get_ecn(),
            total_length: packet.get_total_length(),
            identification: packet.get_identification(),
            flags: packet.get_flags(),
            fragment_offset: packet.get_fragment_offset(),
            ttl: packet.get_ttl(),
            next_level_protocol: format!(
                "{} ({})",
                packet.get_next_level_protocol(),
                packet.get_next_level_protocol().0
            ),
            checksum: packet.get_checksum(),
            source: packet.get_source(),
            destination: packet.get_destination(),
            payload: packet.payload().to_vec(),
        }
    }
}
