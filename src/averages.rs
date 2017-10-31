use std::collections::HashMap;

// Given a list of integers, use a vector and return the mean (average),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.
// returns a tuple: (mean, median, mode)
pub fn averages(nums: &Vec<i32>) -> Option<(i32, i32, i32)> {
    let len = nums.len();
    if len == 0 {
        return None;
    }
    let mut sum = 0;
    let mut counts = HashMap::new();
    let mut mode = 0;
    let mut mode_count = 0;
    for num in nums {
        sum += *num;
        let count = counts.entry(num).or_insert(0);
        *count += 1;
        if *count > mode_count {
            mode = *num;
            mode_count = *count;
        }
    }
    let mean = sum/len as i32;
    let mut sorted = nums.clone();
    sorted.sort();
    let median = sorted[len/2]; // this should be do-able in the first loop..
    Some((mean, median, mode))
}

#[cfg(test)]
mod averages_tests {
    use super::*;

    #[test]
    fn averages_single() {
        let input = vec![1];
        let actual = averages(&input);
        let expected = Some((1,1,1));
        assert_eq!(actual, expected);
    }

    #[test]
    fn averages_multiple() {
        let input = vec![1,2,3,3,3,0];
        let actual = averages(&input);
        let expected = Some((2,3,3));
        assert_eq!(actual, expected);
    }

    #[test]
    fn averages_lots_of_zeros() {
        let input = vec![1,2,3,3,3,0,0,0,0,0,0,12];
        let actual = averages(&input);
        let expected = Some((2,1,0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn averages_empty_vec() {
        let input = vec![];
        let actual = averages(&input);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
