// Findind Duplicate within k distance
use std::collections::{HashSet, HashMap};
pub fn match_within_k(arr: &[isize], k: usize) -> bool {
    let mut hs = HashSet::new();
    for (i, el) in arr.iter().enumerate() {
        if hs.contains(el) { return true; }
        hs.insert(el);
        if i >= k { 
            let val = &arr[i -k];
            let _ = hs.remove(&val);
        }
    }
    false
}

// Rearrange array such that even positioned are greater than odd
// if i is even arr[i] should be greater than arr[i-1]
// if i is odd arr[i] should be lesser than arr[i-1]

pub fn rearrange_array(arr: &mut [isize]) {
    let mut i = 1;
    while i < arr.len() {
        if (i + 1) % 2 == 0 {
            if arr[i] < arr[i-1] {
                let temp = arr[i];
                arr[i] = arr[i-1];
                arr[i-1] = temp;
            }
            i += 1;
        }else {
            if arr[i] > arr[i-1] {
                let temp = arr[i];
                arr[i] = arr[i-1];
                arr[i-1] = temp;
            }
            i += 1;
        }
    }
}

// Stock Buying and selling 
// We need to find the maximum profite for the given array of day and price
// We can buy as many stocks as we want and sell as many as we want but we can only hold one stock
// at a time.
pub fn max_profit(prices: &[usize]) -> usize {
    let mut max = prices[0];
    let mut buy = prices[0];
    let mut profit = 0;
    for price in prices {
       if *price > max {
            max = *price;
       }
       if *price < buy {
            profit += max - buy;
            buy = *price;
            max = *price;
       }
    }
    profit += max - buy;
    profit
}

// Unique Number: return the no. which has no duplicate in the array.
pub fn unique_num(arr: &[isize]) -> isize {
    let mut hm = HashMap::new();
    for num in arr {
       hm.entry(num).and_modify(|count| *count += 1).or_insert(1); 
    }   
    for (k, val) in &hm {
        if *val == 1 { return **k; }
    }
    -1
}
// performing  XOR to find the unique num.
pub fn unique_xor(arr:&[isize]) -> isize {
    let mut res = 0;
    for num in arr {
        res = res ^ *num;
    }
    res
}
// given an array of integers represents permutatin find the missing number from the array.
fn missing_gt(arr: &[isize]) -> isize {
    let mut miss = arr[0];
    for num in arr {
        if *num != miss { return miss;}
        miss += 1;
    }
    -1
}
fn missing_lt(arr: &[isize]) -> isize {
    let mut miss = arr[0];
    for num in arr {
        if *num != miss { return miss;}
        miss -= 1;
    }
    -1
}
pub fn missing_num(arr: &[isize]) -> isize {
    if arr[0] > arr[1] { return missing_lt(arr); }
    missing_gt(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_k() {
        let u = vec![1, 2, 3, 1, 3];
        let v = vec![1, 2, 3, 4, 1];
        let (u, v) = (match_within_k(&u, 3), match_within_k(&v, 3));
        assert_eq!(u, true);
        assert_eq!(v, false);
    }
    #[test]
    fn test_rearrange() {
        let mut u = vec![1, 2, 3];
        rearrange_array(&mut u);
        let res = vec![1, 3, 2];
        assert_eq!(u, res);
    }

    #[test]
    fn test_max_profit() {
         let u = vec![100, 180, 260, 310, 40, 535, 695];
         let res = max_profit(&u);
         assert!(res == 865);
    }

    #[test]
    fn test_unique_num() {
        let v = vec![1, 2, 3, 1, 2];
        let res = unique_num(&v);
        assert!(res == 3);
    }
    #[test]
    fn test_unique_xor() {
        let v = vec![1, 2, 3, 1, 2];
        let res = unique_xor(&v);
        assert!(res == 3);
    }

}
