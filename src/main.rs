use std::time::Instant;
use intervalues;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Running the binary of >intervalues< will show a demo of what the library of it can do.");
    println!("What follows are 4 variations of combining 1 000 000 intervals between 0 and 9:");

    println!("\n(1) Unvalued and using <isize> typed interval borders. Returns HashMap<(lb,ub),val>.");
    let mut input: Vec<[isize; 2]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10)])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!("\n(2) Valued and using <isize> typed interval borders, value set to 1 for all. \
    Returns HashMap<(lb,ub),val>.");
    let mut input: Vec<[isize; 3]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10), 1])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_values(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!("\n(3) Unvalued and using <f64> typed interval borders. Returns Vec<(lb,ub,val)>");
    let mut input: Vec<[f64; 2]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10) as f64, rng.gen_range(0..10) as f64])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_flt(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    println!("\n(4) Valued and using <f64> typed interval borders, value set to 1.0 for all. \
    Returns Vec<(lb,ub,val)>");
    let mut input: Vec<[f64; 3]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10) as f64, rng.gen_range(0..10) as f64, 1.0])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_flt_values(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

}
