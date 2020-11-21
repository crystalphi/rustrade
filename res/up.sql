-- Your SQL goes here
DROP TABLE IF EXISTS candle
;
CREATE TABLE candle (
	id numeric(8) PRIMARY KEY,
	symbol varchar(8) NOT NULL,
	minutes numeric(5) NOT NULL,
    open_time varchar(32) NOT NULL,
    close_time varchar(32) NOT NULL,
	"open" numeric(20,8) NOT NULL,
	high numeric(20,8) NOT NULL,
	low numeric(20,8) NOT NULL,
	"close" numeric(20,8) NOT NULL,
	volume numeric(20,8) NOT NULL
)
