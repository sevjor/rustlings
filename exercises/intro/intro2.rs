// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let var = "Sailor!";
    for number in (1..4).rev() {
        let var = stringify!(number);
        println!("{var}, {number}");
    }
    println!("Hello {var}!");
}
