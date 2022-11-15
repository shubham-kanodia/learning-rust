mod challenges;

fn main() {
    let (oxygen_rating, co2_rating) = challenges::challenge3_b::solve();

    println!("Solution for challenge 3B: {}", oxygen_rating * co2_rating);
}
