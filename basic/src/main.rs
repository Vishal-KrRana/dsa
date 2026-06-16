use basic::algo::array;

fn main() { 
    let mut v = vec![4, 7, 19, 16];
    let ops = array::min_ops(&v, 3);
    println!("{ops}");
}
