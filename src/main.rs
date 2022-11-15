mod challenges;

fn main() {
    let (x, d) = challenges::challenge2A::solve();

    println!("Challenge 2A Solution: {}", x * d);

    let (x, d) = challenges::challenge2B::solve();

    println!("Challenge 2B Solution: {}", x * d);
}
