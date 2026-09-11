// Assignment 10 - Test the vending machine

use std::collections::HashMap;
use std::io::{self, BufRead};

// ---- Given. Already working. Do not change. ----

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

const VALID_DENOMINATIONS: [u32; 8] = [5, 10, 20, 50, 100, 200, 500, 1000];

fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

// ---- Given vending machine code. Do not edit. ----

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
        Machine { slots, order, balance: 0, selected: None, card_attempts: 0 }
    }

    fn handle(&mut self, line: &str) -> String {
        let mut parts = line.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("").trim();

        match cmd {
            "select" => self.do_select(rest),
            "insert" => self.do_insert(rest),
            "pay" => self.do_pay(rest),
            "balance" => format!("Balance: {}", money(self.balance)),
            "refund" => self.do_refund(),
            "list" => self.do_list(),
            other => format!("Unknown command: {}", other),
        }
    }

    fn do_select(&mut self, code: &str) -> String {
        if !self.slots.contains_key(code) {
            return format!("No slot {}.", code);
        }

        let (name, price, stock) = {
            let slot = self.slots.get(code).unwrap();
            (slot.name, slot.price_cents, slot.stock)
        };

        if stock == 0 {
            return format!("{} is sold out.", name);
        }

        if self.selected.as_deref() == Some(code) {
            if self.balance >= price {
                return self.dispense(code);
            }
            let need = price - self.balance;
            return format!(
                "{} is {}. You have inserted {}. Insert {} more cents.",
                name,
                money(price),
                money(self.balance),
                need
            );
        }

        self.selected = Some(code.to_string());
        format!("{} is {}. Insert cash or pay by card or scan.", name, money(price))
    }

    fn do_insert(&mut self, rest: &str) -> String {
        let cents: u32 = match rest.parse() {
            Ok(v) => v,
            Err(_) => return format!("{} is not a valid note or coin.", rest),
        };

        if !VALID_DENOMINATIONS.contains(&cents) {
            return format!("{} cents is not a valid note or coin.", cents);
        }

        self.balance += cents;
        format!("Balance: {}", money(self.balance))
    }

    fn do_pay(&mut self, method: &str) -> String {
        let code = match &self.selected {
            Some(c) => c.clone(),
            None => return "Select a slot first.".to_string(),
        };

        let (name, price, stock) = {
            let slot = self.slots.get(&code).unwrap();
            (slot.name, slot.price_cents, slot.stock)
        };

        if stock == 0 {
            return format!("{} is sold out.", name);
        }

        match method {
            "card" => {
                self.card_attempts += 1;
                if self.card_attempts % 2 == 1 {
                    "Card declined.".to_string()
                } else {
                    self.complete_sale(&code, price, name, "card")
                }
            }
            "scan" => self.complete_sale(&code, price, name, "scan"),
            other => format!("Unknown payment method: {}", other),
        }
    }

    fn complete_sale(&mut self, code: &str, price: u32, name: &'static str, method: &str) -> String {
        if let Some(slot) = self.slots.get_mut(code) {
            slot.stock -= 1;
        }
        self.selected = None;
        // Card and scan never touch the running cash balance - leave it as is.
        let label = if method == "card" { "card" } else { "PayNow" };
        format!("Paid {} by {}. Dispensed: {}.", money(price), label, name)
    }

    fn dispense(&mut self, code: &str) -> String {
        let (name, price) = {
            let slot = self.slots.get(code).unwrap();
            (slot.name, slot.price_cents)
        };

        let change = self.balance - price;

        if let Some(slot) = self.slots.get_mut(code) {
            slot.stock -= 1;
        }

        self.balance = 0;
        self.selected = None;

        if change > 0 {
            format!("Dispensed: {}. Change: {} cents.", name, change)
        } else {
            format!("Dispensed: {}.", name)
        }
    }

    fn do_refund(&mut self) -> String {
        let amount = self.balance;
        self.balance = 0;
        self.selected = None;
        format!("Refunded {}.", money(amount))
    }

    fn do_list(&self) -> String {
        let mut lines = Vec::new();
        for code in &self.order {
            let slot = &self.slots[code];
            lines.push(format!(
                "{:<4}{:<16}{:<8}x{}",
                code,
                slot.name,
                money(slot.price_cents),
                slot.stock
            ));
        }
        lines.join("\n")
    }
}

fn main() {
    let mut machine = Machine::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        println!("{}", machine.handle(trimmed));
    }
}

// ---- Your task ----
// Everything above this line is complete and working. Do not change it.
// Write your tests below, in a #[cfg(test)] mod tests block.
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn example() {
//         let mut machine = Machine::new();
//         assert_eq!(machine.handle("balance"), "Balance: $0.00");
//     }
// }
