// Assignment 03 - Check the sky before you leave
#![allow(unused_variables, dead_code)]

// ---- Your task ----

// Three variant shapes in one type.
//   Clear and Drizzle carry nothing.
//   Thunderstorm carries a named field.
//   Haze carries one unnamed value, the PSI reading.
#[derive(Debug, PartialEq)]
enum Sky {
    Clear,
    Drizzle,
    Thunderstorm { lightning: bool },
    Haze(u32),
}

// Step 1 - Name it. The data does not matter here.
fn name(sky: &Sky) -> &'static str {
    "TODO step 1"
}

// Step 2 - Here write the code for what to do about it.
// Two arms need the data they carry. The haze arm needs Assignment 01's banding.
fn advice(sky: &Sky) -> &'static str {
    "TODO step 2"
}

// Step 3 - Here write the code that decides whether walking is a good idea.
fn walk_ok(sky: &Sky) -> bool {
    false
}

// Step 4 - Here write the code that builds one forecast line.
// Call steps 1, 2 and 3.
fn forecast_line(sky: &Sky) -> String {
    "TODO step 4".to_string()
}

fn main() {
    println!("TODAY'S SKY\n");
    println!("  TODO step 4");

    // After completing forecast_line, add the forecast values and print them.
}

