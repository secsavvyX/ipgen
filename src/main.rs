use dns_lookup;
use dns_lookup::lookup_addr;
use ipnet::Ipv4AddrRange;
use ipnet::Ipv4Net;
use std::io::{stdin, stdout, Write};
use std::iter::Map;
use std::net::Ipv4Addr;

fn main() {
    println!(
        "# #####   ####  ###### #    # 
# #    # #    # #      ##   # 
# #    # #      #####  # #  # 
# #####  #  ### #      #  # # 
# #      #    # #      #   ## 
# #       ####  ###### #    #    by @sk3wl"
    );
    let mut cidr = String::new();
    print!("Enter Cidr Range: ");
    stdout().flush();
    stdin().read_line(&mut cidr).expect("Failed to read line");
    if let Some('\n') = cidr.chars().next_back() {
        cidr.pop();
    }
    if let Some('\r') = cidr.chars().next_back() {
        cidr.pop();
    }



    let net: Ipv4Net = cidr.parse().unwrap();
    let start_ip = net.network().to_string();
    let end_ip = net.broadcast().to_string();
    let ip_iter = Ipv4AddrRange::new(
        start_ip.parse().unwrap(),
        end_ip.parse().unwrap(),
    );
    println!("Total ip's in range: {}", ip_iter.count());
    // let mut count : i64 = 0;
    println!("Start ip: {}, End ip: {}",start_ip, end_ip);
    print!("Press enter to continue: ");
    stdout().flush();
    stdin().read_line(&mut "".to_string());
    ip_iter.for_each(|ip| {
        let add: std::net::IpAddr = ip.to_string().parse().unwrap();
        println!("{}", add);
    });
}
