```rust
/*
Let's model a card deck!

Begin by defining a `Suit` enum with the 4 possible
suits: Clubs, Spades, Hearts, and Diamonds. Derive
the Copy, Clone, and Debug traits.

Define a `Rank` enum with the following ranks:
Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
Jack, Queen, King, Ace, and Joker. Derive the Copy,
Clone, and Debug traits.

Define a `Card` struct with a `rank` field set to a
Rank and a `suit` field set to an Option<Suit>. The
reason we'll use Option here is because the Joker rank
does not technically have a suit. Its `suit` will be
the None variant. Derive the Debug trait.

Define a `Deck` struct with a single `cards` field set
to a vector of `Card` structs. Derive the Debug trait.

Define a `new` constructor function on the `Deck`. It
should iterate over the 4 possible suits and the
13 main ranks (exclude Joker), create 52 Card instances,
populate them into a  vector, and then instantiate and
return a `Deck` instance with the cards.

Define a `shuffle` method on `Deck` that randomizes the
order of the cards. In `main`, instantiate the `Deck`,
call the method, and confirm the card order is
randomized.

Define a `insert_jokers` method on `Deck`. It should
insert 2 Joker cards into the deck at two random index
positions. As a reminder, a Joker will have a None suit.
Invoke the method in `main` and confirm you see the
Jokers added.

Define a `delete_random_card` method on `Deck`. The
deletion will be based on probability. The method should
have a 65% chance of deleting a random card from the
`Deck`.  Target the deleted card by a random index
position.

In `main`, invoke the `delete_random_card` method
10 times (HINT: Don't write the method out 10 times.
Find a more creative way to repeat the action). Print
out the length of `cards` vector after the operation.
We should expect to see the length decrease.
*/
```
