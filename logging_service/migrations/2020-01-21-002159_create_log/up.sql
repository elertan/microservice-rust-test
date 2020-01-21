-- Your SQL goes here
CREATE TYPE log_level AS ENUM ('trace', 'debug', 'info', 'warning', 'error');

CREATE TABLE log
(
    id         SERIAL PRIMARY KEY,
    level      log_level     NOT NULL,
    message    VARCHAR(2500) NOT NULL,
    json_data  VARCHAR(10000),
    created_at TIMESTAMP     NOT NULL,
    updated_at TIMESTAMP
);

