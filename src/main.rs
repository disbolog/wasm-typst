use chrono::NaiveDate;
use gantt_purely_rust::create_timeline;

fn main() {
    let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();

    println!("With weekends:");
    let timeline = create_timeline(start, end, true);
    println!("{}", timeline.format_timeline());

    println!("\nWithout weekends:");
    let timeline_no_weekends = create_timeline(start, end, false);
    println!("{}", timeline_no_weekends.format_timeline());
}
