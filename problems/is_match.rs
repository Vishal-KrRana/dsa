fn is_match(s: String, p: String) -> bool {
        let pattern = p.chars().collect::<Vec<char>>();
        if p.len() <= 1 {
            match pattern[0] {
                '.' | '*' => (),
                _ => return false,
            }
        }
        let input = s.chars().collect::<Vec<char>>();
        for i in (0..p.len()) {
            match pattern[i] {
                '.' => {
                    println!("here with .");
                    continue
                },
                '*' => {
                    println!("here with *");
                    if i + 1 == p.len() {return true;}
                    else if input[i] != pattern[i+1] {continue;}
                },
                x => {
                    println!("here with {x}");
                    if pattern[i] == x {continue}
                    else {return false;}
                },
            }
        }
        false
    }

fn main(){
    let s = "abcab".to_string();
    let p = "a.cab".to_string();
    assert!(is_match(s, p));
}
