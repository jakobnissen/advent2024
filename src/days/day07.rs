use std::ops::RangeInclusive;

pub fn solve(s: &str) -> (u64, u64) {
    let mut v: Vec<u64> = Vec::new();
    let mut r: Vec<RangeInclusive<u64>> = Vec::new();
    s.lines().fold((0u64, 0u64), |(p1, p2), line| {
        let (target_str, rest) = line.split_once(": ").unwrap();
        let target = target_str.parse::<u64>().unwrap();
        v.clear();
        r.clear();
        v.extend(
            rest.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap()),
        );
        let (first, rest) = v.split_first().unwrap();
        let (mut smaller, mut larger) = (*first, *first);
        r.push(smaller..=larger);
        for &i in rest {
            smaller += i * (i.min(smaller) != 1) as u64;
            larger = larger * 10u64.pow(i.ilog10() + 1) + i;
            r.push(smaller..=larger);
        }
        if solves(&v, &r, target, false) {
            (p1 + target, p2 + target)
        } else if solves(&v, &r, target, true) {
            (p1, p2 + target)
        } else {
            (p1, p2)
        }
    })
}

// The trick here is to solve it recursively, and to prune away the recursive
// tree as soon as possible, such that fewer combinations needs testing
fn solves(v: &[u64], ranges: &[RangeInclusive<u64>], target: u64, part2: bool) -> bool {
    // We remove the last element, because the operations all have left associativity.
    // This means e.g. to solve a ? b + c = x, it's equivalent to solving
    // a ? b = x - c, peeling away the last c and solving the remainder as an instance
    // of the same problem, with one less element in the values vector, and a reduced
    // x.
    let (&last, rest) = v.split_last().unwrap();
    // In the trivial case, if there is only one element left, it needs to be equal
    // to the target
    if rest.is_empty() {
        return last == target;
    };
    let (range, rest_range) = ranges.split_last().unwrap();
    if !range.contains(&target) {
        return false;
    }
    // Here, we check all solutions where the last element is added to the rest.
    // We can immediately skip this if target < last.
    if target >= last && solves(rest, rest_range, target - last, part2) {
        return true;
    }
    // Check all solutions where last element is multiplied to the rest.
    // If last doesn't divide target, then there can be no possible solution where
    // rest adds to some number N, and then N * last == target, so we can skip that
    if target % last == 0 && solves(rest, rest_range, target / last, part2) {
        return true;
    }
    if part2 {
        // Suppose we are looking at the line 671: 4 2 71. This has the solution
        // 671 = (4 + 2) || 71. We want to check if 671 = (4 ? 2) || 71 could be a
        // solution.
        // For there to be a valid solution, the last two digits of 671 must be 71,
        // and the remainder (4 ? 2) must add to 671 with its last two digits removed.
        // That is, 671 % 10^2 == 71 && 6 = (4 ? 2)
        let ten_mask = 10u64.pow(last.ilog10() + 1);
        if target % ten_mask == last && solves(rest, rest_range, target / ten_mask, true) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (3749, 11387));
    }
}
