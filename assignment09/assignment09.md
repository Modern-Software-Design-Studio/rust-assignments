# Assignment 09 - The vending machine

## Topic: Open build | Language: **Rust** | Repo: `rust-09-vending-machine`

**AI Policy:** Do not use AI to generate or complete your solution code. The assignment is a carefully guided exercise that would help you practice Rust Programming Language. Using AI to execute your assignment puts you at disadvantage for the class project.

## What You Are Doing

You have paid into a vending machine before and it did not respond. No drink, no change, no message. Build the machine that would have told you why.

Twelve slots of drinks and snacks. Coins and notes go in, or you pay by card or scan. You select an item, you pay for it, and either it comes out or the machine tells you exactly why not.

This assignment is different from the earlier ones. You are building a larger program, but you are not starting from nothing.

Use the starter structure in `src/main.rs`. It gives you the main pieces: `Slot`, `Machine`, `Machine::new`, `Machine::handle`, and one helper function per command.

## Before You Start

1. Open this folder in your code editor.
2. Run `cargo run`. Nothing prints yet, because no commands were typed. That is correct.

The starter code already contains `SLOT_DATA`, `VALID_DENOMINATIONS`, `money`, and a starter structure. Do not change the given constants or `money`.

## The slots

| Code | Item | Price | Starting stock |
|---|---|---:|---:|
| A1 | Coke | $1.20 | 5 |
| A2 | Sprite | $1.20 | 0 |
| A3 | Water | $0.80 | 8 |
| A4 | Iced Milo | $1.50 | 3 |
| B1 | Chips | $1.30 | 6 |
| B2 | Pretzels | $1.30 | 4 |
| B3 | Chocolate | $1.80 | 2 |
| B4 | Chips (Spicy) | $1.30 | 1 |
| C1 | Gum | $0.50 | 10 |
| C2 | Mints | $0.50 | 9 |
| C3 | Cough Drops | $0.90 | 7 |
| C4 | Tissues | $1.00 | 3 |

## The commands

```text
select <code>       pick a slot
insert <cents>       insert a coin or note
pay <method>          pay by "card" or "scan"
balance               show the current balance
refund                return the balance, clear the selection
list                  print every slot, price, and stock
```

One command per line, read from standard input.

```
cargo run < script.txt
```

## The rules, stated exactly

**Selecting a slot the first time** names the price and does not charge anything.

```
> select A1
Coke is $1.20. Insert cash or pay by card or scan.
```

**Selecting the same slot again** is how a cash purchase confirms. If the balance covers the price, it dispenses. If not, it tells you the shortfall.

```
> select A1
Coke is $1.20. Insert cash or pay by card or scan.
> insert 100
Balance: $1.00
> select A1
Coke is $1.20. You have inserted $1.00. Insert 20 more cents.
> insert 20
Balance: $1.20
> select A1
Dispensed: Coke.
```

**Selecting a different slot** switches the selection. It does **not** dispense, even if the balance already covers the new item. Only re-selecting the same code dispenses.

```
> select C1
Gum is $0.50. Insert cash or pay by card or scan.
> insert 50
Balance: $0.50
> select C2
Mints is $0.50. Insert cash or pay by card or scan.
```

Balance carries over when you switch. Nothing is lost.

**Change is only mentioned when change is owed.** Exact payment gets a plain `Dispensed:` line.

```
Dispensed: Water. Change: 20 cents.
Dispensed: Gum.
```

**A slot that does not exist:**

```
> select Z9
No slot Z9.
```

**A slot with no stock:**

```
> select A2
Sprite is sold out.
```

**Cash only accepts real denominations**, in cents: 5, 10, 20, 50, 100, 200, 500, 1000. Anything else is rejected and the balance is untouched.

```
> insert 3
3 cents is not a valid note or coin.
```

**Paying by card or scan requires a selection first.** Card and scan never touch the running cash balance, and neither one owes change.

```
> select B3
Chocolate is $1.80. Insert cash or pay by card or scan.
> pay card
Card declined.
> pay card
Paid $1.80 by card. Dispensed: Chocolate.
```

**Card payments alternate.** The first card attempt made in the whole run is declined. The second succeeds. The third is declined. And so on, counted across the entire session, not per slot.

**Scan always succeeds.**

```
> select B3
Chocolate is $1.80. Insert cash or pay by card or scan.
> pay scan
Paid $1.80 by PayNow. Dispensed: Chocolate.
```

**Refund** returns whatever cash balance is sitting there and clears the selection.

```
> select A1
Coke is $1.20. Insert cash or pay by card or scan.
> insert 50
Balance: $0.50
> refund
Refunded $0.50.
```

**`balance`** on its own prints the current balance. **`list`** prints all twelve slots, in the order given above, with their live stock.

**A command that is not one of the six** gets echoed back:

```
> dance
Unknown command: dance
```

**Blank lines in the input are skipped**, producing no output.

## How to approach this

You are writing the whole program, not filling in blanks. Use this structure. Plan on paper before you type.

**1. Read the rules above until you can narrate a purchase out loud.**
`select`, then pay, then `select` again to confirm a cash purchase. Nothing dispenses until that second `select`, or until `pay` succeeds.

**2. Create a `Slot` struct.**
Name, price, and stock belong together. Stock changes as items sell.

```rust
struct Slot {
    name: &'static str,
    price_cents: u32,
    stock: u32,
}
```

**3. Create a `Machine` struct.**
This keeps the machine state between commands.

```rust
struct Machine {
    slots: HashMap<String, Slot>,
    order: Vec<String>,
    balance: u32,
    selected: Option<String>,
    card_attempts: u32,
}
```

**4. Use a `HashMap<String, Slot>` for the slots.**
Key it by the slot code, like `"A1"`. This lets each command find a slot directly.

**5. Keep a `Vec<String>` of the codes in order.**
Use this for `list`. A `HashMap` does not keep print order.

**6. Write `Machine::new`.**
Read `SLOT_DATA`. Fill `slots`. Fill `order`. Set `balance` to `0`. Set `selected` to `None`. Set `card_attempts` to `0`.

**7. Write `Machine::handle`.**
This function reads one command line and sends it to the correct helper.

```rust
fn handle(&mut self, line: &str) -> String {
    // Split the line.
    // Match the command.
    // Call the correct helper.
}
```

Split the line on the first space. The first word is the command. Everything after it is the argument.

**8. Write one helper function per command.**
Write helpers for `select`, `insert`, `pay`, `balance`, `refund`, and `list`. Each helper returns the line to print. Do not print inside the helpers.

**9. Write the sale logic once.**
A cash sale and a card or scan sale both reduce stock by one. They also clear the balance and selection. Put that shared work in one helper.

**10. Read commands in `main`.**
Use `std::io::stdin().lock().lines()`. Skip blank lines. Stop when there are no more lines.

**11. Keep `main` short.**
`main` should read, call `machine.handle(...)`, and print. Move pricing logic and command logic into helper functions.

**12. Test your own scripts before trusting it.**
Test exact change. Test change owed. Test a declined card followed by an approved one. Test a sold out slot. Test an unknown slot. Test a bad coin. Test switching selection with money already in the balance.

**13. Delete `#![allow(unused_variables, dead_code)]` and run `cargo build`.**
Fix every compiler error before you submit.
## What to Hand In

1. `src/main.rs`, fully working, matching the rules above.
2. Keep `SLOT_DATA`, `VALID_DENOMINATIONS`, and `money` unchanged.
3. Remove `#![allow(unused_variables, dead_code)]`.
4. Run `cargo check`, then test your program against a few scripts of your own.
5. Commit and push your work.

## Grading Rubric

| Criteria | Points |
|---|---:|
| `cargo build` runs with no errors | 10 |
| Cash purchases: exact change, change owed, insufficient balance | 20 |
| Sold out, unknown slot, bad coin, unknown command all match exactly | 20 |
| Card alternates declined and approved correctly across the whole run | 15 |
| Scan always succeeds, never asks for cash | 10 |
| Switching selection never auto-dispenses | 10 |
| Refund and `list` match exactly | 10 |
| Code is organised into functions, not one long `main` | 5 |

Grading runs several scripts through standard input and compares the output exactly, the same way you would test it yourself.
