-- Add migration script here

-- Your SQL goes here
DROP TABLE IF EXISTS candle
;
CREATE TABLE candle
(
    id numeric(8,0) NOT NULL,
    symbol character varying(8) NOT NULL,
    minutes numeric(5,0) NOT NULL,
    open numeric(20,8) NOT NULL,
    high numeric(20,8) NOT NULL,
    low numeric(20,8) NOT NULL,
    close numeric(20,8) NOT NULL,
    volume numeric(20,8) NOT NULL,
    open_time timestamp with time zone NOT NULL,
    close_time timestamp with time zone NOT NULL,
    CONSTRAINT candle_pkey PRIMARY KEY (id),
    CONSTRAINT start_time UNIQUE (symbol, minutes, open_time)
)
