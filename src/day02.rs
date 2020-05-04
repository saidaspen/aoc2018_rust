#[allow(dead_code)]
pub fn part1(input: Vec<String>) -> i32 {
    let mut vals = (0, 0);
    for l in &input {
        let times = two_three_times(l);
        if times.0 > 0 {
            vals.0 += 1;
        }
        if times.1 > 0 {
            vals.1 += 1;
        }
    }
    (vals.0 * vals.1) as i32
}

#[allow(dead_code)]
pub fn part2(input: Vec<String>) -> String {
    for i in 0..input.len() {
        for j in i..input.len() {
            let line = &input[i];
            let diff = diff(line, &input[j]);
            if diff.len() == line.len() - 1 {
                return diff.to_string();
            }
        }
    }
    "".to_string()
}

pub fn diff(a: &str, b: &str) -> String {
    let mut diff = String::new();
    for (i, c) in a.char_indices() {
        if let Some(x) = b.chars().nth(i) {
            if x == c {
                diff.push(c);
            }
        }
    }
    diff
}

pub fn two_three_times(input: &str) -> (u32, u32) {
    let mut occurences = vec![0; 'z' as usize + 1];
    for c in input.chars() {
        occurences[c as usize] += 1;
    }
    (
        occurences.iter().filter(|x| **x == 2).count() as u32,
        occurences.iter().filter(|x| **x == 3).count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_three_times() {
        assert_eq!((0, 0), two_three_times("abcdef"));
        assert_eq!((1, 1), two_three_times("bababc"));
        assert_eq!((1, 0), two_three_times("abbcde"));
        assert_eq!((0, 1), two_three_times("abcccd"));
        assert_eq!((2, 0), two_three_times("aabcdd"));
        assert_eq!((0, 2), two_three_times("ababab"));
    }

    #[test]
    fn test_diff() {
        assert_eq!("abc", diff("abcx", "abcd"));
        assert_eq!("bcd", diff("xbcd", "abcd"));
        assert_eq!("abc", diff("abce", "abck"));
        assert_eq!("ab", diff("abde", "abck"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "fgij",
            part2(vec![
                "abcde".to_string(),
                "fghij".to_string(),
                "klmno".to_string(),
                "pqrst".to_string(),
                "fguij".to_string(),
                "axcye".to_string(),
                "wvxyz".to_string()
            ])
        );
    }
}
