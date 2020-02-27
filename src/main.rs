use std::env;

mod data;
mod route_service;
mod convert_service;

fn main() {
    let args: Vec<String> = env::args().collect();
    let map = data::read_data();

    if args.len() > 1 && &args[1] == "num" {
        convert_service::bin_convert(map);
    } else {      
        route_service::turn_count(map);
    }
}
