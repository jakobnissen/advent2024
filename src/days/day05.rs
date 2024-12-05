use std::{cmp::Ordering, collections::HashSet};

pub fn solve(s: &str) -> (u32, u32) {
    let delimiter = s.find("\n\n").unwrap();
    let order: HashSet<(u8, u8)> = s[..delimiter]
        .lines()
        .map(|line| {
            let (x, y) = line.split_once('|').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let mut v: Vec<u8> = Vec::new();
    s[delimiter + 2..]
        .lines()
        .fold((0u32, 0u32), |(p1, p2), line| {
            v.clear();
            v.extend(line.split(',').map(|n| n.parse::<u8>().unwrap()));
            assert!((v.len() & 1) == 1); // is odd
            if v.is_sorted_by(|&a, &b| !order.contains(&(b, a))) {
                (p1 + v[v.len() / 2] as u32, p2)
            } else {
                v.sort_by(|&a, &b| {
                    if order.contains(&(a, b)) {
                        Ordering::Less
                    } else if order.contains(&(b, a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                (p1, p2 + v[v.len() / 2] as u32)
            }
        })
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (143, 143));
    }
}
