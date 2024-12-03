use std::cmp::Ordering;

struct SameIter<'a> {
    next: &'a [i64],
}

impl Iterator for SameIter<'_> {
    type Item = (usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let fst = if let Some(x) = self.next.first() {
            *x
        } else {
            return None;
        };
        unsafe {
            // Safety: We just checked there is at least one element
            let rest = self.next.split_at_unchecked(1).1;
            let first_diff = rest.iter().position(|&i| i != fst).unwrap_or(rest.len());
            // Safety: The result of `position` is always in [0, self.len()), and the alternative value
            // of rest.len() is also inbounds.
            self.next = rest.split_at_unchecked(first_diff).1;
            Some((first_diff + 1, fst))
        }
    }
}

pub fn solve(s: &str) -> (i64, i64) {
    let (mut left, mut right) = s
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            let a = a.parse::<i64>().unwrap();
            let b = b.parse::<i64>().unwrap();
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
    let mut lefts = SameIter { next: &left };
    let mut rights = SameIter { next: &right };
    let mut elem_left = lefts.next();
    let mut elem_right = rights.next();
    let mut p2 = 0;
    while let (Some((ln, lx)), Some((rn, rx))) = (elem_left, elem_right) {
        match lx.cmp(&rx) {
            Ordering::Less => {
                elem_left = lefts.next();
            }
            Ordering::Greater => {
                elem_right = rights.next();
            }
            Ordering::Equal => {
                p2 += lx * (ln * rn) as i64;
                elem_left = lefts.next();
                elem_right = rights.next();
            }
        }
    }
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
