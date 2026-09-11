// Assignment 09 - The vending machine
#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::io::{self, BufRead};

// ---- Given. Already working. Do not change. ----

// code, name, price in cents, starting stock
const SLOT_DATA: [(&str, &str, u32, u32); 12] = [
    ("A1", "Coke", 120, 5),
    ("A2", "Sprite", 120, 0),
    ("A3", "Water", 80, 8),
    ("A4", "Iced Milo", 150, 3),
    ("B1", "Chips", 130, 6),
    ("B2", "Pretzels", 130, 4),
    ("B3", "Chocolate", 180, 2),
    ("B4", "Chips (Spicy)", 130, 1),
    ("C1", "Gum", 50, 10),
    ("C2", "Mints", 50, 9),
    ("C3", "Cough Drops", 90, 7),
    ("C4", "Tissues", 100, 3),
];

// The only real coins and notes accepted, in cents.
const VALID_DENOMINATIONS: [u32; 8] = [5, 10, 20, 50, 100, 200, 500, 1000];

fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

// ---- Your task ----
// Fill in this structure. Keep the names below so the program stays organised.

struct Slot {
    name: &'static str,
    price_cents: u32,
    stock: u32,
}

struct Machine {
    slots: HashMap<String, Slot>,
    order: Vec<String>,
    balance: u32,
    selected: Option<String>,
    card_attempts: u32,
}

impl Machine {
    fn new() -> Self {
        let mut slots = HashMap::new();
        let mut order = Vec::new();

        for (code, name, price, stock) in SLOT_DATA {
            slots.insert(code.to_string(), Slot { name, price_cents: price, stock });
            order.push(code.to_string());
        }

        Machine {
            slots,
            order,
            balance: 0,
            selected: None,
            card_attempts: 0,
        }
    }

    fn handle(&mut self, line: &str) -> String {
        // Step 2: split the line into a command and an argument.
        // Step 3: match the command and call the correct helper.
        "TODO handle command".to_string()
    }

    fn do_select(&mut self, code: &str) -> String {
        "TODO select".to_string()
    }

    fn do_insert(&mut self, rest: &str) -> String {
        "TODO insert".to_string()
    }

    fn do_pay(&mut self, method: &str) -> String {
        "TODO pay".to_string()
    }

    fn do_refund(&mut self) -> String {
        "TODO refund".to_string()
    }

    fn do_list(&self) -> String {
        "TODO list".to_string()
    }

    fn complete_sale(&mut self, code: &str, price: u32, name: &'static str, method: &str) -> String {
        "TODO complete sale".to_string()
    }

    fn dispense(&mut self, code: &str) -> String {
        "TODO dispense".to_string()
    }
}

fn main() {
    let mut machine = Machine::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(value) => value,
            Err(_) => break,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        println!("{}", machine.handle(trimmed));
    }
}