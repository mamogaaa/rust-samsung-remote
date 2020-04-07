extern crate base64;
extern crate wake_on_lan;
use std::env;
use std::fs;
use std::i32;

mod samsung_remote;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ip = args.get(1).unwrap_or(&String::from("127.0.0.1")).clone();
    let command = args.get(2).unwrap_or(&String::from("KEY_MENU")).clone();
    let mac = args.get(3).unwrap_or(&String::from("00:00:00:00:00:00")).clone();
    let name = args.get(4).unwrap_or(&String::from("Rust Samsung Remote")).clone();

    let macVec: Vec<u8> = mac.split(":").map(|s| i32::from_str_radix(s, 16).unwrap_or(0) as u8).collect();
    
    let remote = samsung_remote::Remote {
        config: samsung_remote::RemoteConfig {
            host: samsung_remote::RemoteHost {
                ip: ip,
                name: name,
                mac: macVec,
                ..Default::default()
            },
            ..Default::default()
        }
    };

    
    if command == "WAKE" {
        remote.wake();
    } else {
        remote.send(&command);
    }
}
