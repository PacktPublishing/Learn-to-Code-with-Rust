## src/main.rs

```rust
/*
For this challenge, we'll be modeling a fitness program.

Create a new Cargo project. Give your project a name of
'fitness'.

The project will include both a binary crate and a library
crate. Create the required files in the `src` directory.

We'll have 3 modules: `diet`, `weightlifting`, and `cardio`.
Declare the 3 modules in the library crate root.

Use an inline module definition for the `diet` module.
Inside the module:
- Declare a NUTRITIONIST constant set to "Norah Nutrition"

Use a file module definition for the `weightlifting` module.
Inside the module:
- Declare a PERSONAL_TRAINER constant set to "Will Weight"
- Declare an ask_about_program function that outputs the text
"The weightlifting trainer is Will Weight" (use the constant)
- Declare an Exercise struct with a name field (String) and
a reps field (u32). Derive a Debug implementation.
- Add a new constructor function to the Exercise struct
that returns an Exercise instance.

Use a folder module definition for the `cardio` module.
Inside the module:
- Declare a PERSONAL_TRAINER constant set to "Cardl Cardio"
- Declare an ask_about_program method that outputs the text
"The cardio trainer is Carl Cardio" (use the constant)
- Declare a CardioTool enum with 2 variants: Treadmill
and Bike. Derive a Debug implementation.
- Declare an Exercise struct with a day field (String),
a tool field (CardioTool), and a minutes field (u32).
Derive a Debug implementation.
- Add a new constructor function to the Exercise struct
that returns an Exercise instance.

In the library crate root,
- Use the `use` keyword to create shortcuts for CardioTool
- Assign the alias `CardioExercise` to cardio::Exercise
- Assign the alias `WeightliftingExercise` to weightlifting::Exercise
- Define a GymWorkout struct with a weightlifting field (an Exercise struct
from the weightlifting module) and a cardio module (an Exercise
struct from the cardio module).
- Define a `new` constructor function on the GymWorkout function
The function should invoke BOTH of the ask_about_program functions,
then return an instance of the GymWorkout struct. Pick arbitrary values
for the required struct fields.

Finally, in the binary crate root, invoke the GymWorkout::new
function and print out the GymWorkout struct in Debug format.
Here is what a sample output might look like:

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

fn main() {
    let workout = playground::GymWorkout::new();
    println!("{workout:#?}");
}

```

## src/lib.rs

```rust
mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";
}

mod cardio;
mod weightlifting;

use cardio::{CardioTool, Exercise as CardioExercise};
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    weightlifting: WeightliftingExercise,
    cardio: CardioExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        cardio::ask_about_program();
        weightlifting::ask_about_program();

        Self {
            weightlifting: WeightliftingExercise::new(String::from("Bench Press"), 8),
            cardio: CardioExercise::new(String::from("Thursday"), CardioTool::Bike, 30),
        }
    }
}

```

## src/weightlifting.rs

```rust
const PERSONAL_TRAINER: &str = "Will Weight";

pub fn ask_about_program() {
    println!("The weightlifting trainer is {PERSONAL_TRAINER}");
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
    reps: u32,
}

impl Exercise {
    pub fn new(name: String, reps: u32) -> Self {
        Self { name, reps }
    }
}

```

## src/cardio/mod.rs

```rust
const PERSONAL_TRAINER: &str = "Carl Cardio";

pub fn ask_about_program() {
    println!("The cardio trainer is {PERSONAL_TRAINER}");
}

#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Bike,
}

#[derive(Debug)]
pub struct Exercise {
    day: String,
    tool: CardioTool,
    minutes: u32,
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        Self { day, tool, minutes }
    }
}

```
