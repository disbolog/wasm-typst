//! Gantt-style timeline visualization library in pure Rust
//!
//! This library provides functionality to create and format timeline visualizations,
//! supporting various time periods (days, weeks, months, quarters, years).

pub mod timeline;

pub use timeline::{GroupResult, TimelineGroups, create_timeline};

use wasm_minimal_protocol::*;
use ciborium::{de::from_reader, ser::into_writer};

initiate_protocol!();

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn create_timeline_wasm(params: &[u8]) -> Result<Vec<u8>, String> {
    use std::io::Cursor;

    // Deserialize parameters using a cursor for the byte slice
    let params: timeline::timeline_grouping::TimelineParams = from_reader(Cursor::new(params))
        .map_err(|e| e.to_string())?;

    // Create timeline
    let timeline = create_timeline(params.start_date, params.end_date, params.show_weekends);

    // Serialize result
    let mut out = Vec::new();
    into_writer(&timeline, &mut out)
        .map_err(|e| e.to_string())?;

    Ok(out)
}
