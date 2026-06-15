pub fn hello(){
    println!("Hello from array module");
    }

// This is get alternate algo's recursive version which return an array of alternating elements
fn getAlternateRec(arr: &Vec<isize>, idx: usize, res: &mut Vec<isize>) {
    if idx < arr.len() {
        res.push(arr[idx]);
        getAlternateRec(&arr, idx + 2, res);
    }
}

pub fn getAlternate(arr: Vec<isize>) -> Vec<isize> {
    let mut res = Vec::new();
    getAlternateRec(&arr, 0, &mut res);
    res
}

// is leader algo cheks if the current element is a leader or not
// leader: if the current element is bigger then the right element then it's a leader element
pub fn leader(arr: &Vec<isize>) -> Vec<isize> {
    let mut res = Vec::new();
    let mut max = arr[arr.len() - 1];
    for elm in arr.into_iter().rev() {
        if *elm > max {
            res.push(*elm);
            max = *elm;
        }
    }
    res.into_iter().rev().collect()
}

// Remove duplicates for an array
// First we would implement it for unsorted array
pub fn rm_duplicate<T>(arr: &Vec<T>) -> Vec<T> 
    where 
        T: std::cmp::Eq + std::hash::Hash + Clone
   {
    let mut res = Vec::new();
    let mut hs = std::collections::HashSet::new();
    for elm in arr {
        hs.insert(elm.clone());
    }
    for elm in &hs {
        res.push(elm.clone());
    }
    res
}

// implemention for sorted array 
pub fn rm_duplicates<T: std::cmp::PartialEq + Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut copy = &arr[0];
    let mut res = Vec::new();
    res.push(copy.clone());
    for elm in arr {
        if *copy != *elm {
            res.push(elm.clone());
            copy = &elm;
        }
    }
    res
}
// Creating sub arrays form the given array.
// not completed. 
pub fn sub_arri<T: Clone>(arr: &Vec<T>) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();
    for (i, el) in arr.iter().enumerate() {
        let mut sub_res = Vec::new();
        if i != arr.len()-1 { res.push(vec![el.clone()]); }
        for j in &arr[i..] {
            sub_res.push(j.clone());
       }
       res.push(sub_res);
    }
    res
}
// TODO: Recursive approach for sub array.

// Reversing an array
pub fn reverse<T: Clone>(arr: &mut [T]) {
    let mut end = arr.len() - 1;
    let mut start = 0;
    while start < end {
        let t = arr[start].clone();
        arr[start] = arr[end].clone();
        arr[end] = t;
        start += 1;
        end -= 1;
    }
}

// TODO: rotating an array by n
pub fn rotate<T: Clone>(arr: &mut Vec<T>, mut steps: usize) -> &mut Vec<T> {
    reverse(arr);
    reverse(&mut arr[..steps]);
    reverse(&mut arr[steps..]);
    arr
}
// -------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_alternate(){
        let v = vec![1, 2, 3, 4, 5];
        let res = vec![1, 3, 5];
        assert_eq!(getAlternate(v), res);
    }
    #[test]
    fn test_leader(){
        let u = vec![1, 15, 3, 9, 6, 0, 8];
        let res = vec![15, 9];
        assert_eq!(res, leader(&u)); 
    }

    #[test]
    fn test_duplicates(){
        let v = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
        let res = vec![1, 2, 3, 4];
        assert_eq!(rm_duplicates(&v), res);
    }

    #[test]
    fn test_reverse(){
        let mut v = vec![1, 2, 3, 4];
        reverse(&mut v);
        let rev = vec![4, 3, 2, 1];
        assert_eq!(v, rev);
    }

    #[test]
    fn test_rotate(){
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let res = vec![4, 5, 6, 1, 2, 3];
        rotate(&mut v, 3);
        assert_eq!(v, res);
    }
}
