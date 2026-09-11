# Assignment 10 - Test the vending machine

## Topic: Testing | Language: **Rust** | Repo: `rust-10-testing`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing
Imagine someone from outside of the class hands you vending machine code. Your
job is to find out if it actually works.

You are not changing what it does. You are just proving it works, by writing tests against it.

`cargo test` must run and pass against the file exactly as given. If your tests fail against correct code, the tests are wrong, not the code. That is what testing actually is: not writing the program, but writing the thing that catches it the moment it stops working.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run < some_script.txt` if you want to see it behave. It is the same machine from Assignment 09, complete.
3. Run `cargo test`. It should say `0 tests`. That is correct. You have not written any yet.

Everything above the `// ---- Your task ----` marker in `src/main.rs` is finished. Do not touch it.

## Why you are testing someone else's code

Because it decouples the two assignments. If your own Assignment 09 build had a bug in it, testing your own broken code here would teach you nothing and cost you twice. Everyone in this class is testing the identical, correct machine, which means everyone's tests can be checked the same way: does the suite catch a real mistake, or does it wave one through.

## How to approach this

**1. Read the whole file before writing a single test.**
Every function, every branch. You are not implementing this assignment. You are understanding someone else's code well enough to interrogate it.

**2. List every behaviour the machine has, straight from reading it, not from memory.**
Confirm each of these is really in there:

- selecting a slot for the first time names the price
- selecting the same slot again dispenses if the balance covers it
- selecting the same slot again with insufficient balance names the shortfall
- selecting a different slot switches without dispensing, even if the balance already covers it
- exact cash payment gives a plain `Dispensed:` line
- overpaying gives a `Dispensed:` line with the change stated
- a slot that does not exist is rejected
- a slot with no stock is rejected
- an invalid coin or note is rejected and the balance is untouched
- paying by card can be declined
- paying by card can succeed
- paying by scan always succeeds
- a successful sale reduces that slot's stock by one
- refund returns the balance and resets it to zero
- an unrecognised command is echoed back

**3. For each behaviour, find the exact method responsible.**
You are calling `Machine` methods directly in your tests, not going through `main` or standard input. Read `Machine::new`, `handle`, and the `do_` functions it calls.

**4. Write one `#[test]` per behaviour.**
Build a `Machine`, drive it through `handle`, assert on the string that comes back.

```rust
#[test]
fn selecting_names_the_price() {
    let mut machine = Machine::new();
    let result = machine.handle("select A1");
    assert_eq!(result, "Coke is $1.20. Insert cash or pay by card or scan.");
}
```


Add more tests using the same pattern:

```rust
#[test]
fn exact_cash_dispenses_without_change() {
    let mut machine = Machine::new();
    assert_eq!(machine.handle("select A3"), "Water is $0.80. Insert cash or pay by card or scan.");
    assert_eq!(machine.handle("insert 50"), "Balance: $0.50");
    assert_eq!(machine.handle("insert 20"), "Balance: $0.70");
    assert_eq!(machine.handle("insert 10"), "Balance: $0.80");
    assert_eq!(machine.handle("select A3"), "Dispensed: Water.");
}

#[test]
fn bad_coin_does_not_change_balance() {
    let mut machine = Machine::new();
    assert_eq!(machine.handle("insert 3"), "3 cents is not a valid note or coin.");
    assert_eq!(machine.handle("balance"), "Balance: $0.00");
}

#[test]
fn switching_selection_does_not_dispense() {
    let mut machine = Machine::new();
    assert_eq!(machine.handle("select C1"), "Gum is $0.50. Insert cash or pay by card or scan.");
    assert_eq!(machine.handle("insert 50"), "Balance: $0.50");
    assert_eq!(machine.handle("select C2"), "Mints is $0.50. Insert cash or pay by card or scan.");
}
```
**5. Test the boundary, not just the middle.**
Exact change and change owed are two different code paths inside the same function. If your tests only ever use round numbers that happen to work either way, you have not really separated them.

**6. Test that state carries across calls.**
`select`, then `insert`, then `select` again. A test that only ever calls one method in isolation cannot see whether the balance is actually remembered between commands.

**7. Test the one rule that is easy to miss.**
Switching to a different slot must never dispense, even when the balance already covers the new price. Write a test that switches selection with money already sitting in the balance, and confirms nothing was dispensed.

**8. Confirm your suite would catch a mistake.**
Do not change the given code. Your grader will check broken copies of the
machine. Your job is to write tests that pass on the correct machine and fail
on broken machines.

**9. `cargo test` passes, with at least fifteen tests, one per behaviour in your list.**

## What to Hand In

1. `src/main.rs` with your `#[cfg(test)] mod tests` block added at the bottom.
2. Everything above the `// ---- Your task ----` marker left exactly as given.
3. `cargo test` passes.
4. Commit and push your work.

## Grading Rubric

| Criteria | Points |
|---|---:|
| `cargo test` runs and passes as submitted | 10 |
| At least fifteen tests are present | 15 |
| Tests call `Machine` methods directly, not `main` | 10 |
| The suite catches each of six seeded mistakes when run against a broken copy of this machine | 60 |
| A broken copy caught | 10 each |

The sixty points are earned by mutation, not by counting tests. Several broken copies of this exact machine exist, each with one behaviour quietly wrong. Your test suite runs against each one. A suite that passes against every version, correct or broken, earns nothing on this line no matter how many `#[test]` functions it contains. A suite that fails on a broken copy has proven it was actually checking something.
