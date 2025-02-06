Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
NOTE: This exercise will (a) likely take some time
and (b) produces a lot of output. I've added the
#![allow(unused, dead_code)] directive at the top
of the file to silence some compiler warnings.
Feel free to comment out certain lines/solutions
to reduce output.

---

Let's imagine we're running an e-commerce store that
sells home appliances. Another developer has left
some starter code to work with.

The Product enum has 4 variants for the products
we sell: blender, microwave, toaster, and fridge.

The CustomerOrder struct represents an online order.
It stores the ordered Product, its quantity, and
whether we've shipped it to the customer. There is
a complementary 'new' constructor in the 'impl'
block.

The Customer struct represents a customer. Each
customer has a unique numeric ID and a vector of
their orders.

---

In `main`, we have an `orders` vector with the 6
orders in our system.

We also have a `customer_ids_by_order` array that
lists the customer ID of each customer who placed
each of the 6 orders.

Our boss needs help figuring out critical
business numbers! Help him!

----

Extract all the customer orders where the customer
ordered a Blender. Our goal is a vector of
&CustomerOrder values. Print out the vector.
It should have 2 total orders.

HINT: Pretty-printing the output will make it
easier to parse.

---

The boss would like to know the total quantity of
microwaves ordered across all customer orders. Filter
for the customer orders where the Product is a
Microwave, extract the 'quantity' field for each
customer order, then calculate the sum of those
values. The answer should be 6.

BONUS: Solve the challenge with both (a) filter + map
and (b) filter_map

---

The boss would like to pass in a quantity from the
command line. They want to see a printed vector of
all orders where the quantity is greater than or
equal to their input.

For example,
'cargo run -- 5'

should print a vector of the two customer orders
with a quantity greater than or equal to 5:

[
CustomerOrder::new(Product::Microwave, 5, true),
CustomerOrder::new(Product::Fridge, 10, false),
]

If the boss does not provide a command-line argument
OR provides an invalid numeric value, fallback to
printing customer orders with a quantity greater
than or equal to 2.

---

The boss would like to know how much inventory
of each product we need for unshipped orders.

Create a HashMap where each key will be a &Product
and each value will be the sum of that products's
quantities across unshipped orders. Make sure to
target only unshipped orders.

Print out the HashMap. It should be:
{Fridge: 10, Toaster: 2, Blender: 4}

---

Our warehouse worker informs us they've shipped
the next unshipped order. Find the first
unshipped order among the customer orders and
change its `shipped` field to `true`. Print out
the customer orders to confirm.

---

THIS IS A TOUGH ONE.

The boss would like to see a vector of Customer
structs. Each Customer struct will hold the user's
id and a vector of their orders. Find a way to merge
the customer orders with the customers who made them,
then aggregate the data into Customer structs,
then collect the Customers in a vector, then
sort the collection by customer id.

The resulting vector should look like this:

[

Customer {
  id: 1,
  orders: [
    CustomerOrder { product: Microwave, quantity: 1, shipped: true },
    CustomerOrder { product: Fridge, quantity: 10, shipped: false }
  ]
},

Customer {
  id: 2,
  orders: [
   CustomerOrder { product: Blender, quantity: 3, shipped: false },
   CustomerOrder { product: Toaster, quantity: 2, shipped: false }
  ]
},

Customer {
  id: 3,
  orders: [
   CustomerOrder { product: Microwave, quantity: 5, shipped: true }
  ]
},

Customer {
  id: 4,
  orders: [
    CustomerOrder { product: Blender, quantity: 1, shipped: false }
  ]
}

]
*/

#![allow(unused, dead_code)]

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];
}
```