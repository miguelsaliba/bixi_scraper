// @generated automatically by Diesel CLI.

diesel::table! {
    station (id) {
        id -> Int2,
        #[max_length = 100]
        name -> Varchar,
        lat -> Float8,
        lon -> Float8,
        capacity -> Int2,
    }
}

diesel::table! {
    station_status (station_id, time) {
        station_id -> Int2,
        time -> Timestamptz,
        num_bikes_available -> Int2,
        num_ebikes_available -> Int2,
        num_bikes_disabled -> Int2,
        num_docks_available -> Int2,
        num_docks_disabled -> Int2,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    station,
    station_status,
);
