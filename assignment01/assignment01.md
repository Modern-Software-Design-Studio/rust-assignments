# Assignment 01 - Who's Running a Rice Cooker?

## Topic: Control structures | Language: **Rust** | Repo: `rust-01-control-structures`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

---

## What You Are Doing

Something in one hostel room is quietly pulling more electricity than it should.
The aircon is prepaid, but everything else such as laptops, fans, fridges, and the occasional banned rice cooker runs off the shared meter.
The fridge is also registered with the Office of Housing, so it consumption is accounted for.
The hostel needs a simple, reliable way to flag unusual consumption from the meter readings without manually checking every room.

Your task is to translate the policy below into four Rust functions that the hostel office can run daily/weekly.

---

## The Policy You Are Implementing

> **HOSTEL ELECTRICITY POLICY**
> Assume this is what the handbook says. Every rule your code needs is here.
> Nothing else is invented, and nothing is left for you to guess.

**Clause 1 - Registered budget.** Each room declares extra appliances at check-in and the office converts that into a weekly allowance for monitoring.

| Declared | Weekly allowance |
|---|---|
| Base estimate, every room: laptop, fan, desk lamp | 5 units |
| Registered fridge | plus 8 units |

So a room with a registered fridge has a budget of 13 units. A room without has 5. Using exactly or a little past the allowance is fine.

**Clause 2 - Overage bands.** How far past the allowance a room went, in whole units.

| Units over budget | Band |
|---|---|
| 0 | `WITHIN BUDGET` |
| 1 to 3 | `SLIGHTLY OVER` |
| 4 to 8 | `WELL OVER` |
| 9 and above | `FAR OVER` |

**Clause 3 - What the office does.** The office may carry out certain actions depending on which band a room falls in.

| Band | Action |
|---|---|
| `WITHIN BUDGET` | no action |
| `SLIGHTLY OVER` | reminder emailed |
| `WELL OVER` | warning emailed, plus a surcharge |
| `FAR OVER` | inspection booked, suspected undeclared appliance, plus a surcharge |

**Clause 4 - Surcharge.** Excess units are charged at the standard rate of **27 cents a unit**.

A surcharge applies only from `WELL OVER` upwards. A room that is within budget, or only slightly over, pays nothing.

---

Four short functions, one clause each. One of them is a single line.

---

## Before You Start

1. Ensure you finish the one-time Rust setup in [course-overview](https://github.com/ORG-PLACEHOLDER/course-overview). Both `rustc --version` and `cargo --version` should print a version number.
2. Accept the assignment, clone your copy, and open the exercise folder in your code editor.
3. Run `cargo run`. It should print this, with no errors:

   ```
   BLOCK 59 - ELECTRICITY CHECK

     59-01-01 | TODO step 4
     59-01-02 | TODO step 4
     59-01-03 | TODO step 4
     59-01-04 | TODO step 4

    4 rooms | 0 over budget | $0.00 in surcharges
   ```

   That is correct. The program runs, but the four functions you are about to write are still placeholders, so the rows say `TODO step 4` instead of the real thing.

   **When you have finished, no `TODO` should appear anywhere in the output.** That is your completion check.

### The starter code

| Item | What it is |
|---|---|
| `cost_cents` | **Given.** Electricity is 27 cents a unit |
| `units_over` | Given. How many units past budget a room went, stopping at zero |
| `money` | Given. Turns 162 into `"$1.62"` |
| `band` | Step 1. Returns the overage band |
| `over_budget` | Step 2. Returns a placeholder |
| `action` | Step 3. Returns a placeholder |
| `room_line` | Step 4. Returns a placeholder |
| `main` | Given. Prints the block report |

`units_over` uses `saturating_sub` rather than `-`. Writing `used - budget` crashes when `used` is the smaller number, because a `u32` cannot go negative. `saturating_sub` returns zero instead. You write this yourself in Assignment 02.

Complete Steps 1 to 4 in order. Run `cargo run` after each step. Delete
`#![allow(unused_variables, dead_code)]` when all steps are complete.

---

## Step 1 - Band the overage

Classify the number of units over budget using the table below.

| Units over budget | Band |
|---|---|
| 0 | `"WITHIN BUDGET"` |
| 1 to 3 | `"SLIGHTLY OVER"` |
| 4 to 8 | `"WELL OVER"` |
| 9 and above | `"FAR OVER"` |

`match` is a control structure - the topic of this assignment. Like `if`, it decides which code runs, but instead of a yes/no condition it checks a value against a list of patterns and runs whichever arm fits.

Complete `band` by filling in the two missing match arms. Use inclusive ranges:
`1..=3` and `4..=8`.

```rust
fn band(units_over: u32) -> &'static str {
    match units_over {
        0 => "WITHIN BUDGET",
        // Add "SLIGHTLY OVER" and "WELL OVER" here.
        _ => "FAR OVER",
    }
}
```
## Step 2 - Is the room over budget at all?

Complete `over_budget`. Return `true` only when `used` is greater than
`budget`.

This one is a plain comparison, not a control structure. `used > budget` already evaluates to a `bool` on its own - return that expression directly, no `if` or `match` needed.

```rust
fn over_budget(used: u32, budget: u32) -> bool {
    // Here compare the two.
}
```

## Step 3 - Decide what the office does

Complete `action`. Return the matching action. Calculate surcharges with
`money(cost_cents(units_over))`.

| Band | What happens |
|---|---|
| `"WITHIN BUDGET"` | `"no action"` |
| `"SLIGHTLY OVER"` | `"reminder emailed"` |
| `"WELL OVER"` | `"warning emailed, $1.62 surcharge"` |
| `"FAR OVER"` | `"inspection booked, suspected undeclared appliance, $3.24 surcharge"` |

Like Step 1, this uses a `match` - but matching on the band text itself, not a number range.

For the `WELL OVER` and `FAR OVER` arms, you need to combine fixed text with a computed value: `money(cost_cents(units_over))` returns a `String` like `"$1.62"`, and you need to build `"warning emailed, $1.62 surcharge"` around it. Use `format!("warning emailed, {} surcharge", money(cost_cents(units_over)))` - `format!` builds a `String` by dropping each value into the matching `{}`, in order. You will see `format!` explained again, in more depth, in Step 4.

Complete `action`:

```rust
fn action(units_over: u32) -> String {
    match band(units_over) {
        "WITHIN BUDGET" => "no action".to_string(),
        // Add the other three.
    }
}
```

---

## Step 4 - Build the row

Complete `room_line` so it returns one report line per room:

```
59-01-03 | budget 5 | used 11 | WELL OVER | warning emailed, $1.62 surcharge
```

Use this order: room, budget, usage, band, action.

Use `format!` inside `room_line` to build and return the row. Use `println!`
inside `main` to print the returned row on the screen.

`format!` creates text. `println!` displays text.

```rust
fn room_line(room: &str, budget: u32, used: u32) -> String {
    // Here work out the units over budget with units_over.
    // Here build the line with format!, calling steps 1 and 3.
}
```

Calculate `units_over` once. Pass it to both `band` and `action`. Run:

```
cargo run
```

**Expected output:**

```
BLOCK 59 - ELECTRICITY CHECK

  59-01-01 | budget 5 | used 4 | WITHIN BUDGET | no action
  59-01-02 | budget 13 | used 15 | SLIGHTLY OVER | reminder emailed
  59-01-03 | budget 5 | used 11 | WELL OVER | warning emailed, $1.62 surcharge
  59-01-04 | budget 5 | used 17 | FAR OVER | inspection booked, suspected undeclared appliance, $3.24 surcharge

  4 rooms | 3 over budget | $4.86 in surcharges
```

---

## Step 5 - Personalize the report (optional)

Add a report date without changing the four graded functions.

1. Add a constant near the top of `main.rs`:

    ```rust
    const REPORT_DATE: &str = "2026-09-04";
    ```

2. Print it below the report heading:

    ```rust
    println!("Report date: {}\n", REPORT_DATE);
    ```

This step is optional and is not graded.

---

## Step 6 - Apply your first test

Rust has a built-in test framework. A test is just a normal function marked `#[test]`. Tests live in a block like this, usually at the bottom of `main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_over_band() {
        // Boundary: 9 is exactly where WELL OVER ends and FAR OVER begins.
        assert_eq!(band(9), "FAR OVER");
    }

    #[test]
    fn exactly_at_budget_is_not_over() {
        // Boundary: using exactly the budget does not count as over.
        assert_eq!(over_budget(5, 5), false);
    }

    #[test]
    fn well_over_names_the_real_surcharge() {
        assert_eq!(action(6), "warning emailed, $1.62 surcharge");
    }

    #[test]
    fn room_59_01_03_reports_correctly() {
        // The overall function: room_line calls units_over, band, and action together.
        assert_eq!(
            room_line("59-01-03", 5, 11),
            "59-01-03 | budget 5 | used 11 | WELL OVER | warning emailed, $1.62 surcharge"
        );
    }
}
```

`#[cfg(test)]` means "only compile this when running tests" - it never runs during a normal `cargo run`. `use super::*;` pulls in everything from outside the block so the tests can call your functions directly. `assert_eq!(actual, expected)` takes whatever your function actually returns and compares it against the answer the policy says it should be. If they don't match, the test fails and prints both values.

Notice each test checks a different one of your four functions, and each picks the input that actually matters for that function: `band` and `over_budget` get their trickiest boundary (where an off-by-one bug would show up), `action` confirms the surcharge is computed rather than hardcoded, and `room_line` - the function that calls the other three - confirms the whole chain agrees on one real room from Step 4's own example.

This exact block is already sitting at the bottom of your `main.rs`, commented out. Once Steps 1 to 4 are done, uncomment it and run:

```
cargo test
```

It should say `4 passed`. Starting in Assignment 02, you will write tests like these yourself, one per function you complete.

---

## What to Hand In

1. `src/main.rs` with Steps 1 to 4 complete and `cargo run` producing the report above.
2. Step 5 is optional.
3. Uncomment the Step 6 tests at the bottom of `main.rs` and confirm `cargo test` shows `4 passed`.
4. Commit and push, then submit.

```
git add src/main.rs
git commit -m "Assignment 01: electricity check"
git push
```

Commit as you finish each step, not all at once at the end. Small commits are better practice and they make it obvious you built it yourself.

---

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`band`) correctly implemented | 20 |
| Step 2 (`over_budget`) correctly implemented | 20 |
| Step 3 (`action`) correctly implemented | 20 |
| Step 4 (`room_line`) correctly implemented | 20 |
| `cargo test` runs, all Step 6 tests pass | 10 |
