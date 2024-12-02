use std::num::NonZeroUsize;

pub fn solve(s: &str) -> (u64, u64) {
    let mut v: Vec<i64> = Vec::new();
    s.lines().fold((0, 0), |(p1, p2), line| {
        v.clear();
        v.extend(
            line.split_ascii_whitespace()
                .map(|field| field.parse::<i64>().unwrap()),
        );
        // Check if report is safe when we don't skip any elements
        let ind = unsafe_index(&v, usize::MAX);

        // If some index element means the report is not safe, then there can be the following reasons:
        // 1: The element had too large a diff, in which case removing it will solve the problem,
        // 2. The two previous elements were monotonically in/decreasing in one direction but this one
        // goes the other way, in which case removing either this, or any of the two previous will solve
        // the problem.
        if let Some(i) = ind {
            (
                p1,
                p2 + ((i.get().saturating_sub(2))..=i.get()).any(|j| unsafe_index(&v, j).is_none())
                    as u64,
            )
        } else {
            // If report is safe, then it's also safe using the Problem Dampener
            (p1 + 1, p2 + 1)
        }
    })
}

// Returns the first index that indicates an unsafe report v,
// skipping the value at index `skip`
fn unsafe_index(v: &[i64], skip: usize) -> Option<NonZeroUsize> {
    let mut it = v.iter().enumerate().filter(|&(i, _)| i != skip);
    let first = *it.next()?.1;
    let mut last = *it.next()?.1;
    let diff = (first - last).abs();
    if !(1..=3).contains(&diff) {
        return Some(1.try_into().unwrap());
    }
    let increasing = last > first;
    for (i, x) in it {
        let diff = (x - last).abs();
        if !(1..=3).contains(&diff) || increasing != (*x > last) {
            // Safety: Since we already removed the first two elements of the enumerate
            // iterator, we know `i` is at least 2.
            debug_assert!(i > 1);
            unsafe {
                return Some(NonZeroUsize::new_unchecked(i));
            }
        }
        last = *x;
    }
    None
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (2, 4));
    }
}
