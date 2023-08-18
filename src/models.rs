use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Debug, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::station_status)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StationStatus {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub station_id: i16,
    #[serde(
        rename = "last_reported",
        deserialize_with = "deserialize_datetime_utc_from_seconds"
    )]
    pub time: DateTime<Utc>,
    pub num_bikes_available: i16,
    pub num_ebikes_available: i16,
    pub num_bikes_disabled: i16,
    pub num_docks_available: i16,
    pub num_docks_disabled: i16,
}

#[derive(Debug, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::station)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Station {
    #[serde(
        rename = "station_id",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub id: i16,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub capacity: i16,
}
