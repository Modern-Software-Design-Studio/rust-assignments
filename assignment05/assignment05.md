# Assignment 05 - Build the screen time report

## Topic: Generics | Language: **Rust** | Repo: `rust-05-generics`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

Your phone already tracks your screen time. You are building your own version of that in Rust.

You want to answer three questions about three different things: which app used the most minutes, which day had the fewest hours, and how many apps went over 300 minutes. Minutes, hours, and app names are different types, for that reason Rust will not let you reuse the same function unless the types match. 
If you answer the same question for all three the normal way, you end up writing three separate functions, one per type, each doing very similar comparison in three different places.

Before USB-C, every phone had its own charger. iPhone used one shape, Android used another. The job was the same, but the cables were all different. USB-C solved that with one rule: any device can use the port as long as the plug fits.

A generic function works the same way; It follows one rule. `<T: PartialOrd>` means the type must support comparison. If it can be compared with `<` and `>`, the function can handle it. One function covers all of them. No separate version for minutes, another for hours, another for names.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile. The unfinished generic methods
    will show `none` and `0` in the report.

The starter code already contains the `App` struct and `most_used`. Do not change them.

## Read `most_used` first

`most_used` is written for you. It finds the app with the most minutes, and it only works on `App`.

```rust
fn most_used(apps: &[App])             -> Option<&App>
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T>
```

Step 1 is `most_used` with the `App` taken out.

`T` stands for a type decided at each call. Call `largest` with `&[u32]` and `T` is `u32`. Call it with `&[&str]` and `T` is `&str`.

`<T: PartialOrd>` is a promise that `T` can be compared with `<` and `>`. Without it the compiler rejects the comparison. It also gives you nothing else, so `.max()` and `.sort()` are not available.

## Step 1 - Return the largest item

Complete `largest`.

Return:

- `None` when the slice is empty.
- `Some(&T)` for the largest item.
- The first item when two items are equal.

Start with:

```rust
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut best = items.first()?;
    // Loop over items and update best when an item is larger.
    Some(best)
}
```

The `?` here works the same way as Assignment 04's `longest` - it returns `None` early if `items` is empty.

`most_used` compares `app.minutes` because it knows it holds apps. `largest` does not know what it holds, so compare the items themselves with `item > best`.

Run `cargo run`. Three lines change: most minutes, busiest day, and last A to Z.

## Step 2 - Return the smallest item

Complete `smallest`. Same rules, opposite comparison.

```rust
fn smallest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    // Same as step 1 with the comparison flipped.
}
```

Run `cargo run`.

## Step 3 - Count items above a threshold

Complete `count_above`.

Return how many items are strictly greater than `threshold`. An item equal to the threshold does not count.

```rust
fn count_above<T: PartialOrd>(items: &[T], threshold: &T) -> usize {
    // Count the items greater than threshold.
}
```

`threshold` is a `&T`. The function compares against the value, it does not take it.

`usize` is the type Rust uses for counts.

Run `cargo run`.

## Step 4 - Check the report

Every line should now hold a value, and none should say `none`.

`largest` and `smallest` are each called on three different types in `main`. If either one only works on numbers, the program will not compile.

Expected completed output:

```text
SCREEN TIME - LAST 7 DAYS

  most used       Telegram (860 min)
  most minutes    860 min
  least minutes   4 min
  busiest day     6.8 h
  quietest day    2.0 h
  first A to Z    Brawl Stars
  last A to Z     YouTube

  apps over 300 min: 2
```

## Step 5 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the three functions you just completed. For each one, pick your own slice and assert what it should return.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_matches_my_items() {
        // Pick a slice and assert what largest should return.
    }

    #[test]
    fn smallest_matches_my_items() {
        // Pick a slice and assert what smallest should return.
    }

    #[test]
    fn count_above_matches_my_items() {
        // Pick a slice and a threshold, and assert what count_above should return.
    }
}
```

If you want a suggestion: including an item exactly equal to your threshold is the trickiest case for `count_above`, since it should not count - but any slice (numbers, floats, or text) you can correctly justify is fine for any of the three.

Run `cargo test`. All three should pass.

## What to Hand In

1. Complete Steps 1 to 3 in `src/main.rs`.
2. Keep the generic signatures unchanged.
3. Add the tests from Step 5 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]`.
5. Run `cargo check`, then run `cargo run`.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`largest`) correctly implemented | 20 |
| Step 2 (`smallest`) correctly implemented | 20 |
| Step 3 (`count_above`) correctly implemented | 20 |
| Step 4, all three work across types without a separate copy per type | 20 |
| `cargo test` runs, all Step 5 tests pass | 10 |
