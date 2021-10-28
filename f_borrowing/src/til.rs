fn print_years(years: &Vec<i32>) {
    for year in years {
        println!("year: {}", year);
    }
}

fn references_and_borrowing() {
    let years = vec![2017, 2018];
    print_years(&years);
    print_years(&years);
}

fn mutable_references() {
    let mut years = vec![2017, 2018];
    let mutable_years: &mut Vec<i32> = &mut years;

    // fn clear(&mut self) {}
    mutable_years.clear();
}

fn mutable_references_restrictions() {
    // 1. cannot borrow as mutable, as it is not declared as mutable
    let mut years: Vec<i32> = vec![1990, 1995];

    // 2. cannot borrow as mutable more than once at a time
    let mutable_borrow: &mut Vec<i32> = &mut years;
    // let mutable_borrow_2: &mut Vec<i32> = &mut years;

    // 3. cannot borrow as immutable, because it is also borrowed as mutable
    // let immutable_borrow: &Vec<i32> = &years;

    print_years(&mutable_borrow);
    // print_years(&immutable_borrow);
}

fn slices() {
    let nums = vec![1, 2, 3, 4, 5];
    let _slice = &nums[1..3];

    // string slices
    let string = String::from("hello world");
    let _str = &string[1..3];

    // slice doesn't own the elements, just references them
}

pub fn main() {
    references_and_borrowing();
    mutable_references();
    mutable_references_restrictions();
    slices();
}
