# Assignment 06 - Someone Already Has My Locker

## Topic: Error handling | Language: **Rust** | Repo: `rust-06-error-handling`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

`REQUESTS` is a record of this term's locker requests, in the order they actually came in. A real system checks each one the moment it arrives: is this a real locker, is the request complete, has somebody earlier in the queue already taken it. Your program runs those same checks, one request at a time, in that same order, against everything already confirmed so far.

A locker that does not exist gets rejected. A blank field gets rejected. If two requests do want the same locker, whoever comes first keeps it, and the other is rejected. Every one of those rejections is a `Result`: `Err(BookingError)`, carrying the exact reason, instead of a crash or a guess.

That error handling looks different depending on where it happens. Steps 1 and 2 use `?`, and it fits there: parse one thing, and it either works or it does not. Step 3 is different. `?` only works inside a function that itself returns a `Result`, and `process` returns two lists instead. Try to use `?` in that loop and the compiler refuses to build your code. You have to catch each line's `Result` yourself and decide, right there, whether it goes into the confirmed list or the rejected one.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. The program should compile and show `0 confirmed` and `0 rejected`.

The starter code already contains `Booking`, `BookingError`, `LOCKERS`, `MAX_MONTHS`, `locker_exists`, and `REQUESTS`. Do not change them.

## `Result` in this exercise

A successful operation returns `Ok(value)`. A failed operation returns `Err(error)`.

```rust
Result<Booking, BookingError>
```

`BookingError` identifies why a request failed. Handle each error and continue reading the remaining lines.

## Step 1 - Read the months field

Complete `parse_months`.

```rust
fn parse_months(field: &str) -> Result<u32, BookingError> {
    // Parse the field and check MAX_MONTHS.
}
```

Return:

- `Ok(months)` for a valid number from 0 to `MAX_MONTHS`.
- `Err(BadMonths(text))` when the field is not a number.
- `Err(TooLong(months))` when the number is above `MAX_MONTHS`.

Use `.parse()`, `.map_err(...)`, and `?`.

`.parse()` tries to turn text into a number, and gives back its own kind of `Result`. `.map_err(...)` swaps that error for a `BookingError`, so the rest of your code only ever has to deal with one error type. `?` then means: if what came back was `Err`, stop right here and hand that error back out of `parse_months`. If it was `Ok`, unwrap it and keep going with the plain number.

Run `cargo run`.

Small checks:

| Call | Result |
|---|---|
| `parse_months("6")` | `Ok(6)` |
| `parse_months("twelve")` | `Err(BadMonths("twelve"))` |
| `parse_months("24")` | `Err(TooLong(24))` |

## Step 2 - Read one request

Complete `parse_booking`.

The line format is:

```text
locker, student, months
```

Check these errors in this order:

1. Empty or spaces only: `Empty`.
2. Not exactly three fields: `WrongFieldCount(n)`.
3. Any field left blank after trimming: `Empty`.
4. Locker not in `LOCKERS`: `UnknownLocker(text)`.
5. Invalid months: return the error from `parse_months`.

Trim spaces around each field. Use `locker_exists`.

Notice how this step actually mixes two styles: the first four checks use `if` with an explicit `return Err(...)`, since there's no existing `Result` to unwrap yet. Only the last check, delegating to `parse_months`, gets to use `?`.

Return a `Booking` only when all fields are valid.

Small checks:

| Line | Result |
|---|---|
| `L-102, 1004512, 6` | `Ok(Booking { ... })` |
| `L-999, 1004512, 6` | `Err(UnknownLocker("L-999"))` |
| `L-102, 1004512` | `Err(WrongFieldCount(2))` |
| `L-102, , 6` | `Err(Empty)` |
| `L-102, 1004512,` | `Err(Empty)` |

Run `cargo run`.

## Step 3 - Process the whole sheet

Complete `process`:

```rust
fn process(text: &str) -> (Vec<Booking>, Vec<(usize, BookingError)>)
```

Rules:

- Skip blank lines.
- Count line numbers from 1, including blank lines.
- Keep valid bookings in order.
- Record every rejected line and its error.
- Continue after an error.
- Reject a locker that is already confirmed with `Taken(student)`.
- Reject a student who already has a locker with `AlreadyHasOne(locker)`.
- If a request would trigger both, check the locker conflict first.

Use `enumerate()` and add 1 to its index. Check conflicts against confirmed bookings before adding a new booking.

`if let Some(x) = value { ... }` is pattern matching too, like Assignment 03's `match`, but for checking one pattern at a time instead of listing every variant - useful here since you're checking one `Option` at a time (is this locker already held? is this student already booked?), not exhausting every case in one `match`.

Do not use `unwrap()` to parse a request. One bad line must not stop the sheet.

Small checks:

| Rule | Example |
|---|---|
| Blank lines are skipped | line 3 is blank, but line 4 is still line 4 |
| Duplicate locker | second `L-102` becomes `Taken("1004512")` |
| Same student twice | second booking becomes `AlreadyHasOne("L-102")` |

Run `cargo run`.

## Step 4 - Check the report

Expected completed output:

```text
LOCKER BOOKINGS

  3 confirmed
    L-102   1004512   6 months
    L-118   1004988   12 months
    L-107   1005644   3 months

  6 rejected
    line 4: Taken("1004512")
    line 5: UnknownLocker("L-999")
    line 6: AlreadyHasOne("L-102")
    line 7: BadMonths("twelve")
    line 8: TooLong(24)
    line 9: WrongFieldCount(2)
```

The blank line in `REQUESTS` is skipped but still counts as line 3.

## Step 5 - Add your own request (optional)

Do this only after Step 4 matches exactly. This step is not graded.

`REQUESTS` is the block of text near the top of `src/main.rs`. Add one line for yourself, written the way you would actually fill in a locker sign-up: your own locker code, a student number, and however many months you want.

```rust
const REQUESTS: &str = "\
L-102, 1004512, 6
L-118, 1004988, 12
L-120, 5551234, 6

L-102, 1005231, 6
...
```

Add your line **between** two existing ones, not after the last one. The last line is followed directly by `";`, with no line break in between, and moving that closing part is an easy way to break the string by accident.

Run `cargo run` again. If your line is valid, it should show up in the confirmed list. If you deliberately get something wrong, an unreal locker code or too many months, it should show up rejected, with the right reason.

The output will no longer match Step 4's block exactly, since your line makes it longer. That is expected. This step exists so you can watch your own code handle a request nobody wrote for you.

## Step 6 - Write a test

Add a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of `main.rs` - the same shape you read in Assignment 01's Step 6.

Write one `#[test]` for each of the three functions you just completed. For each one, pick your own input and assert what it should return.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_months_matches_my_input() {
        // Pick a field value (valid or invalid) and assert what parse_months should return.
    }

    #[test]
    fn parse_booking_matches_my_input() {
        // Pick a line (valid or invalid) and assert what parse_booking should return.
    }

    #[test]
    fn process_matches_my_sheet() {
        // Pick a small block of request text and assert what process should return.
    }
}
```

If you want a suggestion: a number above `MAX_MONTHS`, like `"24"`, is the trickiest case for `parse_months`, since it is a valid number but still a rejected value - but any input you can correctly justify is fine for any of the three.

Run `cargo test`. All three should pass.

## What to Hand In

1. Complete Steps 1 to 3 in `src/main.rs`.
2. Keep the `Result` signatures unchanged.
3. Add the tests from Step 6 and confirm `cargo test` passes.
4. Remove `#![allow(unused_variables, dead_code)]`.
5. Run `cargo check`, then run `cargo run`.
6. Commit and push your work.

## Grading Rubric (100 points)

| Criteria | Points |
|---|---|
| `cargo build` runs with no errors | 10 |
| Step 1 (`parse_months`) correctly implemented | 20 |
| Step 2 (`parse_booking`) correctly implemented | 20 |
| Step 3 (`process`) correctly implemented | 20 |
| Step 4, full report matches exactly and errors do not stop processing | 20 |
| `cargo test` runs, all Step 6 tests pass | 10 |
