use serde_json::{Result, Value};
use reqwest::header;
use reqwest::blocking;

pub enum VehicleMethod{
    Bus,
    Train,
    Other
}

pub struct GtfsVehiclePos{
    pub position_id: String,
    pub longitude: f64, 
    pub latitude: f64,
    pub bearing: i64,
    pub vehicle_id: String,
    pub schedule_relation: i64,
    pub start_time: String,
    pub trip_id: String,
}

pub fn fetch_vehicles_v1(token: String) -> Vec<MetlinkVehicle>{
    let url = String::from("https://api.opendata.metlink.org.nz/v1/gtfs-rt/vehiclepositions");
    let client = reqwest::blocking::Client::new();
    let thing = client.get(&url).header("x-api-key",token).header(reqwest::header::ACCEPT,"application/json");
    let met_response = thing.send().unwrap();
    let met_code = met_response.status();
    if met_code.is_success(){ 
    let body = met_response.text().unwrap();

    let v: Value = serde_json::from_str(&body).unwrap();
    let timestamp = v["LastModified"].as_str().unwrap();
    //if v["Services"] > 0
    let services = v["Services"].as_array().unwrap();
    let mut vehicles: Vec<MetlinkVehicle> =  Vec::new();
    if services.len() > 0 {
        for vehicle_json in services.iter(){
            let service_json = vehicle_json["Service"].as_object().unwrap();
            let method_str = service_json["Mode"].as_str().unwrap();
            let method = match method_str {
                "Bus" => VehicleMethod::Bus,
                "Train" => VehicleMethod::Train,
                _ => VehicleMethod::Other,
            };
            vehicles.push(MetlinkVehicle {
                last_modified: String::from(timestamp),
                report_time: String::from(vehicle_json["RecordedAtTime"].as_str().unwrap()),
                vehicle_id: String::from(vehicle_json["VehicleRef"].as_str().unwrap()),
                service_id: String::from(vehicle_json["ServiceID"].as_str().unwrap()),
                longitude: String::from(vehicle_json["Long"].as_str().unwrap()),
                latitude: String::from(vehicle_json["Lat"].as_str().unwrap()),
                bearing: String::from(vehicle_json["Bearing"].as_str().unwrap()),
                late: vehicle_json["BehindSchedule"].as_bool().unwrap(),
                service_direction: String::from(vehicle_json["Bearing"].as_str().unwrap()),
                destination_id: String::from(vehicle_json["DestinationStopID"].as_str().unwrap()),
                delayed_sec: vehicle_json["DelaySeconds"].as_i64().unwrap(),
                method: method,
            });
        }
    }
    return vehicles;
    }
    println!("Something else happened. Status: {:?}", met_code);
    return Vec::new();
}
// parse a json response from realtime feed
pub fn parse_vehicles(contents: String)-> Vec<GtfsVehiclePos>{
    let v: Value = serde_json::from_str(&contents).expect("failed to parse as json");
    //load tests/fixtures/gtfs-rt-position.json
    let header = v["header"].as_object().unwrap();
    //let timestamp = header["timestamp"].as_str().unwrap();
    //if v["Services"] > 0
    let entities = v["entity"].as_array().unwrap();
    //contstruct a GtfsVehiclePos from the first item in the list (.entity[0] in jq)
    let mut vehicles: Vec<GtfsVehiclePos> =  Vec::new();
    for trip in entities {
        let vehic_details = trip["vehicle"].as_object().expect("failed to decode vehicle details");
        let pos_hash = vehic_details["position"].as_object().expect("failed to decode position");
        let trip_hash = vehic_details["trip"].as_object().expect("failed to decode trip details");
        let second_vehic = vehic_details["vehicle"].as_object().unwrap();
        let first_pos = GtfsVehiclePos {
            position_id: String::from(trip["id"].as_str().unwrap()),
            longitude: pos_hash["longitude"].as_f64().unwrap(), 
            latitude: pos_hash["latitude"].as_f64().unwrap(),
            bearing: pos_hash["bearing"].as_i64().unwrap(),
            vehicle_id: String::from(second_vehic["id"].as_str().unwrap()),
            schedule_relation: trip_hash["schedule_relationship"].as_i64().unwrap(),
            start_time: String::from(trip_hash["start_time"].as_str().unwrap()),
            trip_id: String::from(trip_hash["trip_id"].as_str().unwrap()),
        };
        vehicles.push(first_pos);
    }
    return vehicles;
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, vec};
    use std::io::prelude::*;
    use std::fs::File;
    #[test]
    fn struct_create() {
        let mut file = File::open("tests/fixtures/gtfs-rt-position.json").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        //println!("{}", contents);
        let trips = parse_vehicles(contents);
        let first_pos = trips.first().unwrap();
        assert_eq!(String::from("2__0__717__NBM__8__8_1"),first_pos.trip_id);
        assert_eq!(174.7761536,first_pos.longitude);
    }
}