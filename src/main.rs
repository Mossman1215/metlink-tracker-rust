use metlink_tracker_lib;

fn main() {
    let bus_route_one = metlink_tracker_lib::fetch_vehicles_v1(String::from("1"));
    for vehicle in bus_route_one.iter() {
        println!("{} {} {} {} {}",vehicle.service_id,vehicle.vehicle_id,vehicle.longitude, vehicle.latitude, vehicle.bearing)
    }
    let bus_route_one = metlink_tracker_lib::fetch_vehicles_v1(String::from("2"));
    for vehicle in bus_route_one.iter() {
        println!("{} {} {} {} {}",vehicle.service_id,vehicle.vehicle_id,vehicle.longitude, vehicle.latitude, vehicle.bearing)
    }
    let bus_route_one = metlink_tracker_lib::fetch_vehicles_v1(String::from("3"));
    for vehicle in bus_route_one.iter() {
        println!("{} {} {} {} {}",vehicle.service_id,vehicle.vehicle_id,vehicle.longitude, vehicle.latitude, vehicle.bearing)
    }
}
