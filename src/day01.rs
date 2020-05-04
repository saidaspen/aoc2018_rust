use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for l in &input {
        if !l.is_empty() {
            sum += i32::from_str_radix(l.as_ref(), 10).expect("not a num");
        }
    }
    sum
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    let mut prev_freqs: HashSet<i32> = HashSet::new();
    prev_freqs.insert(0);
    'outer: loop {
        for l in &input {
            if !l.is_empty() {
                sum += i32::from_str_radix(l.as_ref(), 10).expect("not a num");
                if prev_freqs.contains(&sum) {
                    break 'outer;
                }
                prev_freqs.insert(sum);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(vec!["+1".to_string(), "-1".to_string()]));
        assert_eq!(
            10,
            part2(vec![
                "+3".to_string(),
                "+3".to_string(),
                "+4".to_string(),
                "-2".to_string(),
                "-4".to_string()
            ])
        );
        assert_eq!(
            5,
            part2(vec![
                "-6".to_string(),
                "+3".to_string(),
                "+8".to_string(),
                "+5".to_string(),
                "-6".to_string()
            ])
        );
        assert_eq!(
            14,
            part2(vec![
                "+7".to_string(),
                "+7".to_string(),
                "-2".to_string(),
                "-7".to_string(),
                "-4".to_string()
            ])
        );
    }
}
