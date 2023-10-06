use std::{net::Ipv4Addr, sync::Arc};

use etherparse::{InternetSlice, SlicedPacket};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{self, UnboundedSender},
};

use crate::{
    setup::{TUN_BASE_NAME, TUN_DEVICE_ADDRESS, TUN_DEVICE_NETMASK},
    ActivePeers,
};

fn setup_tokio_tun_device(name: &str, address: Ipv4Addr, netmask: Ipv4Addr) -> tokio_tun::Tun {
    log::info!("Creating TUN device with: address={address}, netmask={netmask}");
    tokio_tun::Tun::builder()
        .name(name)
        .tap(false)
        .packet_info(false)
        .mtu(1350)
        .up()
        .address(address)
        .netmask(netmask)
        .try_build()
        .expect("Failed to setup tun device, do you have permission?")
}

pub fn start_tun_device(_active_peers: Arc<ActivePeers>) -> UnboundedSender<Vec<u8>> {
    let tun = setup_tokio_tun_device(
        format!("{}%d", TUN_BASE_NAME).as_str(),
        TUN_DEVICE_ADDRESS.parse().unwrap(),
        TUN_DEVICE_NETMASK.parse().unwrap(),
    );
    log::info!("Created TUN device: {}", tun.name());

    let (mut tun_device_rx, mut tun_device_tx) = tokio::io::split(tun);

    // Channels to communicate with the other tasks
    let (tun_task_tx, mut tun_task_rx) = mpsc::unbounded_channel::<Vec<u8>>();

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            tokio::select! {
                // Reading from the TUN device
                len = tun_device_rx.read(&mut buf) => match len {
                    Ok(len) => {
                        let packet = &buf[..len];
                        let dst_addr = boringtun::noise::Tunn::dst_address(packet).unwrap();

                        let headers = SlicedPacket::from_ip(packet).unwrap();
                        let src_addr = match headers.ip.unwrap() {
                            InternetSlice::Ipv4(ip, _) => ip.source_addr().to_string(),
                            InternetSlice::Ipv6(ip, _) => ip.source_addr().to_string(),
                        };
                        log::info!("iface: read Packet({src_addr} -> {dst_addr}, {len} bytes)");

                        // TODO: route packet to the correct peer.
                        log::info!("...forward packet to the correct peer (NOT YET IMPLEMENTED)");
                    },
                    Err(err) => {
                        log::info!("iface: read error: {err}");
                        break;
                    }
                },

                // Writing to the TUN device
                Some(data) = tun_task_rx.recv() => {
                    let headers = SlicedPacket::from_ip(&data).unwrap();
                    let (source_addr, destination_addr) = match headers.ip.unwrap() {
                        InternetSlice::Ipv4(ip, _) => (ip.source_addr(), ip.destination_addr()),
                        InternetSlice::Ipv6(_, _) => unimplemented!(),
                    };

                    log::info!(
                        "iface: write Packet({source_addr} -> {destination_addr}, {} bytes)",
                        data.len()
                    );
                    // log::info!("iface: writing {} bytes", data.len());
                    tun_device_tx.write_all(&data).await.unwrap();
                }
            }
        }
        log::info!("TUN device shutting down");
    });
    tun_task_tx
}