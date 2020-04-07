use std::net::{IpAddr, Ipv4Addr};
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct RemoteHost {
    pub ip: String,
    pub mac: Vec<u8>,
    pub name: String
}

impl Default for RemoteHost {
    fn default() -> RemoteHost {
        RemoteHost {
            ip: String::from("127.0.0.1"),
            mac: vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
            name: String::from("NodeJS Samsung Remote")
        }
    }
}

#[derive(Debug)]
pub struct RemoteConfig {
    pub host: RemoteHost,
    pub app_string: String,
    pub tv_app_string: String,
    pub port: i32,
    pub timeout: i32
}

impl Default for RemoteConfig {
    fn default() -> RemoteConfig {
        RemoteConfig{
            host: RemoteHost::default(),
            app_string: String::from("iphone..iapp.samsung"),
            tv_app_string: String::from("iphone.UN60D6000.iapp.samsung"),
            port: 55000i32,
            timeout: 5000i32
        }
    }
}

#[derive(Debug, Default)]
pub struct Remote {
    pub config: RemoteConfig
}

impl Remote {
    fn socket_chunk_one(&self) -> String {
        let ip_encoded = base64::encode(self.config.host.ip.clone());
        let mac_encoded = base64::encode(self.config.host.mac.clone());
        let host_name_encoded = base64::encode(self.config.host.name.clone());

        let mut message = String::from("");
        message.push(0x64 as char);
        message.push(0x00 as char);
        message.push(ip_encoded.len() as u8 as char);
        message.push(0x00 as char);
        message.push_str(&ip_encoded);
        message.push(mac_encoded.len() as u8 as char);
        message.push(0x00 as char);
        message.push_str(&mac_encoded);
        message.push(host_name_encoded.len() as u8 as char);
        message.push(0x00 as char);
        message.push_str(&host_name_encoded);


        let mut res = String::from("");
        res.push(0x00 as char);
        res.push(self.config.app_string.len() as u8 as char);
        res.push(0x00 as char);
        res.push_str(&self.config.app_string);
        res.push(message.len() as u8 as char);
        res.push(0x00 as char);
        res.push_str(&message);

        res
    }

    fn socket_chunk_two(&self, command: &str) -> String {
        let command_encoded = base64::encode(command);

        let mut message = String::from("");
        message.push(0x00 as char);
        message.push(0x00 as char);
        message.push(0x00 as char);
        message.push(command_encoded.len() as u8 as char);
        message.push(0x00 as char);
        message.push_str(&command_encoded);

        let mut res = String::from("");
        res.push(0x00 as char);
        res.push(self.config.tv_app_string.len() as u8 as char);
        res.push(0x00 as char);
        res.push_str(&self.config.tv_app_string);
        res.push(message.len() as u8 as char);
        res.push(0x00 as char);
        res.push_str(&message);

        res
    }

    fn get_host_port(&self) -> String {
        let mut res = self.config.host.ip.clone();
        res.push(':');
        res.push_str(&self.config.port.to_string());
        res
    }

    pub fn send(&self, command: &str) -> Result<i32, i32> {
        let mut stream = TcpStream::connect(self.get_host_port())
            .expect("Couldn't connect to the server...");

        stream.write(self.socket_chunk_one().as_bytes())
            .expect("Couldn't write to the server...");

        stream.write(self.socket_chunk_two(command).as_bytes())
            .expect("Couldn't write to the server...");

        Ok(123)
    }

    pub fn wake(&self) {
        use wake_on_lan;
        let mut mac: [u8; 6] = [0,0,0,0,0,0];
        mac.clone_from_slice(&self.config.host.mac[0..6]);
        println!("{:?}", mac);
        let magic_packet = wake_on_lan::MagicPacket::new(&mac);
        magic_packet.send();
    }

}