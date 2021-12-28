use serde_json::{Result, Value};
use reqwest::header;
use reqwest::blocking;
use serde::{Serialize, Deserialize};

pub enum VehicleMethod{
    Bus,
    Train,
    Other
}

#[derive(Serialize, Deserialize)]
pub struct GtfsRoute{
 pub id: i64,
 pub route_id: String,
 pub agency_id: String,
 pub route_short_name:  String,
 pub route_long_name: String,
 pub route_desc: String,
 pub route_type: i64,
 pub route_color: String,
 pub route_text_color: String,
 pub route_url: String,
}

pub struct GtfsVehiclePos{
    pub timestamp: i64,
    pub position_id: String,
    pub longitude: f64, 
    pub latitude: f64,
    pub bearing: i64,
    pub vehicle_id: String,
    pub schedule_relation: i64,
    pub start_time: String,
    pub trip_id: String,
}

pub fn fetch_vehicles_v1(token: String) -> Vec<GtfsVehiclePos>{
    let url = String::from("https://api.opendata.metlink.org.nz/v1/gtfs-rt/vehiclepositions");
    let client = reqwest::blocking::Client::new();
    let request = client.get(&url).header("x-api-key",token).header(reqwest::header::ACCEPT,"application/json");
    let met_response = request.send().unwrap();
    let met_code = met_response.status();
    let mut vehicles = Vec::new();
    if met_code.is_success(){ 
    let body = met_response.text().unwrap();
        vehicles = parse_vehicles(body);
    }else   {
    println!("Request Failed. Status: {:?}", met_code);
    return vehicles;
    }
    return vehicles;
}
pub fn fetch_routes_v1(token: String) -> Vec<GtfsRoute>{
    let url = String::from("https://api.opendata.metlink.org.nz/v1/gtfs/routes");
    let client = reqwest::blocking::Client::new();
    let request = client.get(&url).header("x-api-key",token).header(reqwest::header::ACCEPT,"application/json");
    let met_response = request.send().unwrap();
    let met_code = met_response.status();
    let mut routes = Vec::new();
    if met_code.is_success(){ 
        let body = met_response.text().unwrap();
        routes = parse_routes(body);
    }else   {
        println!("Request Failed. Status: {:?}", met_code);
        return routes;
    }
    return routes;
}
//parse json api from routes feed
pub fn parse_routes(contents: String)-> Vec<GtfsRoute>{
    let v: Vec<GtfsRoute> = serde_json::from_str(&contents).expect("failed to parse as json");
    return v;
}
// parse a json response from realtime feed
pub fn parse_vehicles(contents: String)-> Vec<GtfsVehiclePos>{
    let v: Value = serde_json::from_str(&contents).expect("failed to parse as json");
    //load tests/fixtures/gtfs-rt-position.json
    let header = v["header"].as_object().unwrap();
    let timestamp = header["timestamp"].as_i64().unwrap();
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
            timestamp: timestamp,
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
    fn struct_create_pos() {
        let mut file = File::open("tests/fixtures/gtfs-rt-position.json").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        //println!("{}", contents);
        let trips = parse_vehicles(contents);
        let first_pos = trips.first().unwrap();
        assert_eq!(String::from("2__0__717__NBM__8__8_1"),first_pos.trip_id);
        assert_eq!(174.7761536,first_pos.longitude);
    }
    #[test]
    fn struct_crete_route(){
        let mut file = File::open("tests/fixtures/gtfs-route.json").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        let route: GtfsRoute = serde_json::from_str(contents.as_str()).expect("failed to parse route data");
        assert_eq!(String::from("1"),route.route_short_name);
        assert_eq!(3,route.route_type);
    }
}