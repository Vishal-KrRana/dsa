pub fn hello(){
    println!("Hello from array module");
    }

// This is get alternate algo's recursive version which return an array of alternating elements
fn get_alternate_rec(arr: &Vec<isize>, idx: usize, res: &mut Vec<isize>) {
    if idx < arr.len() {
        res.push(arr[idx]);
        get_alternate_rec(&arr, idx + 2, res);
    }
}

pub fn get_alternate(arr: Vec<isize>) -> Vec<isize> {
    let mut res = Vec::new();
    get_alternate_rec(&arr, 0, &mut res);
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

// Rotating an array by n steps.

pub fn rotate<T: Clone>(arr: &mut Vec<T>, steps: usize) {
    reverse(arr);
    reverse(&mut arr[..steps]);
    reverse(&mut arr[steps..]);
}

// a generic case for push zeros to end here we are pushing any number specified to the end.
// can be improved by thinking other way around 
/*
#include <iostream>
#include <vector>
using namespace std;

void pushZerosToEnd(vector<int>& arr) {
    int count = 0;
    for (int i = 0; i < arr.size(); i++) {
        if (arr[i] != 0) {
            swap(arr[i], arr[count]);
            count++;
        }
    }
}
*/
// above code is copy pasted from gfg. This is better version of the implementation below refer it.
pub fn push_all_to_end(arr: &mut [usize], num: usize) {
    let mut i = 0;
    let mut idr = None; // idex element to be replaced.
    let end_idx = arr.len()-1;
    while i != end_idx {
        if arr[i] == num {
            if i < end_idx && arr[i+1] != num {
                match idr {
                    None => {
                        arr[i] = arr[i+1];
                        arr[i+1] = num;
                        i += 1;
                    },
                    Some(x) => {
                        arr[x] = arr[i+1];
                        arr[i+1] = num;
                        let Some(v) = idr else { panic!() };
                        if i > v && arr[v+1] == num { idr = Some(v+1) } else { idr = Some(i); }
                        i += 1;
                    }
                }
              } else {
                match idr {
                    None => {
                        idr = Some(i);
                        i += 1;
                    },
                    Some(_) => {
                        i += 1;
                        continue;
                    }
                }
            }
            
        } else {
            i += 1;
            continue;
        }
    }
}

// min operation by k value to make all equal.
/* here we need to make all elements of array equal and find the number of operations required to do by the value k
 if not possible we need to return -1.
*/

pub fn min_ops(arr: &[usize], k: usize) -> isize {
    let mut max = arr[0];
    let m = arr[0] % k;
    let mut ops = 0;
    for el in arr {
        if *el % k != m { return -1; }
        if *el > max { max = *el; }
    }
    for el in arr {
        let mut v = *el;
        while v != max {
            v += k;
            ops += 1;
        }
    }
    ops
}

pub fn binary_search(arr: &[usize], target: usize) -> Option<usize> {
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i<j {
        let mid = i + (j - i)/ 2;
        if arr[mid] == target { return Some(target); }
        if arr[mid] < target {
            i = mid + 1;
        }else {
            j = mid - 1;
        }
    }
    None
}

pub fn selection_sort(arr: &mut [usize]){
    let mut i = 0;
    while i < arr.len() {
        let mut j = i;
        while j < arr.len() {
            if arr[i] >= arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}

pub fn insertion_sort(arr: &mut [usize]) {
    let mut i = 1;
    while i < arr.len() {
        let mut j = 0;
        'l2:
        while j < i {
            if arr[i] <= arr[j] {
                let temp = arr[j];
                arr[j] = arr[i];
                arr[i] = temp;
                break 'l2;
            }
            j += 1;
        }
        i += 1;
    }
}


// -------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_alternate(){
        let v = vec![1, 2, 3, 4, 5];
        let res = vec![1, 3, 5];
        assert_eq!(get_alternate(v), res);
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

    #[test]
    fn test_push_end(){
        let mut v = vec![1, 2, 3, 3, 4, 5, 3];
        let res = vec![1, 2, 4, 5, 3, 3, 3];
        push_all_to_end(&mut v, 3);
        assert_eq!(v, res);
    }
}
