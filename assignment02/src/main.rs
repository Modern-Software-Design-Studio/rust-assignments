// Assignment 02 - Unlimited, until it slows down
#![allow(unused_variables, dead_code)]

// Given: turns 350 into "$3.50".
fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

// ---- Your task ----

// A plan keeps related data together.
#[derive(Debug, PartialEq)]
struct DataPlan {
    provider: &'static str,
    plan_name: &'static str,
    monthly_price_cents: u32,
    data_limit_gb: u32,
    data_used_gb: u32,
}

impl DataPlan {
    // Step 1: classify data usage.
    fn usage_band(&self) -> &'static str {
        "TODO step 1" // TODO step 1
    }

    // Step 2: calculate the data remaining this month.
    fn remaining_data_gb(&self) -> u32 {
        0 // TODO step 2
    }

    // Step 3: build the monthly summary.
    fn summary(&self) -> String {
        "TODO step 3".to_string() // TODO step 3
    }
}

fn main() {
    let plan = DataPlan {
        provider: "TELCOM Mobile",
        plan_name: "Unlimited Max",
        monthly_price_cents: 2_500,
        data_limit_gb: 10_000,
        data_used_gb: 10_700,
    };

    println!("PHONE DATA PLAN\n");
    println!("usage band: {}", plan.usage_band());
    println!("remaining: {} GB", plan.remaining_data_gb());
    println!("{}", plan.summary());
}



