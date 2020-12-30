use metlink_tracker_lib;

fn main() {
    let bus_route_one = metlink_tracker_lib::fetch_vehicles_v1(String::from("1"));
    println!("{}",bus_route_one[0].service_id)
}
