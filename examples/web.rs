// Copyright 2020 New Relic Corporation. (for the original go-agent)
// Copyright 2020 Masaki Hara.

use newrelic_unofficial::Daemon;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let license = std::env::var("NEW_RELIC_LICENSE_KEY").unwrap();
    let daemon = Daemon::new("rust-test", &license).unwrap();
    let app = daemon.application();
    for _ in 0..120 {
        let req = http::Request::post("/upload")
            .header("Host", "example.com")
            .body(())
            .unwrap();
        let txn = app.start_web_transaction("/upload", &req);
        sleep(Duration::from_millis(500));
        drop(txn);
        sleep(Duration::from_millis(500));
    }
}