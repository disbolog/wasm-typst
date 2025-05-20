pub mod timeline_grouping;
pub mod format;

#[cfg(test)]
mod tests;

// Re-export primary types and functions
pub use self::timeline_grouping::{GroupResult, TimelineGroups, create_timeline};
