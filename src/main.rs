mod challenges;

fn main() {
    let (x, d) = challenges::challenge2_a::solve();

    println!("Challenge 2A Solution: {}", x * d);

    let (x, d) = challenges::challenge2_b::solve();

    println!("Challenge 2B Solution: {}", x * d);
}
