use super::{TimelineGroups, GroupResult};

impl TimelineGroups {
    /// Creates a human-readable timeline representation with aligned time units
    pub fn format_timeline(&self) -> String {
        // Define our time units in order of display
        let time_units = [
            ("Years", &self.years),
            ("Quarters", &self.quarters),
            ("Months", &self.months),
            ("Weeks", &self.weeks),
        ];

        // Format each time unit row
        let formatted_rows: Vec<String> = time_units
            .iter()
            .map(|(_, group)| format_time_row(group))
            .map(|row| ensure_ends_with_pipe(row))
            .collect();

        // Add the days row and join everything
        [
            formatted_rows,
            vec![ensure_ends_with_pipe(format_time_row(&self.days))]
        ]
        .concat()
        .join("\n")
    }
}

/// Formats a single row of time units (years, quarters, months, weeks, or days)
pub(crate) fn format_time_row(group: &GroupResult) -> String {
    collect_row_segments(group)
        .into_iter()
        .map(format_segment)
        .collect::<String>()
}

/// A segment represents a portion of the timeline row
#[derive(Debug)]
struct TimeSegment<'a> {
    content: &'a str,    // The text to display
    span: usize,         // Number of cells this segment spans
    is_empty: bool,      // Whether this is an empty spacer segment
}

/// Collects all segments (filled and empty) for a timeline row
fn collect_row_segments(group: &GroupResult) -> Vec<TimeSegment> {
    let mut segments = Vec::new();
    let mut last_position = 0_i32;  // Using i32 to match GroupResult's delimiter type

    // Process each title and its position in the timeline
    for (idx, title) in group.titles.iter().enumerate() {
        let start_pos = group.delimiters[idx];
        let end_pos = group.delimiters[idx + 1];

        // Add empty segments if there's a gap
        if start_pos > last_position {
            segments.push(TimeSegment {
                content: "",
                span: (start_pos - last_position) as usize,
                is_empty: true,
            });
        }

        // Add the actual content segment
        segments.push(TimeSegment {
            content: title,
            span: (end_pos - start_pos) as usize,
            is_empty: false,
        });

        last_position = end_pos;
    }

    segments
}

/// Formats a single segment of the timeline
fn format_segment(segment: TimeSegment) -> String {
    const CELL_WIDTH: usize = 3;  // Each cell is 3 characters wide (|XX)

    match (segment.is_empty, segment.span) {
        // Empty segments (spacers)
        (true, span) => "|   ".repeat(span),

        // Single-cell segments (typically days)
        (false, 1) => format!("|{:>3}", segment.content),

        // Multi-cell segments (years, quarters, months, weeks)
        (false, span) => {
            let total_width = CELL_WIDTH * span + (span - 1);
            format!("|{:>width$}", segment.content, width = total_width)
        }
    }
}

/// Ensures a string ends with a pipe character
fn ensure_ends_with_pipe(s: String) -> String {
    if !s.ends_with('|') {
        s + "|"
    } else {
        s
    }
}
