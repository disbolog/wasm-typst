#{
    // Create a timeline for 2024 Q1
    let timeline_plugin = plugin("./target/wasm32-unknown-unknown/release/gantt_purely_rust.wasm")

    let timeline = timeline_plugin.create_timeline_wasm(cbor.encode((
        start_date: datetime(year: 2024, month: 1, day: 1),
        end_date: datetime(year: 2024, month: 3, day: 31),
        show_weekends: true,
    )))

    // Decode the timeline data
    let data = cbor(timeline)

    // Print the data structure
    [
        *Timeline Structure*
        Days: #data.days.titles.len() days
        Weeks: #data.weeks.titles.join(", ")
        Months: #data.months.titles.join(", ")
        Quarters: #data.quarters.titles.join(", ")
        Years: #data.years.titles.join(", ")
    ]
}
