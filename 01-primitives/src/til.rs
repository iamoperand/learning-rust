fn main() {
    print!("this is");
    print!("not on a new line");

    println!("");

    println!("this is a new line");

    let parameter_type = "string";
    println!("basic {} interpolation", parameter_type);
    println!("ordinal parameters: {1} & {0}", "first", "second");

    println!(
        "named parameters: {first}, {second}",
        first = "first",
        second = "second"
    );
}
