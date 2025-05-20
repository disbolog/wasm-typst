use chrono::{NaiveDate, Datelike, Weekday};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupResult {
    pub titles: Vec<String>,
    pub delimiters: Vec<i32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineGroups {
    pub days: GroupResult,
    pub weeks: GroupResult,
    pub months: GroupResult,
    pub quarters: GroupResult,
    pub years: GroupResult,
}

#[derive(Deserialize)]
pub struct TimelineParams {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub show_weekends: bool,
}

type DatePosition = (NaiveDate, i32);

// Internal helper functions
fn infinite_dates(start: NaiveDate) -> impl Iterator<Item = NaiveDate> {
    std::iter::successors(Some(start), |prev| prev.succ_opt())
}

fn is_weekend(date: &NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

fn quarter_number(month: u32) -> u32 {
    (month - 1) / 3 + 1
}

// Helper function to check if a date represents a transition point
fn is_transition<F>(date: &NaiveDate, dates: &[DatePosition], transition_check: F) -> bool
where
    F: Fn(&NaiveDate, &NaiveDate) -> bool
{
    *date == dates[0].0 || // First date is always a transition
    dates.iter()
        .find(|(prev, _)| prev.succ_opt() == Some(*date))
        .map(|(prev, _)| transition_check(prev, date))
        .unwrap_or(true)
}

// Helper function to generate group results for time periods
fn create_group_result<F, G>(
    dated_positions: &[DatePosition],
    max_pos: i32,
    transition_check: F,
    format_title: G,
) -> GroupResult
where
    F: Fn(&NaiveDate, &NaiveDate) -> bool,
    G: Fn(&NaiveDate) -> String,
{
    // Get unique transition points for titles
    let mut titles = Vec::new();
    let mut current_value = None;

    for (date, _) in dated_positions.iter() {
        let title = format_title(date);
        if current_value.as_ref() != Some(&title) {
            titles.push(title.clone());
            current_value = Some(title);
        }
    }

    // Calculate delimiters at transition points
    let delimiters = std::iter::once(0)
        .chain(dated_positions.windows(2)
            .filter(|w| transition_check(&w[0].0, &w[1].0))
            .map(|w| w[0].1 + 1))
        .chain(std::iter::once(max_pos + 1))
        .collect();

    GroupResult {
        titles,
        delimiters,
    }
}

pub(crate) fn group_timeline(start_date: NaiveDate, end_date: NaiveDate, show_weekends: bool) -> TimelineGroups {
    // Convert date range to sequence of (date, position) pairs
    let dated_positions: Vec<DatePosition> = infinite_dates(start_date)
        .take_while(|&d| d <= end_date)
        .filter(|d| show_weekends || !is_weekend(d))
        .enumerate()
        .map(|(i, d)| (d, i as i32))
        .collect();

    let max_pos = dated_positions.last().map(|(_, p)| *p).unwrap_or(0);

    // Days are handled specially since they don't need transition checks
    let days = GroupResult {
        titles: dated_positions.iter()
            .map(|(d, _)| d.day().to_string())
            .collect(),
        delimiters: (0..=max_pos + 1).collect(),
    };

    // Weeks need special handling for Monday starts and weekend transitions
    let weeks = GroupResult {
        titles: dated_positions.iter()
            .filter(|(d, _)| d.weekday() == Weekday::Mon || *d == start_date)
            .map(|(d, _)| d.iso_week().week().to_string())
            .collect(),
        delimiters: std::iter::once(0)
            .chain(dated_positions.windows(2)
                .filter(|w| {
                    let (curr, _) = w[0];
                    let (next, _) = w[1];
                    curr.iso_week().week() != next.iso_week().week()
                })
                .map(|w| w[0].1 + 1))
            .chain(std::iter::once(max_pos + 1))
            .collect(),
    };

    let months = create_group_result(
        &dated_positions,
        max_pos,
        |prev, curr| prev.month() != curr.month(),
        |d| d.format("%b").to_string()
    );

    let quarters = create_group_result(
        &dated_positions,
        max_pos,
        |prev, curr| quarter_number(prev.month()) != quarter_number(curr.month()),
        |d| format!("Q{}", quarter_number(d.month()))
    );

    let years = create_group_result(
        &dated_positions,
        max_pos,
        |prev, curr| prev.year() != curr.year(),
        |d| format!("{:02}", d.year() % 100)
    );

    TimelineGroups {
        days,
        weeks,
        months,
        quarters,
        years,
    }
}
pub fn create_timeline(start_date: NaiveDate, end_date: NaiveDate, show_weekends: bool) -> TimelineGroups {
    group_timeline(start_date, end_date, show_weekends)
}
