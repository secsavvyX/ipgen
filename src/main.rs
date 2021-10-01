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
    let ip_iter = Ipv4AddrRange::new(
        net.network().to_string().parse().unwrap(),
        net.broadcast().to_string().parse().unwrap(),
    );
    println!("Total ip's in range: {}", ip_iter.count());
    // let mut count : i64 = 0;
    ip_iter.for_each(|ip| {
        let add: std::net::IpAddr = ip.to_string().parse().unwrap();
        match lookup_addr(&add) {
            Ok(addr) => {
                println!("Host: {}", addr);
            }
            Err(err) => return (),
        };
    });
}
