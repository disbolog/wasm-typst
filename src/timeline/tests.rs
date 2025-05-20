use chrono::NaiveDate;
use indoc::indoc;
use super::timeline_grouping::group_timeline;
#[test]
fn test_workweek_basic() {
    // Test a basic work week (Mon-Fri)
    // 2025-01-06 (Mon) to 2025-01-10 (Fri)
    let start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();

    let result = group_timeline(start, end, false);

    // Should show 5 days (Mon-Fri)
    assert_eq!(result.days.titles, vec!["6", "7", "8", "9", "10"]);
    assert_eq!(result.days.delimiters, vec![0, 1, 2, 3, 4, 5]);

    // Should show one week
    assert_eq!(result.weeks.titles, vec!["2"]);
    assert_eq!(result.weeks.delimiters, vec![0, 5]);
}

#[test]
fn test_full_week_with_weekends() {
    // Test a full week including weekends
    // 2025-01-04 (Sat) to 2025-01-10 (Fri)
    let start = NaiveDate::from_ymd_opt(2025, 1, 4).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();

    let result = group_timeline(start, end, true);

    // Should show all 7 days
    assert_eq!(result.days.titles, vec!["4", "5", "6", "7", "8", "9", "10"]);
    assert_eq!(result.days.delimiters, vec![0, 1, 2, 3, 4, 5, 6, 7]);

    // Should show two weeks (week 1: Sat-Sun, week 2: Mon-Fri)
    assert_eq!(result.weeks.titles, vec!["1", "2"]);
    assert_eq!(result.weeks.delimiters, vec![0, 2, 7]);
}

#[test]
fn test_month_transition() {
    // Test transition between months
    // 2025-01-30 to 2025-02-03
    let start = NaiveDate::from_ymd_opt(2025, 1, 30).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 2, 3).unwrap();

    let result = group_timeline(start, end, false);

    // Should show workdays only
    assert_eq!(result.days.titles, vec!["30", "31", "3"]);

    assert_eq!(result.weeks.titles, vec!["5", "6"]);
    assert_eq!(result.weeks.delimiters, vec![0, 2, 3]);

    // Should show both months
    assert_eq!(result.months.titles, vec!["Jan", "Feb"]);
    assert_eq!(result.months.delimiters, vec![0, 2, 3]);
}

#[test]
fn test_quarter_transition() {
    // Test transition between quarters
    // 2025-03-31 to 2025-04-02
    let start = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 4, 2).unwrap();

    let result = group_timeline(start, end, false);

    // Should show both quarters
    assert_eq!(result.quarters.titles, vec!["Q1", "Q2"]);
    assert_eq!(result.quarters.delimiters, vec![0, 1, 3]);

    // Should show both months
    assert_eq!(result.months.titles, vec!["Mar", "Apr"]);
    assert_eq!(result.months.delimiters, vec![0, 1, 3]);

    // Week 14 spans both quarters
    assert_eq!(result.weeks.titles, vec!["14"]);
    // Delimiters: [start=0, Q1 end=1, end=3]
    assert_eq!(result.weeks.delimiters, vec![0, 3]);
}

#[test]
fn test_year_transition() {
    // Test transition between years
    // 2025-12-31 to 2026-01-02
    let start = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 2).unwrap();

    let result = group_timeline(start, end, false);

    // Should show both years
    assert_eq!(result.years.titles, vec!["25", "26"]);
    assert_eq!(result.years.delimiters, vec![0, 1, 3]);

    // Should show both months
    assert_eq!(result.months.titles, vec!["Dec", "Jan"]);

    // Should show both quarters
    assert_eq!(result.quarters.titles, vec!["Q4", "Q1"]);
}

#[test]
fn test_weekend_skipping() {
    // Test that weekends are properly skipped
    // 2025-01-03 (Fri) to 2025-01-07 (Tue)
    let start = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 1, 7).unwrap();

    let result = group_timeline(start, end, false);

    // Should skip weekend (Jan 4-5) and show only Fri and Tue
    assert_eq!(result.days.titles, vec!["3", "6", "7"]);
    assert_eq!(result.days.delimiters, vec![0, 1, 2, 3]);

    // Should show both weeks
    assert_eq!(result.weeks.titles, vec!["1", "2"]);
}

#[test]
fn test_year_end_transition() {
    // Test transition over year end with weekends hidden
    // 2025-12-25 (Thu) to 2026-01-06 (Mon)
    let start = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 6).unwrap();

    let result = group_timeline(start, end, false);

    // Should show workdays only (skipping weekends)
    // Dec: Thu 25, Fri 26, Mon 29, Tue 30, Wed 31
    // Jan: Thu 1, Fri 2, Mon 5, Tue 6
    assert_eq!(result.days.titles, vec!["25", "26", "29", "30", "31", "1", "2", "5", "6"]);
    assert_eq!(result.days.delimiters, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Should show all relevant weeks
    // Week 52 (2025): Dec 25-26 [0-2]
    // Week 1 (2026): Dec 29-31, Jan 1-2 [2-7]
    // Week 2 (2026): Jan 5-6 [7-9]
    assert_eq!(result.weeks.titles, vec!["52", "1", "2"]);
    assert_eq!(result.weeks.delimiters, vec![0, 2, 7, 9]);

    // Should show both months
    assert_eq!(result.months.titles, vec!["Dec", "Jan"]);
    assert_eq!(result.months.delimiters, vec![0, 5, 9]);

    // Should show both quarters
    assert_eq!(result.quarters.titles, vec!["Q4", "Q1"]);
    assert_eq!(result.quarters.delimiters, vec![0, 5, 9]);

    // Should show both years
    assert_eq!(result.years.titles, vec!["25", "26"]);
    assert_eq!(result.years.delimiters, vec![0, 5, 9]);
}

#[test]
fn test_output_format() {
    // Test the string formatting for 2025-12-25 to 2026-01-06 with weekends hidden
    let start = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 6).unwrap();

    let result = group_timeline(start, end, false);

    let output = result.format_timeline();
    let expected = indoc! {"
        |                 25|             26|
        |                 Q4|             Q1|
        |                Dec|            Jan|
        |     52|                  1|      2|
        | 25| 26| 29| 30| 31|  1|  2|  5|  6|"}.trim_end();

    // Use println! to show both actual and expected output with proper formatting
    println!("\nActual output:\n{}\n", output);
    println!("Expected output:\n{}\n", expected);

    // Use assert! with Debug format to get better error messages
    assert!(output == expected, "\nExpected:\n{:#?}\n\nGot:\n{:#?}", expected, output);
}

#[test]
fn test_output_format_with_weekdays() {
    // Test the string formatting for 2025-12-25 to 2026-01-06 with weekends hidden
    let start = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 6).unwrap();

    let result = group_timeline(start, end, true);

    let output = result.format_timeline();
    let expected = indoc! {"
        |                         25|                     26|
        |                         Q4|                     Q1|
        |                        Dec|                    Jan|
        |             52|                          1|      2|
        | 25| 26| 27| 28| 29| 30| 31|  1|  2|  3|  4|  5|  6|"}.trim_end();

    // Use println! to show both actual and expected output with proper formatting
    println!("\nActual output:\n{}\n", output);
    println!("Expected output:\n{}\n", expected);

    // Use assert! with Debug format to get better error messages
    assert!(output == expected, "\nExpected:\n{:#?}\n\nGot:\n{:#?}", expected, output);
}

#[test]
fn test_output_format_25_to_26() {
    // Test the string formatting for 2025-12-25 to 2026-01-06 with weekends hidden
    let start = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

    let result = group_timeline(start, end, true);

    let output = result.format_timeline();
    let expected = indoc! {"
        | 25| 26|
        | Q4| Q1|
        |Dec|Jan|
        |      1|
        | 31|  1|"}.trim_end();

    // Use println! to show both actual and expected output with proper formatting
    println!("\nActual output:\n{}\n", output);
    println!("Expected output:\n{}\n", expected);

    // Use assert! with Debug format to get better error messages
    assert!(output == expected, "\nExpected:\n{:#?}\n\nGot:\n{:#?}", expected, output);
}
