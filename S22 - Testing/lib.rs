/*
We're running a salad restaurant! You discover some starter
code from a previous developer working at the company. The
code  includes:
- A Vegetable enum
- A Protein enum
- A Dressing enum

Our next goal is to build a Salad struct. Each Salad will
have a 'protein', 'vegetables', and a 'dressing' field. A
Salad can store 1 protein, any number of vegetables, and
1 dressing. Use a vector to store the vegetables. Derive
the Debug trait.

We need to implement the following 4 functions/methods on
a Salad. All 4 must have a complementary unit test. It's up
to you whether you want to write your tests first (TDD) or
write your implementation first. Follow the best practices
for unit tests (modules, configuration, etc). Feel free
to utilize any helper crates (pretty_assertions, rstest,
etc).

First, define a 'new' constructor function that accepts a
'protein', a 'vegetables' vector, and a 'dressing' and
returns an instance of the Salad. In the test, assert that
the 3 fields of the Salad are correctly populated.

Next, define an 'is_valid' method that returns a Boolean.
Return a true if a salad has more than 0 vegetables.

Next, define a 'calories' method that calculates the total
calories in the salad. The Vegetable, Protein, and Dressing
enums all support a 'calories' method that return the
calories of the item. Remember that 'vegetables' is a vector
of multiple Vegetable values -- you'll have to include all of
them in your calculation.

Finally, define a 'has_duplicate_vegetables' method. It
should determine if the salad includes any vegetable more
than once. Return a Boolean.
*/

trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}
