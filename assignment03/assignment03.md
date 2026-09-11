# Assignment 03 - Check the sky before you leave

## Topic: Enums and pattern matching | Language: **Rust** | Repo: `rust-03-enums`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

Before stepping out for an outing, you first check the sky. If it is Clear, you walk. If there is a Thunderstorm, you wait until conditions improve. If you find Haze, you check the PSI number before deciding what to do.

A PSI of 80 and a PSI of 250 are both haze, but the number drives completely different actions. The PSI value is what determines the decision.

You are building a program that reads the sky and tells you what to do about it.


## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile and print one `TODO step 4`.

The starter code already contains the enum. Do not change the enum variants.

## The enum

```rust
enum Sky {
    Clear,
    Drizzle,
    Thunderstorm { lightning: bool },
    Haze(u32),
}
```

| Variant | Carries |
|---|---|
| `Clear` | Nothing |
| `Drizzle` | Nothing |
| `Thunderstorm { lightning }` | A named `bool` field |
| `Haze(psi)` | One PSI value |

PSI bands:

| PSI | Meaning |
|---|---|
| 0 to 100 | Fine |
| 101 to 200 | Unhealthy |
| 201 and above | Very unhealthy |

## Step 1 - Name the condition

Complete `name` using the table below:

| Variant | Return |
|---|---|
| `Clear` | `"clear"` |
| `Drizzle` | `"drizzle"` |
| `Thunderstorm` | `"thunderstorm"` |
| `Haze` | `"haze"` |

Match all four variants. Do not use `_`.

This is the same `match` from Assignments 01 and 02, now matching on enum variants instead of numbers or booleans - which is the actual topic of this assignment.

```rust
fn name(sky: &Sky) -> &'static str {
    match sky {
        Sky::Clear => "clear",
        Sky::Drizzle => "drizzle",
        // Add Thunderstorm and Haze.
    }
}
```

Use `Sky::Thunderstorm { .. }` when the lightning value does not matter. Use `Sky::Haze(_)` when the PSI value does not matter.

Run `cargo run`.

## Step 2 - Give advice

Complete `advice` using the table below:

| Condition | Return |
|---|---|
| Clear | `"walk, it's fine"` |
| Drizzle | `"bring a brolly"` |
| Thunderstorm with lightning | `"stay indoors"` |
| Thunderstorm without lightning | `"use the covered walkway"` |
| Haze, PSI 0 to 100 | `"normal activities"` |
| Haze, PSI 101 to 200 | `"reduce outdoor exertion"` |
| Haze, PSI 201 and above | `"stay indoors, N95 if you must"` |

Bind the data inside the variants:

```rust
Sky::Thunderstorm { lightning } => { }
Sky::Haze(psi) => { }
```

This is the new part: pattern matching doesn't just pick a branch, it can pull the data out of the variant into a variable (`lightning`, `psi`) in the same step.

Because `sky` is a reference, compare `*lightning` and `*psi`.

Run `cargo run`.

## Step 3 - Decide whether walking is safe

Complete `walk_ok`.

Return `true` for:

- `Clear`
- `Drizzle`
- `Haze` with PSI 100 or below

Return `false` for every other condition.

```rust
fn walk_ok(sky: &Sky) -> bool {
    match sky {
        // Clear and Drizzle are both fine to walk in.
        // Thunderstorm is never fine to walk in.
        // Haze depends on the PSI value.
    }
}
```

`Sky::Clear | Sky::Drizzle => true` is another new pattern-matching feature: `|` means "match either of these," letting one arm cover two variants at once.

Run `cargo run`.

## Step 4 - Build the forecast line

Complete `forecast_line`:

```text
clear | walk, it's fine | walk
haze | stay indoors, N95 if you must | do not walk
```

The line contains the condition name, advice, and walking decision.

No new construct here either - `forecast_line` just calls the three functions you already wrote and formats the result.

Call `name`, `advice`, and `walk_ok`. Use `format!` to build and return the line. Use `println!` in `main` to display it.

After completing `forecast_line`, add the sample forecast values to `main` and
print one line for each value:

```rust
let week = [
    Sky::Clear,
    Sky::Drizzle,
    Sky::Thunderstorm { lightning: true },
    Sky::Thunderstorm { lightning: false },
    Sky::Haze(172),
    Sky::Haze(341),
];

for sky in &week {
    println!("  {}", forecast_line(sky));
}
```

Run:

```text
cargo run
```

Expected completed output:

```text
TODAY'S SKY

  clear | walk, it's fine | walk
  drizzle | bring a brolly | walk
  thunderstorm | stay indoors | do not walk
  thunderstorm | use the covered walkway | do not walk
  haze | reduce outdoor exertion | do not walk
  haze | stay indoors, N95 if you must | do not walk
```

## Step 5 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the four functions you just completed. For each one, pick your own `Sky` value and assert what it should return - like Assignment 01's Step 6, one test per function you wrote.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_matches_my_sky() {
        // Build a Sky value and assert what name should return for it.
    }

    #[test]
    fn advice_matches_my_sky() {
        // Build a Sky value and assert what advice should return for it.
    }

    #[test]
    fn walk_ok_matches_my_sky() {
        // Build a Sky value and assert what walk_ok should return for it.
    }

    #[test]
    fn forecast_line_matches_my_sky() {
        // Build a Sky value and assert what forecast_line should return for it.
    }
}
```

If you want a suggestion: the PSI 100/101 boundary is the trickiest case for `walk_ok` (`Haze(100)` should still be walkable, `Haze(101)` should not) - but any `Sky` value you can correctly justify is fine for any of the four.

Run `cargo test`. All four should pass.

## What to Hand In

1. Complete Steps 1 to 4 in `src/main.rs`.
2. Run `cargo run` and check the forecast.
3. Add the tests from Step 5 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]` if it is present.
5. Run `cargo check`.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`name`) correctly implemented | 20 |
| Step 2 (`advice`) correctly implemented | 20 |
| Step 3 (`walk_ok`) correctly implemented | 20 |
| Step 4 (`forecast_line`) correctly implemented | 20 |
| `cargo test` runs, all Step 5 tests pass | 10 |
