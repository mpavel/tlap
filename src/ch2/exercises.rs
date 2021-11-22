fn half_of_a_square() {
    println!("Half of a square:");

    for i in 1..6 {
        for _hash_num in 1..(6 - i + 1) {
            print!("#");
        }
        println!();
    }
}

fn sideways_triangle() {
    println!("Sideways triangle:");

    for row in 1..8 {
        let empty_spaces = i16::abs(4 - row);
        let hashes = 4 - empty_spaces;

        for _hash_num in 1..(hashes + 1) {
            print!("#");
        }
        println!();
    }
}

pub fn run() {
    half_of_a_square();
    sideways_triangle();
}
