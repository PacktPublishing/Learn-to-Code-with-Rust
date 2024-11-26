What character is a catch-all pattern in a match statement?

`@`
`!`
`?`
! `_`

---

Which keyword terminates a loop?

stop
terminate
! break
continue
restart

---

Which keyword forces a loop to start at the next iteration?

restart
begin
break
! continue
redo

---

Which statement is true?

You can assign the result of an if/else statement to a variable but not the result of a match statement.

You can assign the result of a match statement to a variable but not the result of an if/else statement.

! You can assign both the result of a match statement and an if/else statement to a variable.

You cannot assign neither the result of a match statement nor an if/else statement to a variable.

---

What is the wrong with the following code?

```rust
fn main() {
    let fruit = "apple";

    let calories = match fruit {
        "apple" => 150,
        "pear" => 200,
        "oranges" => 150,
        _ => "Unknown",
    };

    println!("{calories}");
}

```

There is nothing wrong with the code.

The match arms do not cover all possibilities of what the fruit variable could be.

The match arms should not use commas to separate the patterns.

The match statement result cannot be assigned to a variable.

! The match arms do not return a consistent type.

---

Will this code produce an infinite loop?

```rust
fn main() {
    let mut value = 10;

    loop {
        if value == 1 {
            println!("Terminating the loop!");
            break;
        }

        println!("The current value is {value}");
        value -= 2;
    }
}

```

! Yes
Sometimes
No

---

What is recursion?

When a function uses a macro from the Rust standard library
When a function uses a loop in its body
! When a function calls itself
When a function terminates early with a `return` keyword

---

TRUE or FALSE: The `else` keyword requires an `else if`.

True
! False

---

What symbols connect a `match` value to its corresponding block?

`==>`
`--`
`->`
! `=>`

---

What's the problem with the following code?

```rust
fn main() {
    let amount = 99;

    while amount > 0 {
        println!("The current value is {amount}");
        amount -= 3;
    }
}
```

There is no problem.
The code will run into an infinite loop.
The code is missing a `break` keyword.
! The `amount` variable needs to be mutable.
