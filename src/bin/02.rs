use aoc::read_lines;

use std::io;

fn main() -> io::Result<()> {
    let lines: Vec<Vec<i32>> = read_lines("data/inputs/02.txt")
        .unwrap()
        .iter_mut()
        .map(|c| {
            c.split_whitespace()
                .map(|d| d.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let (safe, not_safe): (Vec<Vec<_>>, Vec<Vec<_>>) = lines.into_iter().partition(|v| is_safe(v));

    let mut more_safe = 0;
    for r in not_safe {
        // not we remove each element one by one and test
        // and store how many removals made it safe

        let mut safety_score = 0;
        for i in 0..r.len() {
            let mut temp_row: Vec<_> = Vec::new();
            temp_row.extend_from_slice(&r[..i]);
            temp_row.extend_from_slice(&r[i + 1..]);

            if is_safe(&temp_row) {
                safety_score += 1;
            }
        }

        if safety_score >= 1 {
            more_safe += 1;
        }
    }

    dbg!(safe.len());
    dbg!(safe.len() + more_safe);

    Ok(())
}

fn is_safe(row: &Vec<i32>) -> bool {
    let diff: i32 = row[0] - row[1];

    // establish the direction
    let is_asc = if diff.is_negative() { true } else { false };

    for i in 0..(row.len() - 1) {
        let curr = row[i];
        let next = row[i + 1];

        let diff: i32 = curr - next;

        if !(1..=3).contains(&diff.abs()) {
            return false;
        } else if diff.is_negative() != is_asc {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe_test() {
        assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 6, 7, 9]), true);
    }
}
