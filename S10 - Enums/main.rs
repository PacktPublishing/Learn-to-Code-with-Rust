#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            }
            Subscription::Basic(price, month) => {
                println!("You have limited access to the site's premium features for {price} for {month} months");
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                );
            }
        }
    }
}

fn main() {
    Subscription::Free.summarize();

    let basic = Subscription::Basic(4.99, 3);
    basic.summarize();

    let premium = Subscription::Premium {
        tier: Tier::Platinum,
    };
    premium.summarize();
}
