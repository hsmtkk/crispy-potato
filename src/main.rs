use pnet::datalink::interfaces;

fn main() {
    let all_interfaces = interfaces();

    let default_interface = all_interfaces
    .iter()
    .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    match default_interface {
        Some(interface) => println!("Found default interface with [{}]", interface),
        None => println!("Error while finding the default interface"),
    }
}
