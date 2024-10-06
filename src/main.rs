use intervalues;
use intervalues::BaseInterval;
use itertools::Itertools;
use number_general::Number;
use rand::Rng;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();

    println!(
        "Running the binary of >intervalues< will show a demo of what the library of it can do."
    );
    println!("What follows are variations of combining 1 000 000 intervals between 0 and 9:");

    println!(
        "\n(1) Unvalued and using <isize> typed interval borders. Returns HashMap<(lb,ub),val>."
    );
    let mut input: Vec<[isize; 2]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10)])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_isize_no_val(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!(
        "\n(2) Valued and using <isize> typed interval borders, value set to 1 for all. \
    Returns HashMap<(lb,ub),val>."
    );
    let mut input: Vec<[isize; 3]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10), 1])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_isize(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    // println!("\n(3) Unvalued and using <f64> typed interval borders. Returns Vec<(lb,ub,val)>");
    // let mut input: Vec<[f64; 2]> = Vec::new();
    // for _ in 0..1000000 {
    //     input.push([rng.gen_range(0..10) as f64, rng.gen_range(0..10) as f64])
    // }
    // let before = Instant::now();
    // let hi = intervalues::combine_intervals_flt(input);
    // let after = Instant::now();
    // println!("{:?} in {:?}", hi, after - before);
    //
    // println!("\n(4) Valued and using <f64> typed interval borders, value set to 1.0 for all. \
    // Returns Vec<(lb,ub,val)>");
    // let mut input: Vec<[f64; 3]> = Vec::new();
    // for _ in 0..1000000 {
    //     input.push([rng.gen_range(0..10) as f64, rng.gen_range(0..10) as f64, 1.0])
    // }
    // let before = Instant::now();
    // let hi = intervalues::combine_intervals_flt_values(input);
    // let after = Instant::now();
    // println!("{:?} in {:?}", hi, after - before);
    //
    // println!("\n(5) Valued and using Number typed interval borders, value set to 1.0 for all. \
    // Returns BaseInterval(lb,ub,val)]");
    // let mut input: Vec<[Number; 3]> = Vec::new();
    // for _ in 0..1000000 {
    //     input.push([Number::from(rng.gen_range(0..10)),
    //         Number::from(rng.gen_range(0..10)),
    //         Number::from(1)])
    // }
    // let before = Instant::now();
    // let hi = intervalues::combine_intervals_general(input);
    // let after = Instant::now();
    // println!("{:?} in {:?}", hi, after - before);

    println!(
        "\n(6) Valued and using Number typed interval borders, value set to 1.0 for all. \
    Converts to BaseInterval and returns IntervalCollection"
    );
    let mut input: Vec<BaseInterval> = Vec::new();
    for _ in 0..1000000 {
        input.push(BaseInterval::new(
            Number::from(rng.gen_range(0..10)),
            Number::from(rng.gen_range(0..10)),
            Number::from(1),
        ));
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!(
        "{:?} in {:?}",
        hi.to_vec().iter().map(|x| x.to_array()).collect_vec(),
        after - before
    );
}
