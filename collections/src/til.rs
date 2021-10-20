pub fn main() {
    tuples();
    structs();
}

fn tuples() {
    let unit = (); // unit tuple

    let tuple = (1, "hello");
    let one = tuple.0;
    let two = tuple.1;
    println!("{} {}", one, two);
}

fn structs() {
    let Point { x, y, z } = new_struct(1, 2, 3);
    println!("{} {} {}", x, y, z);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn new_struct(x: i32, y: i32, z: i32) -> Point {
    Point { x, y, z }
}

fn arrays() {
    let mut years: [i32; 3] = [2010, 2020, 2030];
    let first_year = years[0];
    let [_, second_year, third_year] = years;
}
