use intervalues;
use intervalues::BaseInterval;
use intervalues::IntFloat;
use num_traits::ToPrimitive;
use rand::Rng;
use rust_decimal::Decimal;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();

    println!(
        "Running the binary of >intervalues< will show a demo of what the library of it can do."
    );
    println!("What follows are variations of combining 1 000 000 intervals between 0 and 9:");

    println!(
        "\n(1) Valued and using i32 typed interval borders, value set to 1 for all. \
    Converts to BaseInterval and returns IntervalCollection"
    );
    let mut input = Vec::new();
    let n = 1000000;
    for _ in 0..n {
        input.push(BaseInterval::new(
            rng.gen_range(0..10),
            rng.gen_range(0..10),
            1,
        ));
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!(
        "\n(2) Valued and using i32 typed interval borders, value set to Decimal 1.5 for all. \
    Converts to BaseInterval and returns IntervalCollection"
    );
    let mut input = Vec::new();
    let n = 1000000;
    for _ in 0..n {
        input.push(BaseInterval::new(
            rng.gen_range(0..10),
            rng.gen_range(0..10),
            // 1),
            Decimal::from_f32_retain(1.5).unwrap(),
        ));
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!(
        "\n(3) Valued and using Decimal (via float) typed interval borders, value set to Decimal(1.5) for all. \
    Converts to BaseInterval and returns IntervalCollection"
    );
    let mut input = Vec::new();
    let n = 1000000;
    for _ in 0..n {
        input.push(BaseInterval::new(
            Decimal::from_f32_retain(0.5 + rng.gen_range(0..10).to_f32().unwrap()).unwrap(),
            Decimal::from_f32_retain(0.5 + rng.gen_range(0..10).to_f32().unwrap()).unwrap(),
            Decimal::from_f32_retain(1.5).unwrap(),
        ));
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!(
        "\n(4) Valued and using IntFloat typed interval borders, value set to IntFloat(1.5) for all. \
    Converts to BaseInterval and returns IntervalCollection"
    );
    let mut input = Vec::new();
    let n = 1000000;
    for _ in 0..n {
        input.push(BaseInterval::new(
            IntFloat::from(0.5 + rng.gen_range(0..10).to_f32().unwrap(), 1),
            IntFloat::from(0.5 + rng.gen_range(0..10).to_f32().unwrap(), 1),
            IntFloat::from(1.5, 1),
        ));
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);
}
