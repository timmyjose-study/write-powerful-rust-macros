use listing21::my_vec;

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("empty = {empty:#?}");

    let also_empty: Vec<i32> = my_vec! (make an empty vec);
    println!("also_empty = {also_empty:#?}");

    let three_numbers = my_vec!(1, 2, 3);
    println!("three_numbers = {three_numbers:#?}");
}
