## WIP: Experimental bot trader for crypto currency

### Objectives:
* Synchronize candles from exchange into local database to provide fast back test with custom trade rules.
* Automatize live trade operations.
* Tooling to verify operations from plotted graph images.

### Current state
- [x] Read candles from Binance API
- [x] Download and synchronize missing candles in local database
- [x] Convert traditional candles to Heikin-Ashi candles
- [x] Plot image with candles and indicators
- [x] Detect top/bottom candles
- [x] Allow run with input/output stream to interop with other process
- [ ] Plot sma indicator
- [ ] Bot trade runner
- [ ] Register position, operation, profits and others
- [ ] GUI

### Sample generated graph
![plotted image](out/stock.png?sanitize=true&raw=true)

### Prerequisites

1) Rust 1.51 or greater

2) Environment variables (or .env file in current directory):
  `API_KEY` Binance API key
  `SECRET_KEY` Binance API secret
  `DATABASE_URL` Postgres database URL

3) Dependencies (due plot library)
```
sudo apt install git curl autoconf libx11-dev libfreetype6-dev libgl1-mesa-dri \
    libglib2.0-dev xorg-dev gperf g++ build-essential cmake libssl-dev \
    liblzma-dev libxmu6 libxmu-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libgles2-mesa-dev libegl1-mesa-dev libdbus-1-dev libharfbuzz-dev ccache \
    clang libunwind-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-bad1.0-dev autoconf2.13 llvm-dev
```

3) SQLx
```
cargo install sqlx-cli
```

### Setup
Steps to create Postgres database:
```
sqlx database create
sqlx migrate run
```

### Running
Example plot command, that generates image in `out/` directory.
```
cargo run --release -- --debug -y BTCUSDT -m 15 -s "2020-12-21 00:00:00" -e "2020-12-25 23:00:00" plot
```
Parameters:
-y symbol
-m minutes candle time
-s start date time
-e end date time

Other commands samples in `command/` directory.
