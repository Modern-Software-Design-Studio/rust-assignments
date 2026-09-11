// Assignment 04 - Track the weekly commute
#![allow(unused_variables, dead_code)]

// ---- Given, the Assignment 03 skill already applied. Do not change. ----

fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

#[derive(Debug, PartialEq)]
enum Mode {
    Walk { minutes: u32 },
    Cycle { minutes: u32 },
    Bus { service: u32 },
    Mrt { stops: u32 },
    Grab { fare_cents: u32 },
}

#[derive(Debug, PartialEq)]
struct Trip {
    day: &'static str,
    mode: Mode,
}

fn cost_cents(mode: &Mode) -> u32 {
    match mode {
        Mode::Walk { .. } => 0,
        Mode::Cycle { .. } => 0,
        Mode::Bus { .. } => 109,
        Mode::Mrt { stops } => 90 + stops * 4,
        Mode::Grab { fare_cents } => *fare_cents,
    }
}

fn minutes(mode: &Mode) -> u32 {
    match mode {
        Mode::Walk { minutes } => *minutes,
        Mode::Cycle { minutes } => *minutes,
        Mode::Bus { .. } => 31,
        Mode::Mrt { stops } => 5 + stops * 3,
        Mode::Grab { .. } => 12,
    }
}

fn mode_name(mode: &Mode) -> String {
    match mode {
        Mode::Walk { .. } => "Walk".to_string(),
        Mode::Cycle { .. } => "Cycle".to_string(),
        Mode::Bus { service } => format!("Bus {}", service),
        Mode::Mrt { stops } => format!("MRT {} stops", stops),
        Mode::Grab { .. } => "Grab".to_string(),
    }
}

// ---- Your task ----

// Step 1 - Here write the code that totals what the week cost.
fn total_cost_cents(week: &[Trip]) -> u32 {
    0
}

// Step 2 - Here write the code that finds the longest trip.
// None if the week is empty. On a tie, the first.
fn longest(week: &[Trip]) -> Option<&Trip> {
    None
}

// Step 3 - Here write the code that flags trips costing more than the limit.
// In order. References, not copies.
fn over(week: &[Trip], limit_cents: u32) -> Vec<&Trip> {
    Vec::new()
}

fn main() {
    // Swap in your own week. Otherwise leave this one.
    let week = vec![
        Trip { day: "Mon", mode: Mode::Mrt { stops: 6 } },
        Trip { day: "Tue", mode: Mode::Bus { service: 8 } },
        Trip { day: "Wed", mode: Mode::Walk { minutes: 14 } },
        Trip { day: "Thu", mode: Mode::Grab { fare_cents: 940 } },
        Trip { day: "Fri", mode: Mode::Cycle { minutes: 18 } },
    ];

    println!("HOW I GOT TO CLASS\n");

    for trip in &week {
        println!(
            "  {}   {:<12} {:>6}   {} min",
            trip.day,
            mode_name(&trip.mode),
            money(cost_cents(&trip.mode)),
            minutes(&trip.mode)
        );
    }

    println!("\n  total {}", money(total_cost_cents(&week)));

    match longest(&week) {
        Some(trip) => println!("  longest   {}, {} min", trip.day, minutes(&trip.mode)),
        None => println!("  longest   no trips logged"),
    }

    println!("  {} trip(s) over $2.00", over(&week, 200).len());

    // Three checks ran and the week is still ours.
    println!("  {} trips still logged", week.len());
}

