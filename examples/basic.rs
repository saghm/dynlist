#[macro_use]
extern crate dynlist;

fn main() {
    let list = dyn_list![[[1]], 2, [3, [4, 5]]];
    println!("{}", list);

    let sum: i32 = list.iter().sum();
    println!("{}", sum);

    let flattened: Vec<_> = list.into_iter().collect();
    println!("{:?}", flattened);
}
