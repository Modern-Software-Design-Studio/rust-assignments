// Assignment 05 - Build the screen time report
#![allow(unused_variables, dead_code)]

// ---- Given, the Assignment 04 shape already applied. Do not change. ----

#[derive(Debug, PartialEq)]
struct App {
    name: &'static str,
    minutes: u32,
}

// Finds the app with the most minutes. This only works on App.
// Step 1 is this function with the App taken out.
fn most_used(apps: &[App]) -> Option<&App> {
    let mut best = apps.first()?;

    for app in apps {
        if app.minutes > best.minutes {
            best = app;
        }
    }

    Some(best)
}

// ---- Your task ----

// Step 1 - Return the largest item. None when the slice is empty.
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    None
}

// Step 2 - Return the smallest item. None when the slice is empty.
fn smallest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    None
}

// Step 3 - Count the items strictly above the threshold.
// The function header is already provided. Complete only the function body.


fn count_above<T: PartialOrd>(items: &[T], threshold: &T) -> usize {
    0
}

fn main() {
    // Swap in your own screen time. Otherwise leave this.
    let apps = vec![
        App { name: "Telegram", minutes: 860 },
        App { name: "Instagram", minutes: 410 },
        App { name: "Brawl Stars", minutes: 300 },
        App { name: "YouTube", minutes: 240 },
        App { name: "Calculator", minutes: 4 },
    ];

    let minutes: Vec<u32> = apps.iter().map(|app| app.minutes).collect();
    let names: Vec<&str> = apps.iter().map(|app| app.name).collect();
    let daily_hours = [4.2, 6.8, 3.1, 5.5, 2.0, 6.1, 4.9];

    println!("SCREEN TIME - LAST 7 DAYS\n");

    match most_used(&apps) {
        Some(app) => println!("  most used       {} ({} min)", app.name, app.minutes),
        None => println!("  most used       no apps logged"),
    }

    match largest(&minutes) {
        Some(value) => println!("  most minutes    {} min", value),
        None => println!("  most minutes    none"),
    }

    match smallest(&minutes) {
        Some(value) => println!("  least minutes   {} min", value),
        None => println!("  least minutes   none"),
    }

    match largest(&daily_hours) {
        Some(value) => println!("  busiest day     {:.1} h", value),
        None => println!("  busiest day     none"),
    }

    match smallest(&daily_hours) {
        Some(value) => println!("  quietest day    {:.1} h", value),
        None => println!("  quietest day    none"),
    }

    match smallest(&names) {
        Some(value) => println!("  first A to Z    {}", value),
        None => println!("  first A to Z    none"),
    }

    match largest(&names) {
        Some(value) => println!("  last A to Z     {}", value),
        None => println!("  last A to Z     none"),
    }

    println!("\n  apps over 300 min: {}", count_above(&minutes, &300));
}
