// Assignment 08 - The notice board
#![allow(unused_variables, dead_code)]

// ---- Given. Already working. Do not change. ----

struct Event {
    title: &'static str,
    when: &'static str,
}

struct LostItem {
    what: &'static str,
    contact: &'static str,
}

struct ForSale {
    what: &'static str,
    price_cents: u32,
}

fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

// The board holds a mix of the three kinds above, but it only knows about
// them through this trait. It never sees Event, LostItem, or ForSale by name.
trait Notice {
    fn headline(&self) -> String;
    fn is_urgent(&self) -> bool;
}

// ---- Your task ----

// Step 1 - Implement Notice for Event.
// Never urgent.
impl Notice for Event {
    fn headline(&self) -> String {
        "TODO step 1".to_string()
    }

    fn is_urgent(&self) -> bool {
        false
    }
}

// Step 2 - Implement Notice for LostItem.
// Always urgent.
impl Notice for LostItem {
    fn headline(&self) -> String {
        "TODO step 2".to_string()
    }

    fn is_urgent(&self) -> bool {
        false
    }
}

// Step 3 - Implement Notice for ForSale.
// Urgent only when the price is under $5.
impl Notice for ForSale {
    fn headline(&self) -> String {
        "TODO step 3".to_string()
    }

    fn is_urgent(&self) -> bool {
        false
    }
}

// Step 4 - Print one board line for anything that implements Notice.
// Mark urgent notices with [URGENT] in front.
fn board_line(notice: &dyn Notice) -> String {
    "TODO step 4".to_string()
}

fn main() {
    let event = Event { title: "Designing for Repair", when: "Thu 7pm, Campus Centre" };
    let lost = LostItem { what: "student card near B59", contact: "9123 4567" };
    let cheap = ForSale { what: "desk lamp", price_cents: 800 };
    let free = ForSale { what: "textbook", price_cents: 0 };

    let board: Vec<&dyn Notice> = vec![&event, &lost, &cheap, &free];

    println!("NOTICE BOARD\n");

    for notice in &board {
        println!("  {}", board_line(*notice));
    }
}
