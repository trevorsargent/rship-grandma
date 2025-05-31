use rosc::{
    OscMessage, OscType,
    address::{Matcher, OscAddress},
    decoder::decode_udp,
};
use util::IntoResult;

use std::{collections::HashMap, net::UdpSocket, time::Duration};

mod cue;
mod sequence;
use cue::parse_cue_info;
mod hardware;
use sequence::Sequence;

mod util;

use anyhow::{Error, Ok, Result};

const FADER_BUTTON: &str = "/14.14.1.6.*";

pub const TOUCH: &str = "192.168.0.51:10000";

fn extract_osc_mesages(packet: &rosc::OscPacket) -> Vec<rosc::OscMessage> {
    match packet {
        rosc::OscPacket::Message(m) => {
            vec![m.clone()]
        }
        rosc::OscPacket::Bundle(b) => b
            .content
            .iter()
            .flat_map(|p| extract_osc_mesages(p))
            .collect(),
    }
}

async fn handle_cues(
    function: &String,
    args: &Vec<OscType>,
    sequence: &mut Sequence,
    // instance: InstanceProxy,
    udp_socket: &UdpSocket,
) -> Result<(), Error> {
    let cue_slug = match args.len() {
        3 => args[2].clone().string(),
        _ => None,
    };

    let is_on_cue = match args.len() {
        3 => args[1].clone().int().unwrap_or(0) == 1,
        _ => false,
    };

    // if !is_on_cue {
    //     println!("not on cue, {:?}", args);
    //     return Ok(());
    // }

    let cue_info = cue_slug
        .map(|s| parse_cue_info(s))
        .transpose()?
        .into_result()?;

    sequence
        .register_cue(&cue_info)
        .await
        .expect("could not register cue");

    if function == "Off" {
        println!("off function, not sending cue");
        return Ok(());
    }

    let cue_msg = OscMessage {
        addr: format!(
            "/ma/seq/{}/cue/{}/{}",
            sequence.id, cue_info.number, function
        ),
        args: vec![OscType::Int(1 as i32)],
    };

    let cue_off_msg = OscMessage {
        addr: format!(
            "/ma/seq/{}/cue/{}/{}",
            sequence.id, cue_info.number, function
        ),
        args: vec![OscType::Int(0 as i32)],
    };

    let cue_packet = rosc::OscPacket::Message(cue_msg.clone());

    let buf = rosc::encoder::encode(&cue_packet).expect("could not encode message");

    let send_res = udp_socket.send_to(&buf, TOUCH);

    if send_res.is_err() {
        println!("could not send message, {:?}", send_res);
        return Ok(());
    }

    tokio::time::sleep(Duration::from_millis(50)).await;

    let cue_off_packet = rosc::OscPacket::Message(cue_off_msg.clone());

    let buf = rosc::encoder::encode(&cue_off_packet).expect("could not encode message");

    let send_res = udp_socket.send_to(&buf, TOUCH);

    if send_res.is_err() {
        println!("could not send message, {:?}", send_res);
        return Ok(());
    }

    let seq_msg = OscMessage {
        addr: format!("/ma/seq/{}/{}", sequence.id, function),
        args: vec![OscType::Int(is_on_cue as i32)],
    };

    let packet = rosc::OscPacket::Message(seq_msg.clone());

    let buf = rosc::encoder::encode(&packet).expect("could not encode message");

    let send_res = udp_socket.send_to(&buf, TOUCH);

    if send_res.is_err() {
        println!("could not send message, {:?}", send_res);
        return Ok(());
    }

    match function.as_str() {
        "Go+" => {
            sequence.go_plus(cue_info).await?;
        }
        "Go-" => {
            sequence.go_minus(cue_info).await?;
        }
        "Goto" => {
            sequence.goto(cue_info).await?;
        }
        ">>>" => {
            sequence.jump_forward(cue_info).await?;
        }
        "<<<" => {
            sequence.jump_backward(cue_info).await?;
        }
        "Pause" => {
            // let _ = sequence
            //     .pause
            //     .pulse(Momentary {
            //         id: uuid::Uuid::new_v4().to_string(),
            //         tag: None,
            //     })
            //     .await;
        }

        _ => {
            println!("not a knwon Cue Movement, {:?}, {:?}", function, args);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let fader_matcher = Matcher::new(FADER_BUTTON).expect("valid address");

    let mut sequences = HashMap::<i32, Sequence>::new();

    {
        let socket = UdpSocket::bind("0.0.0.0:10000");

        let socket = socket.expect("could not bind");

        println!("Listening on 0.0.0.0:10000");

        // let client = SdkClient::init();

        // client.set_address(Some("192.168.1.216:5155".into()));

        // client.await_connection().await;

        // let instance = client
        //     .add_instance(InstanceArgs {
        //         cluster_id: None,
        //         code: "ma".into(),
        //         name: "Cudi".into(),
        //         color: "#dd0000".into(),
        //         machine_id: "Cudi".into(),
        //         service_id: "Cudi".into(),
        //         short_id: "Cudi".into(),
        //         message: None,
        //         status: rship_sdk::InstanceStatus::Available,
        //     })
        //     .await;

        let mut buf = [0; 1024];

        loop {
            let (_amt, _src) = socket.recv_from(&mut buf)?;

            let packet = decode_udp(&buf);

            let p = packet.as_ref().ok();
            if let Some(p) = p {
                let messages = extract_osc_mesages(&p.1);

                for m in messages {
                    let addr_str: String = m.addr.clone();

                    let addr = OscAddress::new(addr_str.clone());

                    if addr.is_err() {
                        println!("invalid address, {}", p.1);

                        continue;
                    }

                    println!("decoded message, {:?}, {:?}", m.addr, m.args);

                    let args = m.args.clone();

                    let addr = addr.expect("could not get address");

                    if fader_matcher.match_address(&addr) {
                        let seq_number = addr_str
                            .split(".")
                            .last()
                            .expect("could not get last sequence")
                            .parse::<i32>()
                            .expect("msg seq number not a number");

                        if !sequences.contains_key(&seq_number) {
                            println!("creating sequence");
                            let sequence = Sequence::new(seq_number).await;

                            sequences.insert(seq_number, sequence);
                        }

                        let sequence = sequences
                            .get_mut(&seq_number)
                            .expect("could not get parent");

                        match args[0] {
                            OscType::String(ref function) => {
                                let _ = handle_cues(function, &args, sequence, &socket).await;

                                let _ = sequence.process_fader(function, &args, &socket).await;

                                // if function.starts_with("Fader") {

                                //     if !faders.contains_key(&id) {
                                //         println!("did not find fader master");
                                //         let proxy = sequence
                                //             .add_emitter::<Fader>(EmitterArgs::new(
                                //                 format!("{} {}", function, seq_number),
                                //                 format!("{}{}", function, seq_number),
                                //             ))
                                //             .await;

                                //         faders.insert(id.clone(), proxy);
                                //     }

                                //     let fader = faders.get_mut(&id).expect("could not get fader");

                                //     let level =
                                //         args[2].clone().float().expect("msg level not a float");

                                //     let _ = fader.pulse(Fader { level: level }).await;
                                // }
                            }

                            _ => {
                                println!("not a string, {:?}", p.1);
                            }
                        }
                    } else {
                        println!("did not match address, {:?}", addr_str);
                    }
                }
            }
        }
    }
}
