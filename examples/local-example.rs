#[cfg(not(feature = "tokio-cache"))]
use std::collections::HashMap;
#[cfg(not(feature = "tokio-cache"))]
use std::str::FromStr;
#[cfg(not(feature = "tokio-cache"))]
use std::thread::sleep;
#[cfg(not(feature = "tokio-cache"))]
use std::time::Duration;
#[cfg(not(feature = "tokio-cache"))]
use chrono::{DateTime, Utc};
#[cfg(not(feature = "tokio-cache"))]
use full_dataset_cache::processors::RawLineMapProcessor;
#[cfg(not(feature = "tokio-cache"))]
use full_dataset_cache::sources::LocalFileConfigSource;
#[cfg(not(feature = "tokio-cache"))]
use full_dataset_cache::cache::{Error, Fallback, MirrorCache, OnFailure, OnUpdate, Result};
#[cfg(not(feature = "tokio-cache"))]
use full_dataset_cache::collections::UpdatingMap;
#[cfg(not(feature = "tokio-cache"))]
use full_dataset_cache::metrics::Metrics;

#[cfg(feature = "tokio-cache")]
fn main() {}

#[cfg(not(feature = "tokio-cache"))]
fn main() {
    let source = LocalFileConfigSource::new("./src/bin/my.config");
    let processor = RawLineMapProcessor::new(parse_line);

    let cache = MirrorCache::<UpdatingMap<u128, String, i32>>::map_builder()
        // These are required.
        .with_source(source)
        .with_processor(processor)
        .with_fetch_interval(Duration::from_secs(2))
        // These are optional
        .with_name("my-cache")
        .with_fallback(Fallback::with_value(HashMap::new()))
        .with_update_callback(OnUpdate::with_fn(|_, v, _| println!("Updated to version {}", v.unwrap_or(0))))
        .with_failure_callback(OnFailure::with_fn(|e, _| println!("Failed with error: {}", e)))
        .with_metrics(ExampleMetrics::new())
        .build().unwrap();

    // Collection instances are safe to hold on to, borrow, clone, or pass ownership of.
    let map = cache.get_collection();
    loop {
        println!("C={}", map.get(&String::from("C")).unwrap_or_default());
        sleep(Duration::from_secs(3));
    }
}

#[cfg(not(feature = "tokio-cache"))]
fn parse_line(raw: String) -> Result<Option<(String, i32)>> {
    if raw.trim().is_empty() || raw.starts_with('#') {
        return Ok(None);
    }

    if let Some((k, v)) = raw.split_once('=') {
        Ok(Some((String::from(k), i32::from_str(v)?)))
    } else {
        Err(Error::new(format!("Failed to parse '{}'", raw).as_str()))
    }
}

#[cfg(not(feature = "tokio-cache"))]
struct ExampleMetrics {}

#[cfg(not(feature = "tokio-cache"))]
impl Metrics<u128> for ExampleMetrics {
    fn update(&self, _new_version: &Option<u128>, fetch_time: Duration, process_time: Duration) {
        println!("Update fetch took {}ms and process took {}ms", fetch_time.as_millis(), process_time.as_millis());
    }

    fn last_successful_update(&self, ts: &DateTime<Utc>) {
        println!("Last successful update is now at {}", ts);
    }

    fn check_no_update(&self, check_time: &Duration) {
        println!("File hasn't changed. Check in {}ms", check_time.as_millis())
    }

    fn last_successful_check(&self, ts: &DateTime<Utc>) {
        println!("Last successful check is now at {}", ts);
    }

    fn fallback_invoked(&self) {
        println!("Fallback invoked!");
    }

    fn fetch_error(&self, err: &Error) {
        println!("Fetch failed with: '{}'", err)
    }

    fn process_error(&self, err: &Error) {
        println!("Process failed with: '{}'", err)
    }
}

#[cfg(not(feature = "tokio-cache"))]
impl ExampleMetrics {
    fn new() -> ExampleMetrics {
        ExampleMetrics {}
    }
}