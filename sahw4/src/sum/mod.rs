pub fn sum(numbers: &[u32]) -> Option<u32> {
    if numbers.is_empty() {
        return None;
    }

    let mut sum = 0u32;
    for n in numbers {
        if u32::MAX - sum < *n {
            return None;
        }
        sum += n;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_case() {
        let numbers = [0u32, 1, 2, 3, 4];
        assert_eq!(sum(numbers.as_slice()), Some(10));
    }

    #[test]
    fn empty_case() {
        assert_eq!(sum(&[]), None);
    }

    #[test]
    fn overflow_case() {
        let numbers = [1, u32::MAX, 3].as_slice();
        assert_eq!(sum(numbers), None);
    }
}
