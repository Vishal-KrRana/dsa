fn longest_palidrome(s: String) -> String {
    todo!()
}

fn reverse(s: &mut str) -> String {
    let len = s.len() - 1;
    let string = String::new();
    for i in len..0 {
        string.push(s[i]);
    }
    string
}

fn main(){
    let mut s: &mut str = "hello".to_string();
    let rev = reverse(&mut s);
    assert_eq!("olleh".to_string(), rev.to_string()); 
}
