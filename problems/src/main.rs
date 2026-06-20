use easy::array::*;


fn main() {
    let v = [1, 2, 3, 4, 6, 7];
    let u = [7, 6, 5, 3, 2, 1];
    println!("res: {}, {}", missing_num(&u), missing_num(&v));
}
