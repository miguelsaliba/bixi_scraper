CREATE TABLE station_status (
  station_id SMALLINT NOT NULL,
  time TIMESTAMPTZ NOT NULL,
  num_bikes_available SMALLINT NOT NULL,
  num_ebikes_available SMALLINT NOT NULL,
  num_bikes_disabled SMALLINT NOT NULL,
  num_docks_available SMALLINT NOT NULL,
  num_docks_disabled SMALLINT NOT NULL,
  PRIMARY KEY(station_id, time)
);
