use basic::algo::array;

fn main() { 
    let mut v = vec![26, 23, 2, 5, 24, 64, 22, 36, 69];
    array::selection_sort(&mut v);
    println!("{v:?}");
}
