// Assignment 01 - Control structures

#![allow(unused_variables, dead_code)]

// Given: electricity costs 27 cents per unit.
fn cost_cents(units: u32) -> u32 {
    units * 27
}

// Given: returns zero when usage is below the budget.
// Writing `used - budget` would crash when used is smaller, because a u32
// cannot hold a negative number. saturating_sub stops at zero instead.
fn units_over(used: u32, budget: u32) -> u32 {
    used.saturating_sub(budget)
}

// Given: formats cents as dollars.
fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

// ---- Your task ----

// Step 1: return the correct band for units over budget.
fn band(units_over: u32) -> &'static str {
    "TODO step 1"
}

// Step 2: return true only when usage is over budget.
fn over_budget(used: u32, budget: u32) -> bool {
    false
}

// Step 3: return the office action for the band.
fn action(units_over: u32) -> String {
    "TODO step 3".to_string()
}

// Step 4: return one formatted report line.
fn room_line(room: &str, budget: u32, used: u32) -> String {
    format!("{} | TODO step 4", room)
}

fn main() {
    let block_readings = [
        ("59-01-01", 5, 4),
        ("59-01-02", 13, 15),
        ("59-01-03", 5, 11),
        ("59-01-04", 5, 17),
    ];

    println!("BLOCK 59 - ELECTRICITY CHECK\n");

    let mut flagged = 0;
    let mut surcharges = 0;

    for (room, budget, used) in block_readings {
        println!("  {}", room_line(room, budget, used));

        if over_budget(used, budget) {
            flagged += 1;
        }

        let units_over = units_over(used, budget);
        if over_budget(used, budget) && units_over > 3 {
            surcharges += cost_cents(units_over);
        }
    }

    println!(
        "\n  {} rooms | {} over budget | {} in surcharges",
        block_readings.len(),
        flagged,
        money(surcharges)
    );
}

// Step 6: once Steps 1 to 4 are done, uncomment this block and run `cargo test`.
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn far_over_band() {
//         // Boundary: 9 is exactly where WELL OVER ends and FAR OVER begins.
//         assert_eq!(band(9), "FAR OVER");
//     }
//
//     #[test]
//     fn exactly_at_budget_is_not_over() {
//         // Boundary: using exactly the budget does not count as over.
//         assert_eq!(over_budget(5, 5), false);
//     }
//
//     #[test]
//     fn well_over_names_the_real_surcharge() {
//         assert_eq!(action(6), "warning emailed, $1.62 surcharge");
//     }
//
//     #[test]
//     fn room_59_01_03_reports_correctly() {
//         // The overall function: room_line calls units_over, band, and action together.
//         assert_eq!(
//             room_line("59-01-03", 5, 11),
//             "59-01-03 | budget 5 | used 11 | WELL OVER | warning emailed, $1.62 surcharge"
//         );
//     }
// }

