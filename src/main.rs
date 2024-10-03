use intervalues;
use rand::Rng;

fn main() {
    // nums = [Random().randint(0, 10) for _ in range(nr_intervals * 2)]
    let mut input: Vec<[isize; 3]> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1000000 {
        input.push([rng.gen_range(0..10), rng.gen_range(0..10), 1])
    }
    let hi = intervalues::combine_intervals(input);
    println!("{:?}", hi);
}
