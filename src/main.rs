use std::time::Instant;
use intervalues;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();


    let mut input: Vec<[isize; 3]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10), 1])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_values(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);


    let mut input: Vec<[isize; 2]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10)])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    let mut input: Vec<[f64; 2]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10) as f64, rng.gen_range(0..10) as f64])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_flt(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

    let mut input: Vec<[f64; 3]> = Vec::new();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10) as f64, rng.gen_range(0..10) as f64, 1.0])
    }
    let before = Instant::now();
    let hi = intervalues::combine_intervals_flt_values(input);
    let after = Instant::now();
    println!("{:?} in {:?}", hi, after - before);

}
