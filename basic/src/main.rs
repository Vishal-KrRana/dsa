use basic::algo::array;

fn main() { 
    let mut v = vec![1, 2, 3, 4, 5, 6, 6];
    array::rotate(&mut v, 3);
    for elm in v {
        print!("{elm} ");
    }
}
