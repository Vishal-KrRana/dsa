pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Findind Duplicate within k distance
use std::collections::HashSet;
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
            profit += (max - buy);
            buy = *price;
            max = *price;
       }
    }
    profit += (max - buy);
    profit
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

}
