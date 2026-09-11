// Assignment 06 - Sort out the locker bookings
#![allow(unused_variables, dead_code)]

// ---- Given. Already working. Do not change. ----

#[derive(Debug, PartialEq)]
struct Booking {
    locker: String,
    student: String,
    months: u32,
}

// The lockers that exist on this floor.
const LOCKERS: [&str; 6] = ["L-102", "L-107", "L-110", "L-115", "L-118", "L-120"];

// Nobody may hold a locker for longer than this.
const MAX_MONTHS: u32 = 12;

fn locker_exists(id: &str) -> bool {
    LOCKERS.contains(&id)
}

// The requests, straight off the sign-up sheet.
const REQUESTS: &str = "\
L-102, 1004512, 6
L-118, 1004988, 12

L-102, 1005231, 6
L-999, 1004433, 6
L-107, 1004512, 6
L-110, 1004777, twelve
L-115, 1004120, 24
L-120, 1004300
L-107, 1005644, 3";

// ---- Your task ----

#[derive(Debug, PartialEq)]
enum BookingError {
    Empty,
    WrongFieldCount(usize),
    UnknownLocker(String),
    BadMonths(String),
    TooLong(u32),
    Taken(String),
    AlreadyHasOne(String),
}

// Step 1 - Read the months field. It has to be a number, and it has to be allowed.
fn parse_months(field: &str) -> Result<u32, BookingError> {
    Err(BookingError::BadMonths(field.to_string()))
}

// Step 2 - Turn one line into a Booking. Format: locker, student, months
fn parse_booking(line: &str) -> Result<Booking, BookingError> {
    Err(BookingError::Empty)
}

// Step 3 - Read the whole sheet. Keep the good bookings, record the bad lines.
fn process(text: &str) -> (Vec<Booking>, Vec<(usize, BookingError)>) {
    (Vec::new(), Vec::new())
}

fn main() {
    let (confirmed, rejected) = process(REQUESTS);

    println!("LOCKER BOOKINGS\n");

    println!("  {} confirmed", confirmed.len());
    for booking in &confirmed {
        println!(
            "    {}   {}   {} months",
            booking.locker, booking.student, booking.months
        );
    }

    println!("\n  {} rejected", rejected.len());
    for (line_number, error) in &rejected {
        println!("    line {}: {:?}", line_number, error);
    }
}
