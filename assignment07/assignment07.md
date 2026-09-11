# Assignment 07 - Where the canteen money went

## Topic: Collections | Language: **Rust** | Repo: `rust-07-collections`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

You have been collecting canteen receipts all month, and you have no idea how fast you have exhausted the food budget you set aside. Fourteen purchases, four stalls, no particular order.

Group them by stall, total each one, and find out which stall got the most visits. You do not know the stall names ahead of time, so you end up checking every receipt against every stall, one at a time. That repeated scanning is exactly what a `Vec` forces on you. A `HashMap` skips it entirely: look a stall up directly, no scanning. That is what collections, this assignment's topic, actually gives you: the structure that fits the shape of the problem, instead of forcing a list to do a lookup's job.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile and show `0 visits total | $0.00 spent | busiest: none`, with every stall line saying `TODO step 4`.

The starter code already contains the `Purchase` struct, `STALLS`, `RECEIPTS`, and `money`. Do not change them.

## What a `HashMap` gives you

```rust
use std::collections::HashMap;

let mut totals: HashMap<&str, u32> = HashMap::new();
*totals.entry("Stall 1").or_insert(0) += 350;
```

`.entry(key).or_insert(default)` looks up the key. If it is already there, you get a mutable reference to its value. If it is not, it is inserted with the default first, then you get the same reference. One line handles both cases.

`HashMap` does not remember the order things were added, which is why `STALLS` exists separately. The report walks `STALLS` in a fixed order and looks each one up, rather than walking the map itself.

## Step 1 - Total spend by stall

Complete `total_cents_by_stall`.

For every purchase, add its `cents` to the running total for its `stall`.

```rust
fn total_cents_by_stall(receipts: &[Purchase]) -> HashMap<&str, u32> {
    let mut totals = HashMap::new();
    // Loop over receipts. For each one, add its cents to totals.
    totals
}
```

Run `cargo run`.

## Step 2 - Visit count by stall

Complete `visit_count_by_stall`. Same idea, counting instead of summing.

```rust
fn visit_count_by_stall(receipts: &[Purchase]) -> HashMap<&str, u32> {
    let mut counts = HashMap::new();
    // Loop over receipts. For each one, add 1 to counts.
    counts
}
```

Run `cargo run`.

## Step 3 - Find the busiest stall

Complete `busiest_stall`.

Return the stall with the highest visit count. On a tie, either one is acceptable, because the sample data has none.

```rust
fn busiest_stall<'a>(counts: &'a HashMap<&'a str, u32>) -> Option<&'a str> {
    let mut best: Option<(&str, u32)> = None;
    // Loop over counts with `for (&stall, &count) in counts`.
    // Keep the stall with the highest count seen so far.
    best.map(|(stall, _)| stall)
}
```

`for (&stall, &count) in counts` unpacks both the key and the value out of each entry in one go.

Run `cargo run`.

## Step 4 - Build one stall's report line

Complete `stall_line`. Look up the stall's total and visit count, then build the report line.

```rust
fn stall_line(stall: &str, totals: &HashMap<&str, u32>, counts: &HashMap<&str, u32>) -> String {
    // Look up stall's total and visit count. Build the line with format!.
}
```

This is the same `Option` idea as Assignment 04's `longest` and Assignment 05's `largest` - `HashMap` can't guarantee the key exists, so `.get()` hands back a maybe instead of crashing. `.get(stall)` on a `HashMap` returns an `Option<&u32>`, because the key might not be there. `.copied().unwrap_or(0)` turns that into a plain `u32`, defaulting to 0 if the stall never appears.

Run `cargo run`.

## Step 5 - Check the report

Expected completed output:

```text
CANTEEN SPEND BY STALL

  Stall 1   4 visits   $14.70
  Stall 2   2 visits   $16.40
  Stall 4   3 visits   $12.60
  Stall 9   5 visits   $7.00

  14 visits total | $50.70 spent | busiest: Stall 9
```

## Step 6 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the four functions you just completed. For each one, pick your own purchases or counts and assert what it should return.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_cents_matches_my_receipts() {
        // Build a &[Purchase] of your choosing and assert what total_cents_by_stall should return.
    }

    #[test]
    fn visit_count_matches_my_receipts() {
        // Build a &[Purchase] of your choosing and assert what visit_count_by_stall should return.
    }

    #[test]
    fn busiest_stall_matches_my_counts() {
        // Build a small HashMap<&str, u32> with counts of your choosing.
        // Assert busiest_stall(&counts) returns the stall with the highest count.
    }

    #[test]
    fn stall_line_matches_my_data() {
        // Build small totals and counts maps and assert what stall_line should return for one stall.
    }
}
```

If you want a suggestion: at least three stalls with clearly different counts makes `busiest_stall`'s result unambiguous - but any data you can correctly justify is fine for any of the four.

Run `cargo test`. All four should pass.

## What to Hand In

1. Complete Steps 1 to 4 in `src/main.rs`.
2. Keep the given signatures unchanged.
3. Add the tests from Step 6 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]`.
5. Run `cargo check`, then run `cargo run`.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`total_cents_by_stall`) correctly implemented | 20 |
| Step 2 (`visit_count_by_stall`) correctly implemented | 20 |
| Step 3 (`busiest_stall`) correctly implemented | 20 |
| Step 4 (`stall_line`) correctly implemented | 20 |
| `cargo test` runs, all Step 6 tests pass | 10 |
