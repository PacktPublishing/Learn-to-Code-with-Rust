What is a generic?

When two functions return a value of the same type.
A variable declaration without a concrete type declared.
A cheap cereal in the supermarket that is a knockoff of a major brand.
! A type argument.

---

What symbols create room for a generic type declaration?

`[]`
`()`
`~~
! `<>`

---

What is the common name for a single generic type?

Type
G
GT
! T

---

What are the symbols for a turbofish operator?

`::(type)`
`@type@`
! `::<type>`
`::<type>::`

---

Say we have a struct with a generic and we want to define a method that will be on all instances regardless of type `T`.

Why does Rust require the syntax `impl<T> MyStruct<T> {}`. Why isn't `impl MyStruct<T>` sufficient?

It is sufficient. The syntax will work but we have to include `::` before the `<T>`.

`T` is an invalid name for a generic type when defining an `impl` block.

! Rust wants to avoid ambiguity with a concrete type named `T` in the program.

Either syntax option will work.

---
