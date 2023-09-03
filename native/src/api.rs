use std::fs::File;
use std::net::UdpSocket;
use std::time::Duration;
use flutter_rust_bridge::StreamSink;
use pcap_parser::*;
use pcap_parser::traits::PcapReaderIterator;
use crate::pcap_parser::framewriter::FrameWriter;
use crate::pcap_parser::vertexwriter::VertexWriter;

use super::pcap_parser::parse_packet_body;

pub struct PcdFrame {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub other_data: Vec<f32>,
}

pub fn read_pcap_stream(stream: StreamSink<PcdFrame>, path: String) {
    let start = std::time::Instant::now();
    let file = File::open(path).unwrap();
    let mut reader = LegacyPcapReader::new(65536, file).unwrap();

    let mut writer = VertexWriter::create(stream);
    loop {
        match reader.next() {
            Ok((offset, block)) => {
                // num_packets += 1;
                match block {
                    PcapBlockOwned::Legacy(packet) => {
                        // println!("{}", packet.data.len());
                        // etherのヘッダ長は14byte
                        let ether_data = &packet.data[14..];
                        // ipv4のヘッダ長は可変(基本20byte)
                        let ip_header_size = ((ether_data[0] & 15) * 4) as usize;
                        let packet_size = (((ether_data[2] as u32) << 8) + ether_data[3] as u32) as usize;
                        let ip_data = &ether_data[ip_header_size..packet_size];
                        // udpのヘッダ長は8byte
                        let udp_data = &ip_data[8..ip_data.len()];
                        parse_packet_body(udp_data, &mut writer);
                    },
                    _ => ()
                }
                reader.consume(offset);
            }
            Err(pcap_parser::PcapError::Eof) => break,
            Err(pcap_parser::PcapError::Incomplete) => {
                reader.refill().unwrap();
            },
            Err(err) => panic!("packet read failed: {:?}", err),
        }

        // frame_count += 1;
        // if frame_count == frames_per_fragment {
        //     writer.finalize();
        //     stream.add(PcdFragment { 
        //         vertices: writer.buffer.clone(), 
        //         frame_start_indices: writer.frame_start_indices.clone(),
        //         max_point_num: writer.max_point_num
        //     });
        //     writer = VertexWriter::create();
        //     frame_count = 0;
        // }
    }
    writer.finalize();
    // if frame_count > 0 {
    //     writer.finalize();
    //     stream.add(PcdFragment { 
    //         vertices: writer.buffer.clone(), 
    //         frame_start_indices: writer.frame_start_indices.clone(),
    //         max_point_num: writer.max_point_num
    //     });
    // }
    println!("elapsed in rust: {} ms", start.elapsed().as_millis());
}

pub fn capture_hesai(stream: StreamSink<PcdFrame>, address: String) {
    // println!("Hello, world!");
    let socket = UdpSocket::bind(address).expect("Failed to bind socket");
    socket.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    let mut buf = [0; 1500];

    // let mut frame_counter = 0;
    // let mut invalid_point_num = 0;

    let mut writer = VertexWriter::create(stream);

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                // println!("recv_from function succeeded: {} bytes read from {}", amt, src);
                parse_packet_body(&buf[..amt], &mut writer);
                // invalid_point_num += points.iter().filter(|p| p.distance_m < 0.05).count();
                // frame_counter += 1;
                // if frame_counter >= 500 {
                //     println!("{}", invalid_point_num);
                //     invalid_point_num = 0;
                //     frame_counter = 0;
                // }
            }
            Err(e) => {
                println!("recv_from function failed: {}", e);
                break;
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_pcap() {
//         let path = String::from("C:\\Users\\hykwy\\flutter_pcd\\2023-03-03-1.pcap");
//         let pcd_video = read_pcap(path);
//         println!("max_point_num: {}", pcd_video.max_point_num);
//         println!("vertices: {}", pcd_video.vertices.len());
//     }
// }