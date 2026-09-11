// Assignment 07 - Where the canteen money went
#![allow(unused_variables, dead_code)]

use std::collections::HashMap;

// ---- Given. Already working. Do not change. ----

struct Purchase {
    stall: &'static str,
    cents: u32,
}

// Every stall that appears in RECEIPTS, in the order the report prints them.
// A HashMap does not remember the order things were put in, so the report
// walks this fixed list instead of the map, and looks each stall up.
const STALLS: [&str; 4] = ["Stall 1", "Stall 2", "Stall 4", "Stall 9"];

const RECEIPTS: [Purchase; 14] = [
    Purchase { stall: "Stall 1", cents: 350 },
    Purchase { stall: "Stall 9", cents: 140 },
    Purchase { stall: "Stall 2", cents: 820 },
    Purchase { stall: "Stall 1", cents: 350 },
    Purchase { stall: "Stall 4", cents: 420 },
    Purchase { stall: "Stall 9", cents: 140 },
    Purchase { stall: "Stall 1", cents: 350 },
    Purchase { stall: "Stall 9", cents: 140 },
    Purchase { stall: "Stall 4", cents: 420 },
    Purchase { stall: "Stall 2", cents: 820 },
    Purchase { stall: "Stall 9", cents: 140 },
    Purchase { stall: "Stall 1", cents: 420 },
    Purchase { stall: "Stall 4", cents: 420 },
    Purchase { stall: "Stall 9", cents: 140 },
];

fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

// ---- Your task ----

// Step 1 - Add up every purchase, grouped by stall.
fn total_cents_by_stall(receipts: &[Purchase]) -> HashMap<&str, u32> {
    HashMap::new()
}

// Step 2 - Count how many purchases happened at each stall.
fn visit_count_by_stall(receipts: &[Purchase]) -> HashMap<&str, u32> {
    HashMap::new()
}

// Step 3 - Find the stall with the most visits.
fn busiest_stall<'a>(counts: &'a HashMap<&'a str, u32>) -> Option<&'a str> {
    None
}

// Step 4 - Build one report line for a stall.
fn stall_line(stall: &str, totals: &HashMap<&str, u32>, counts: &HashMap<&str, u32>) -> String {
    format!("{}   TODO step 4", stall)
}

fn main() {
    let totals = total_cents_by_stall(&RECEIPTS);
    let counts = visit_count_by_stall(&RECEIPTS);

    println!("CANTEEN SPEND BY STALL\n");

    for stall in STALLS {
        println!("  {}", stall_line(stall, &totals, &counts));
    }

    let total_spend: u32 = totals.values().sum();
    let total_visits: u32 = counts.values().sum();

    match busiest_stall(&counts) {
        Some(stall) => println!(
            "\n  {} visits total | {} spent | busiest: {}",
            total_visits,
            money(total_spend),
            stall
        ),
        None => println!("\n  {} visits total | {} spent | busiest: none", total_visits, money(total_spend)),
    }
}
