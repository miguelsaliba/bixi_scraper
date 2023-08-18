mod db;
mod models;
mod schema;

use diesel::prelude::*;
use models::StationStatus;
use serde::Deserialize;
use std::time::Duration;
use tokio::{task, time};

#[derive(Deserialize)]
#[allow(dead_code)]
struct L1 {
    last_updated: i64,
    data: L2,
}

#[derive(Deserialize)]
struct L2 {
    stations: Vec<StationStatus>,
}

#[tokio::main]
async fn main() {
    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(15 * 60));

        loop {
            interval.tick().await;
            let stations = fetch_station_status().await;
            let result = save_status_to_db(stations).await;
            match result {
                Ok(num) => println!("{:?} : {}", chrono::offset::Local::now(), num),
                Err(err) => eprintln!("{:?}", err),
            }
        }
    });

    let _ = forever.await;

    // println!(
    //     "{:?}",
    //     stations
    //         .iter()
    //         .find(|&station| station.station_id == 80)
    //         .unwrap()
    // );
}

async fn fetch_station_status() -> Vec<StationStatus> {
    const BIXI_URL: &str = "https://gbfs.velobixi.com/gbfs/en/station_status.json";
    let result = reqwest::get(BIXI_URL).await.unwrap();

    result.json::<L1>().await.unwrap().data.stations
}

async fn save_status_to_db(status: Vec<StationStatus>) -> Result<usize, diesel::result::Error> {
    use crate::schema::station_status;
    let mut connection = db::establish_connection();

    diesel::insert_into(station_status::table)
        .values(&status)
        .on_conflict_do_nothing()
        .execute(&mut connection)
}
