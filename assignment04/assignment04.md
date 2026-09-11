# Assignment 04 - Track the weekly commute

## Topic: Ownership and borrowing | Language: **Rust** | Repo: `rust-04-ownership`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

You top up your transport card again on Sunday, sooner than you expected to. You cannot say what actually used it up. Most days it was the MRT. You used the bus once when it rained. Thursday you overslept and took a Grab without checking the fare first.

You want to check three things from the week: the total spent, the longest ride, and which trips cost too much.

Rust has one main rule here: one variable owns the data.

If you pass the week into a function the plain way, that function takes ownership. After that, `main` cannot use the week again.

Use `&week` instead. This borrows the week. The function can read it, and `main` can still use it afterward.

That is the point of this exercise: borrow the week, do not take it away. Build a program that gets all three answers and still has the week available at the end.


## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile and show placeholder results.

The starter code already contains the `Mode` enum, the `Trip` struct, and the helper functions. Do not change them.

## The important signatures

```rust
fn total_cost_cents(week: &[Trip]) -> u32
fn longest(week: &[Trip]) -> Option<&Trip>
fn over(week: &[Trip], limit_cents: u32) -> Vec<&Trip>
```

The `&` means the functions borrow the week. They can read it, but they do not take it away from `main`.

A `Vec<&Trip>` stores references to trips already inside the week. Do not clone the trips.

## Step 1 - Calculate the weekly cost

Complete `total_cost_cents`.

Add the cost of every trip. An empty week returns `0`.

```rust
fn total_cost_cents(week: &[Trip]) -> u32 {
    let mut total = 0;
    // Loop over week and add cost_cents(&trip.mode).
    total
}
```

Use `let mut` because `total` changes inside the loop. Keep the `&[Trip]` parameter.

Run `cargo run`.

## Step 2 - Find the longest trip

Complete `longest`.

Return:

- `None` when the week is empty.
- `Some(&Trip)` for the longest trip.
- The first trip when two trips have the same duration.

Start with:

```rust
fn longest(week: &[Trip]) -> Option<&Trip> {
    let mut best = week.first()?;
    // Loop over the week and update best when a trip is longer.
    Some(best)
}
```

The `?` after `week.first()` is new: if the week is empty, `first()` returns `None`, and `?` immediately returns that `None` from `longest` without you writing an `if`.

Use `minutes(&trip.mode)` to compare trips. The returned reference still belongs to the original week.

Run `cargo run`.

## Step 3 - Find expensive trips

Complete `over`.

Return every trip whose cost is strictly greater than `limit_cents`, in the original order.

A trip exactly at the limit is not over the limit.

```rust
fn over(week: &[Trip], limit_cents: u32) -> Vec<&Trip> {
    // Create a list of references to trips above the limit.
}
```

Use `Vec::new()` and `.push()`, or use `.iter().filter(...).collect()`. That second option is a new style: instead of a loop that pushes matching items one at a time, `.filter()` keeps only the trips over the limit and `.collect()` gathers them into the `Vec` in one chained expression.

Run `cargo run`.

## Step 4 - Check the borrow

The report calls all three functions and then prints the number of trips still logged. That final count should still be `5`.

If the functions take `week` instead of `&[Trip]`, `main` loses ownership after the first call. Keep the signatures unchanged.

Expected completed output:

```text
HOW I GOT TO CLASS

  Mon   MRT 6 stops   $1.14   23 min
  Tue   Bus 8         $1.09   31 min
  Wed   Walk          $0.00   14 min
  Thu   Grab          $9.40   12 min
  Fri   Cycle         $0.00   18 min

  total $11.63
  longest   Tue, 31 min
  1 trip(s) over $2.00
  5 trips still logged
```

## Step 5 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the three functions you just completed. For each one, pick your own week (a `&[Trip]`) and assert what it should return.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_cost_matches_my_week() {
        // Build a &[Trip] (or use &[] for the empty case) and assert what total_cost_cents should return.
    }

    #[test]
    fn longest_matches_my_week() {
        // Build a &[Trip] (or use &[] for the empty case) and assert what longest should return.
    }

    #[test]
    fn over_matches_my_week() {
        // Build a &[Trip] and a limit_cents, and assert what over should return.
    }
}
```

If you want a suggestion: an empty week (`&[]`) is the trickiest case for `total_cost_cents` and `longest`, since the loop runs zero times - but any week you can correctly justify is fine for any of the three.

Run `cargo test`. All three should pass.

## What to Hand In

1. Complete Steps 1 to 3 in `src/main.rs`.
2. Keep the borrowed function signatures unchanged.
3. Add the tests from Step 5 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]`.
5. Run `cargo check`, then run `cargo run`.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`total_cost_cents`) correctly implemented | 20 |
| Step 2 (`longest`) correctly implemented | 20 |
| Step 3 (`over`) correctly implemented | 20 |
| Step 4, all functions borrow and do not clone the week | 20 |
| `cargo test` runs, all Step 5 tests pass | 10 |
