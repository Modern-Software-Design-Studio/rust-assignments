# Assignment 02 - Unlimited, until it slows down

## Topic: Structs and methods | Language: **Rust** | Repo: `rust-02-structs`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

In Singapore, an unlimited phone bundle can still include a provider-set high-speed data limit. Your connection may continue after the limit, but the provider can reduce your speed or restrict hotspot use. This exercise focuses on building a Rust app that checks whether you are still within the full-speed allowance or whether throttling has begun.

A sample plan is provided to test your app after completing the steps. The program will verify whether the provider-set limit has been reached and show how much high-speed data remains. If the limit is reached, the report shows zero high-speed data remaining, meaning the provider has started limiting the speed.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile and show the unfinished method
  results.

Use the sample plan first. Match the required output first.

After that works, you may try your own plan. Use your provider app or bill for
the plan values. Enter the monthly price in cents and data amounts in whole GB.
For an unlimited plan, enter the high-speed allowance, not the word unlimited.

## The starter code

| Item | What it is |
|---|---|
| `money` | Given. Formats cents as dollars |
| `DataPlan` | Given. Stores the plan details |
| `usage_band` | Step 1. Classifies data usage |
| `remaining_data_gb` | Step 2. Calculates remaining data |
| `summary` | Step 3. Builds the report |

All three methods are inside `impl DataPlan` - an **impl block**, where a struct's methods live. `&self` in each method's signature is a borrowed reference to the specific `DataPlan` the method was called on; that's what lets you write `self.field_name` to read one of its fields. This is the topic of this assignment: methods live next to the data they act on, instead of being separate functions that take the struct as an argument.

## Step 1 - Classify data usage

Complete `usage_band` using the table below:

| Data used | Band |
|---|---|
| Half the limit or less | `"LOW"` |
| More than half up to the limit | `"OK"` |
| More than the limit | `"OVER"` |

For a 10 TB plan, 5,000 GB is `LOW` and 5,001 GB is `OK`. 10,001 GB is `OVER`.

```rust
fn usage_band(&self) -> &'static str {
    // Match self.data_used_gb against the plan limit - self.data_limit_gb.
}
```

Use `if`, `else if`, and `else`:

1. If usage is at most half the limit, return `"LOW"`.
2. Else if usage is at most the full limit, return `"OK"`.
3. Otherwise, return `"OVER"`.

This is the same control-structure family as Assignment 01's `match` - `match` picks one arm from a list of patterns; this picks one branch from a list of yes/no conditions, checked top to bottom.

Run `cargo run`.

## Step 2 - Calculate remaining data

Complete `remaining_data_gb`:

```rust
fn remaining_data_gb(&self) -> u32 {
    // Return the data limit minus the data used.
}
```

If usage reaches or passes the provider-set limit, return `0` instead of a
negative number. This represents the point where the provider may begin to
limit the speed. Use `saturating_sub`.

Like Step 2 of Assignment 01, this is a single expression, not a control structure - `self.data_limit_gb.saturating_sub(self.data_used_gb)` already returns the right number directly, no `if` needed.

Run `cargo run`.

## Step 3 - Build the summary

Complete `summary` so it returns this exact text:

```text
TELCOM Mobile - Unlimited Max
  price     $25.00
  used      10700/10000 GB (OVER)
  remaining 0 GB
```

Use `format!` inside `summary` to build and return the text. Use `println!` in `main` to display it.

`format!` creates text. `println!` displays text.

This step introduces nothing new either - `summary` just calls the two methods you already wrote (`self.usage_band()`, `self.remaining_data_gb()`) and formats their results.

Call `usage_band()` and `remaining_data_gb()`. Format the price with `money`.

Run:

```text
cargo run
```

## Step 4 - Use the sample plan

Use the sample plan first. This is the required check.

No new construct here either - you're building a `DataPlan` value and calling the methods you already wrote on it.

Add its provider, plan name, monthly price, high-speed data limit, and current
data usage to `DataPlan`.

The sample `DataPlan` is already in `main`. Use these plan details:

```rust
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
```

The `usage band:` and `remaining:` lines are checkpoints - they let you confirm Steps 1 and 2 work as soon as you finish each one, before `summary` (Step 3) exists to combine them. Keep all four lines; do not delete the first two once `summary` is done.

The sample plan costs $25.00 per month. Its provider-set high-speed limit is
10,000 GB, and it has used 10,700 GB. This is above the limit, so the report
should show zero remaining data and explain that speed is throttled.

Use these values:

| Field | Enter |
|---|---|
| `provider` | `"TELCOM Mobile"` |
| `plan_name` | `"Unlimited Max"` |
| `monthly_price_cents` | `2_500` for $25.00 |
| `data_limit_gb` | `10_000` GB |
| `data_used_gb` | `10_700` GB |

The `_` separates digits for readability. `10_000` means `10000`.

Run `cargo run`. The unfinished methods will show their TODO or placeholder
results. Replace each method body as you complete Steps 1 to 3.

Your completed `main` should keep the plan and method calls:

```rust
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
```

The `usage band:` and `remaining:` lines are checkpoints - they let you confirm Steps 1 and 2 work as soon as you finish each one, before `summary` (Step 3) exists to combine them. Keep all four lines; do not delete the first two once `summary` is done.

Use these values for the required check. Keep the field names and method calls
unchanged.

After your output matches, you may replace the sample values with your own plan.
Do this only after the required check works.

## Step 5 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the three methods you just completed. For each one, pick your own input and assert what it should return - like Assignment 01's Step 6, one test per function you wrote.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_band_matches_my_plan() {
        // Build a DataPlan of your choosing and assert what usage_band() should return.
    }

    #[test]
    fn remaining_data_matches_my_plan() {
        // Build a DataPlan of your choosing and assert what remaining_data_gb() should return.
    }

    #[test]
    fn summary_matches_my_plan() {
        // Build a DataPlan of your choosing and assert what summary() should return.
    }
}
```

If you want suggestions: `OVER` is the one `usage_band` case that changes what the report actually says; a plan already at or past its limit is the trickiest `remaining_data_gb` case, since it should not go negative - but any inputs you can correctly justify are fine here.

Run `cargo test`. All three should pass.

## What to Hand In

1. Complete `src/main.rs`.
2. Complete Step 4 with the sample plan details.
3. Add the tests from Step 5 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]` from `main.rs`.
5. Run `cargo check`, then run `cargo run` and check the summary.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`usage_band`) correctly implemented | 20 |
| Step 2 (`remaining_data_gb`) correctly implemented | 20 |
| Step 3 (`summary`) correctly implemented | 20 |
| Step 4, sample plan wired in exactly as required | 20 |
| `cargo test` runs, all Step 5 tests pass | 10 |
