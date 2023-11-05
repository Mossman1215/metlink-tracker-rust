use serde_json::{Result, Value};
use reqwest::header;
use reqwest::blocking;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap};

pub enum VehicleMethod{
    Bus,
    Train,
    Other
}
#[derive(Serialize, Deserialize)]
struct Stop{ id: i64, location_type: i64, parent_station: String, stop_code: String, stop_desc: String, stop_id: String, stop_lat: f64, stop_lon: f64, stop_name: String, stop_timezone: String, stop_url: String, zone_id: String }
#[derive(Serialize, Deserialize)]
struct Trip{ bikes_allowed: i64, block_id: String, direction_id: i64, route_id: i64, service_id: String, shape_id: String, trip_headsign: String, trip_id: String, wheelchair_accessible: i64 }
#[derive(Serialize, Deserialize)]
struct ShapePt{ id: i64, shape_dist_traveled: i64, shape_id: String, shape_pt_lat: f64, shape_pt_lon: f64, shape_pt_sequence: i64 }

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
//shapes are seqences of lat long points with an identifier
#[derive(Serialize, Deserialize)]
pub struct GtfsShape{
    pub id: String,
    pub pt_lat: f64,
    pub pt_lon: f64,
    pub pt_sequence: i64,
    pub dist_travelled: f64
}
#[derive(Serialize, Deserialize)]
pub struct GtfsTrip{
    pub id: i64,
    pub route_id: i64,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: String,
    pub direction_id: i64,
    pub block_id: String,
    pub shape_id: String,
    pub wheelchair_accessible: i64,
    pub bikes_allowed: i64
}
//TODO: re-use the client and create constructor function for the lib object?
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
pub fn fetch_trips_v1(token: String) -> Vec<GtfsTrip>{
    let url = String::from("https://api.opendata.metlink.org.nz/v1/gtfs/trips");
    let client = reqwest::blocking::Client::new();
    let request = client.get(&url).header("x-api-key",token).header(reqwest::header::ACCEPT,"application/json");
    let met_response = request.send().unwrap();
    let met_code = met_response.status();
    let mut trips = Vec::new();
    if met_code.is_success(){ 
    let body = met_response.text().unwrap();
        trips = parse_trips(body);
    }else   {
    println!("Request Failed. Status: {:?}", met_code);
    return trips;
    }
    return trips;
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
pub fn parse_trips(contents: String)-> Vec<GtfsTrip>{
    let v: Vec<GtfsTrip> = serde_json::from_str(&contents).expect("failed to parse as json");
    return v;
}