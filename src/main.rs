use std::io::{self, Cursor};

use dnslib::dns::rfc::{query::Query, response::Response, response_code::ResponseCode};

use regex::Regex;
use tokio::net::UdpSocket;
use type2network::{FromNetworkOrder, ToNetworkOrder};

mod filter;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buffer = [0; 1024];

    let re = Regex::new(r"liberation").unwrap();

    loop {
        let (received, client_addr) = sock.recv_from(&mut buffer).await?;
        println!("{:?} bytes received from {:?}", received, client_addr);

        // receive DNS query
        let mut query = Query::default();
        let mut cursor = Cursor::new(&buffer[..received]);
        query.deserialize_from(&mut cursor).unwrap();
        // println!("{:?}", query);

        let domain = query.question.qname.to_string();
        println!("requested: {}", domain);

        if re.is_match(&domain) {
            println!("not accepted!");
        }

        // send NXDOMAIN
        let mut response = Response::from(&query);
        response.set_response_code(ResponseCode::NXDomain);
        println!("================ {:?}", response);

        // serialize response to send it
        let mut buffer: Vec<u8> = Vec::new();
        let message_size = response.serialize_to(&mut buffer).unwrap();

        let sent = sock.send_to(&buffer, client_addr).await?;
        println!("{:?}", sent);
        // let len = sock.send_to(&buf[..len], addr).await?;
    }
}
