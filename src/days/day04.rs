pub fn solve(s: &str) -> (usize, usize) {
    let mut p1 = 0;
    let matrix: Vec<Vec<u8>> = s.lines().map(|line| line.as_bytes().to_vec()).collect();
    let fstlen = matrix.first().unwrap().len();
    if matrix.iter().any(|i| i.len() != fstlen) {
        panic!("Not all rows in Day 4 input have same length")
    }
    for (rowno, row) in matrix.iter().enumerate() {
        let rowno = rowno as i64;
        for (colno, byte) in row.iter().enumerate() {
            let colno = colno as i64;
            if *byte != b'X' {
                continue;
            }
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let mut found = true;
                    for (step, byte) in b"MAS".iter().enumerate() {
                        let rw = if let Some(r) =
                            matrix.get((rowno + dy * (1 + step as i64)) as usize)
                        {
                            r
                        } else {
                            found = false;
                            break;
                        };
                        let bt = if let Some(b) = rw.get((colno + dx * (1 + step as i64)) as usize)
                        {
                            b
                        } else {
                            found = false;
                            break;
                        };
                        if bt != byte {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        p1 += 1
                    }
                }
            }
        }
    }
    let mut p2 = 0;
    for (rowno, row) in matrix.iter().enumerate() {
        if rowno < 1 || rowno > matrix.len() - 2 {
            continue;
        }
        let rowno = rowno as i64;
        for (colno, byte) in row.iter().enumerate() {
            if colno < 1 || colno > row.len() - 2 {
                continue;
            }
            let colno = colno as i64;
            if *byte != b'A' {
                continue;
            }
            let mut ms = 0;
            for dy in [-1, 1] {
                for dx in [-1, 1] {
                    // Safety: We know all rows have same length (see where matrix is constructed)
                    // and we know from continue statements above that rowno-1 or rowno+1 is OOB of matrix,
                    // or that colno-1 or colno+1 is OOB of row.
                    unsafe {
                        if *matrix
                            .get_unchecked((rowno + dy) as usize)
                            .get_unchecked((colno + dx) as usize)
                            == b'M'
                            && *matrix
                                .get_unchecked((rowno - dy) as usize)
                                .get_unchecked((colno - dx) as usize)
                                == b'S'
                        {
                            ms += 1
                        }
                    }
                }
            }
            if ms > 1 {
                p2 += 1
            }
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (18, 9));
    }
}
