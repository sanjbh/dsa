use std::collections::HashMap;

pub fn pair_sum_unsorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut hash_map: HashMap<i32, usize> = HashMap::new();

    for (index, &value) in nums.iter().enumerate() {
        let complement = target - value;

        if let Some(&i) = hash_map.get(&complement) {
            return Some((i, index));
        }

        hash_map.insert(value, index);
    }
    None
}

mod tests {
    use super::*;

    #[test]
    fn test_pair_sum_unsorted() {
        let result = pair_sum_unsorted(&[-1, 3, 4, 2], 3).unwrap();
        assert_eq!(result, (0, 2))
    }
}
