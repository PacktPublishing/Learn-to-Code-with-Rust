Where is data with a fixed size stored at runtime?

The heap
! The stack

---

What trait does a type implement that promises that a full duplicate can be made of the value?

Duplicate
Clone
Display
Debug
! Copy

---

What method concatenates text to the end of a String?

concat
insert
push_string
! push_str
add

---

Where is data with a dynamic or unpredictable size stored at runtime?

! The heap
The stack

---

What method creates a copy of a heap-allocated data type?

copy
! clone
dupe
duplicate

---

What operator creates a reference to a value?

`!`
! `&`
`$`
`#`

---

What operator follows a reference to the value at its memory address?

`&`
! `*`
`!`
`$`

---

**TRUE** or **FALSE**: When interpolating a reference in a `println!`, we must include the dereference operator to print the value at the memory address.

True
! False

---

Who is the owner of the `String` at the end of the function?

```rust
fn main() {
    let drink = String::from("Snapple");
    let beverage = drink;
    let delight = &beverage;
}
```

drink
! beverage
delight

---

**TRUE** or **FALSE**: Moving ownership from one variable to another transfers the original variable's mutability permissions.

True
! False
