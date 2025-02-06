const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "Fall";
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!("My favorite season is {0}. The team scored {1} points. The event started at {2}. A touchdown is worth {3} points.", season, points_scored, event_time, TOUCHDOWN_POINTS);

    #[allow(unused_variables)]
    let favorite_beverage = "Snapple Apple";
}
