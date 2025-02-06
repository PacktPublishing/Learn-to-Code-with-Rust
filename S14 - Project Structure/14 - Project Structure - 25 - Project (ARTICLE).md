Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
For this challenge, we'll be modeling a fitness program.

Create a new Cargo project. Give your project a name of
`fitness`.

The project will include both a binary crate and a library
crate. Create the required files in the `src` directory.

We'll have 3 modules: `diet`, `weightlifting`, and `cardio`.
Declare the 3 modules in the library crate root.

Use an inline module definition for the `diet` module.
Inside the `diet` module:
- Declare a NUTRITIONIST constant set to "Norah Nutrition"

- Declare an `ask_about_program` function that outputs the text
  "The nutritionist is Norah Nutrition" (use the constant).

Use a file module definition for the `weightlifting` module.
Inside the `weightlifting` module:
- Declare a PERSONAL_TRAINER constant set to "Will Weight"

- Declare an `ask_about_program` function that outputs the text
  "The weightlifting trainer is Will Weight" (use the constant)

- Declare an Exercise struct with a `name` field (String) and
  a `reps` field (u32). Derive a Debug implementation.

- Add a `new` constructor function to the Exercise struct
  that returns an Exercise instance.

Use a folder module definition for the `cardio` module.
Inside the `cardio` module:
- Declare a PERSONAL_TRAINER constant set to "Carl Cardio"

- Declare an `ask_about_program` function that outputs the text
  "The cardio trainer is Carl Cardio" (use the constant)

- Declare a CardioTool enum with 2 variants: Treadmill
  and Bike. Derive a Debug implementation.

- Declare an Exercise struct with a `day` field (String),
  a `tool` field (CardioTool), and a `minutes` field (u32).
  Derive a Debug implementation.

- Add a `new` constructor function to the Exercise struct
  that returns an Exercise instance.

In the library crate root:
- Use the `use` keyword to bring the CardioTool enum into
  scope

- Assign the alias `CardioExercise` to cardio::Exercise

- Assign the alias `WeightliftingExercise` to
  weightlifting::Exercise

- Define a GymWorkout struct with a `cardio` field (an
  Exercise struct from the `cardio` module) and a `weightlifting`
  field (an Exercise struct from the `weightlifting` module).
  Implement the Debug trait.

- Define a `new` constructor function on the GymWorkout struct.
  The function should invoke ALL 3 of the `ask_about_program`
  functions, then return an instance of the GymWorkout struct.
  Pick arbitrary values for the required struct fields.

Finally, in the binary crate root:
- Invoke the `GymWorkout::new` function and print out the
  GymWorkout struct in Debug format.

---

Here is what a sample output might look like:

The nutritionist is Norah Nutrition
The cardio trainer is Carl Cardio
The weightlifting trainer is Will Weight
GymWorkout {
    weightlifting: Exercise {
        name: "Bench Press",
        reps: 8,
    },
    cardio: Exercise {
        day: "Thursday",
        tool: Bike,
        minutes: 30,
    },
}
*/

```
