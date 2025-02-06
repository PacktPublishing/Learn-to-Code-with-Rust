What is the default return value of a function if an explicit return value type is not declared?

An empty array
An empty string
! An empty tuple (unit)
The number 0

---

What is a parameter?

An empty tuple
What the function produces as its output
The set of parentheses that invokes a function
! A name for an expected input to a function

---

What will a function return if the return keyword is omitted?

The evaluation of the first executed line
! The evaluation of the last executed line
An empty array
A empty string

---

What will this function return?

```rust
fn shoe_size() -> i32 {
    return 12;
    3 + 5
}
```

! 12
8
0
An empty tuple

---

A pair of curly braces creates a ...

Segment
Sequence
Area
! Block

---

What is the alternate name for an empty tuple?

Curly
A blank
A nuple
! A unit

---

What is wrong with this code?

```rust
fn shoe_size() {
    12
}
```

There is nothing wrong with the code.

If there are no parameters, the function declaration should not include parentheses.

! The function does not declare the type of its return value.

Every value created inside a function body needs to be assigned to a variable.

The fn keyword should be replaced with fun.

---

What keyword declares a function?

fun
function
def
! fn

---

What is wrong with this code?

```rust
fn shoe_size() -> i32 {
    12;
}
```

There is nothing wrong with the code.

The `i32` return type does not support positive numbers like 12.

The function should omit the return value type.

! The semicolon creates a statement which prevents the implicit return of 12. Instead, the function returns a unit.

Every value created inside a function body needs to be assigned to a variable.

---

What is the `mystery` variable equal to?

```rust
fn main() {
    let mystery = {
        5 + 4;
    };
}

```

9
! A unit
9.0
Nothing; the code will raise an error.
