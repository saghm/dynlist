#[macro_use]
extern crate dynlist;

fn main() {
    let list = dyn_list![[[1]], 2, [3, [4, 5]]];
    println!("dynlist: {}", list);

    let sum: i32 = list.iter().sum();
    println!("sum: {}", sum);

    let flattened: Vec<_> = list.into_iter().collect();
    println!("collected into vec: {:?}", flattened);
}
