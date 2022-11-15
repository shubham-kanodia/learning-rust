mod challenges;

fn main() {
    let (epsilon, gamma) = challenges::challenge3_a::solve();

    println!("Solution for challenge 3A: {}", epsilon * gamma);
}
