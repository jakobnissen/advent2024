pub fn solve(s: &str) -> (u32, u32) {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut enabled = true;
    for cap in re.captures_iter(s) {
        let whole = &cap[0];
        if whole.starts_with("mul(") {
            let sm = cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
            p1 += sm;
            p2 += sm * (enabled as u32);
        } else if whole.starts_with("do()") {
            enabled = true;
        } else if whole.starts_with("don'") {
            enabled = false;
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    static TEST_STR_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static TEST_STR_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR_1).0, 161);
        assert_eq!(super::solve(TEST_STR_2).1, 48);
    }
}
