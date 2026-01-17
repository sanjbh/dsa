pub fn pair_sum_sorted(nums: &mut [i32], target: i32) -> Option<(usize, usize)> {
    if nums.is_empty() {
        return None;
    }

    nums.sort();

    let mut left: usize = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];

        match sum.cmp(&target) {
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Equal => return Some((left, right)),
            std::cmp::Ordering::Greater => right -= 1,
        }
    }

    None
}

pub fn triplet_sums(nums: &mut [i32]) -> Option<Vec<(i32, i32, i32)>> {
    if nums.is_empty() {
        return None;
    }
    let mut result = Vec::new();

    nums.sort();

    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let val_a = nums[i];
        let a = -val_a;

        let sub_slice = &mut nums[i + 1..];

        match pair_sum_sorted(sub_slice, a) {
            Some((b, c)) => {
                result.push((val_a, sub_slice[b], sub_slice[c]));
            }
            None => continue,
        }
    }

    Some(result)
}

pub fn is_palindrome(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let s: Vec<_> = s.chars().collect();

    let mut left: usize = 0;
    let mut right = s.len() - 1;

    while left < right {
        if !s[left].is_alphanumeric() {
            left += 1;
            continue;
        }

        if !s[right].is_alphanumeric() {
            right -= 1;
            continue;
        }

        if s[left].to_ascii_lowercase() == s[right].to_ascii_lowercase() {
            left += 1;
            right -= 1;
        } else {
            return false;
        }
    }

    true
}

pub fn is_palindrome_idiomatic(s: &str) -> bool {
    let chars = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    chars.clone().eq(chars.rev())
}

pub fn largest_container(heights: &[i32]) -> i32 {
    if heights.len() < 2 {
        return 0;
    }

    let mut max_area = 0;
    let mut left = 0;
    let mut right = heights.len() - 1;

    while left < right {
        let current_area = heights[left].min(heights[right]) * (right - left) as i32;
        max_area = max_area.max(current_area);

        if heights[left] < heights[right] {
            left += 1;
        } else if heights[right] < heights[left] {
            right -= 1;
        } else {
            right -= 1;
            left += 1;
        }
    }

    max_area
}

mod tests {
    use super::*;

    #[test]
    fn test_pair_sum_sorted() {
        let result = pair_sum_sorted(&mut [-5, -2, 3, 4, 6], 7);

        assert_eq!(result, Some((2, 3)));
    }

    #[test]
    fn test_triplet_sums() {
        let result = triplet_sums(&mut [0, 0, 1, -1, 1, -1]).unwrap();
        assert_eq!(result, vec![(-1, 0, 1)]);
    }

    #[test]
    fn test_is_palindrome() {
        let result = is_palindrome("12.02.2021");
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_palindrome_idiomatic() {
        let result = is_palindrome_idiomatic("12.02.2021");
        assert_eq!(result, true);
    }

    #[test]
    fn test_largest_container() {
        let result = largest_container(&[2, 7, 8, 3, 7, 6]);
        assert_eq!(result, 24);
    }
}
