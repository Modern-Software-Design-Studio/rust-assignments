# Assignment 08 - The notice board

## Topic: Traits | Language: **Rust** | Repo: `rust-08-traits`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

You are building the code behind a shared notice board. A talk gets pinned to it with a title and a time. A lost item gets pinned with a description and a contact number. A product listing gets pinned with a price. These are three different things, and the code has to print all of them.

The notice board does not need to know whether the item is a talk, a lost item, or a product listing. It only asks two questions: what is your headline, and are you urgent. Give any pinned item answers to those two questions, and the board can print all of them in one list.

You could use an enum the way Assignment 03 did and match on it, but look at the cost. In Assignment 03, three different functions matched on `Sky`. Add a fifth kind of weather, and all three functions must be updated before the code will even build.

That is the point of a trait. It tells Rust what actions a type must support. It is like a contract that lists the functions a type must provide. If a type can perform those actions, Rust accepts it. The actual type does not matter. Only the actions matter.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile and print four `TODO step 4` lines.

The starter code already contains `Event`, `LostItem`, `ForSale`, `money`, and the `Notice` trait itself. Do not change them.

## The trait

```rust
trait Notice {
    fn headline(&self) -> String;
    fn is_urgent(&self) -> bool;
}
```

Any type that implements both of these methods is a `Notice`, as far as the board is concerned. It does not matter what fields the type has, or what it is called.

```rust
impl Notice for Event {
    fn headline(&self) -> String { /* ... */ }
    fn is_urgent(&self) -> bool { /* ... */ }
}
```

`&self` is the value the method was called on, borrowed. Same as Assignment 02's methods.

## Step 1 - Implement `Notice` for `Event`

Complete the `impl Notice for Event` block.

`headline` returns:

```text
Talk: Designing for Repair, Thu 7pm, Campus Centre
```

`title`, then a comma, then `when`.

An event is never urgent. `is_urgent` is already correct.

Run `cargo run`.

## Step 2 - Implement `Notice` for `LostItem`

Complete the `impl Notice for LostItem` block.

`headline` returns:

```text
Lost: student card near B59, contact 9123 4567
```

`what`, then a comma, then the contact.

A lost item is always urgent. Fix `is_urgent` to return `true`.

Run `cargo run`.

## Step 3 - Implement `Notice` for `ForSale`

Complete the `impl Notice for ForSale` block.

`headline` returns:

```text
For sale: desk lamp, $8.00
```

`what`, then a comma, then the price, formatted with the given `money`.

A listing is urgent only when the price is under $5.00. Use `self.price_cents < 500`.

Run `cargo run`.

## Step 4 - Print any notice

Complete `board_line`. If the notice is urgent, prefix `[URGENT] ` onto its headline; otherwise just the headline.

```rust
fn board_line(notice: &dyn Notice) -> String {
    // If notice.is_urgent(), prefix "[URGENT] " onto notice.headline().
    // Otherwise, just the headline.
}
```

`&dyn Notice` means "a reference to something, any something, as long as it implements `Notice`." This function has never heard of `Event`, `LostItem`, or `ForSale`. It only calls the two methods the trait promises are there.

Run `cargo run`.

## Step 5 - Check the board

Expected completed output:

```text
NOTICE BOARD

  Talk: Designing for Repair, Thu 7pm, Campus Centre
  [URGENT] Lost: student card near B59, contact 9123 4567
  For sale: desk lamp, $8.00
  [URGENT] For sale: textbook, $0.00
```

Look at the last two lines. Both are `ForSale`, printed by the same call to `board_line`, and one is urgent while the other is not. The type did not change. The data inside it did, and `is_urgent` read it correctly both times.

## Step 6 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the four things you just completed: `Event`, `LostItem`, and `ForSale`'s trait implementations, plus `board_line`. For each one, pick your own values and assert what it should return.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_matches_my_values() {
        // Build an Event and assert what headline() and is_urgent() should return.
    }

    #[test]
    fn lost_item_matches_my_values() {
        // Build a LostItem and assert what headline() and is_urgent() should return.
    }

    #[test]
    fn for_sale_matches_my_values() {
        // Build a ForSale and assert what headline() and is_urgent() should return.
    }

    #[test]
    fn board_line_matches_my_notice() {
        // Build an Event, LostItem, or ForSale, and assert what board_line(&it) should return.
    }
}
```

If you want a suggestion: `LostItem` is always urgent, and a `ForSale` priced right at $5.00 is the trickiest boundary to check - but any values you can correctly justify are fine for any of the four.

Run `cargo test`. All four should pass.

## What to Hand In

1. Complete Steps 1 to 4 in `src/main.rs`.
2. Keep the `Notice` trait and its method signatures unchanged.
3. Add the tests from Step 6 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]`.
5. Run `cargo check`, then run `cargo run`.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`Event`) correctly implemented | 20 |
| Step 2 (`LostItem`) correctly implemented | 20 |
| Step 3 (`ForSale`) correctly implemented | 20 |
| Step 4 (`board_line`) correctly implemented, using the trait alone | 20 |
| `cargo test` runs, all Step 6 tests pass | 10 |
