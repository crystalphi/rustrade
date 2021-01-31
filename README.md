## WIP: Experimental bot trader for crypto currency

Objectives:
* Synchronize candles from exchange into local database to provide fast back test with custom trade rules.
* Automatize live trade operation.
* Tooling to verify operations from plotted images.

Todo:
- [x] Download and synchronize missing candles in local database
- [x] Convert traditional candles to Heikin-Ashi candles
- [x] Plot image with candles and indicators
- [x] Detect top/bottom candles
- [x] Allow run with input/output stream to interop with other process
- [ ] Plot sma indicator
- [ ] Bot trade runner
- [ ] Register position, operation, profits and others




sqlx database create
sqlx migrate run