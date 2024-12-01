use std::collections::HashMap;

pub fn solve(s: &str) -> (i64, i64) {
    let mut counts: HashMap<i64, i64> = HashMap::new();
    let (mut left, mut right) = s
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            let a = a.parse::<i64>().unwrap();
            let b = b.parse::<i64>().unwrap();
            *counts.entry(b).or_insert(0) += 1;
            (a, b)
        })
        .collect::<(Vec<_>, Vec<_>)>();
    left.sort_unstable();
    right.sort_unstable();
    let p1 = left
        .iter()
        .zip(right.iter())
        .map(|(i, j)| (i - j).abs())
        .sum();
    let p2 = left
        .iter()
        .fold(0, |acc, i| acc + i * counts.get(i).unwrap_or(&0));
    (p1, p2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (11, 31));
    }
}
