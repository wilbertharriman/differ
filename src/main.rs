use std::cmp::max;

fn longest_common_subsequence<T: PartialEq + Clone>(s1: &[T], s2: &[T]) -> Vec<T> {
    let rows = s1.len() + 1;
    let cols = s2.len() + 1;
    let mut dp: Vec<u32> = vec![0u32; rows * cols];
    let idx = |r, c| r * cols + c;
    for row in (0..s1.len()).rev() {
        for col in (0..s2.len()).rev() {
            dp[idx(row, col)] = if s1[row] == s2[col] {
                1 + dp[idx(row + 1, col + 1)]
            } else {
                max(dp[idx(row, col + 1)], dp[idx(row + 1, col)])
            }
        }
    }
    // Construct the subsequence.
    let mut res = Vec::new();
    let (mut r, mut c) = (0, 0);
    while r < s1.len() && c < s2.len() {
        if s1[r] == s2[c] {
            res.push(s1[r].clone());
            r += 1;
            c += 1;
            continue;
        }
        if dp[idx(r + 1, c)] > dp[idx(r, c + 1)] {
            r += 1;
        } else {
            c += 1;
        }
    }
    res
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_char_subsequences() {
        let a: Vec<char> = "abcde".chars().collect();
        let b: Vec<char> = "aebcd".chars().collect();
        assert_eq!(longest_common_subsequence(&a, &b), vec!['a', 'b', 'c', 'd']);

        let b2: Vec<char> = "ace".chars().collect();
        assert_eq!(longest_common_subsequence(&a, &b2), vec!['a', 'c', 'e']);

        let c1: Vec<char> = "ABXCDEF".chars().collect();
        let c2: Vec<char> = "CDXABCD".chars().collect();
        assert_eq!(
            longest_common_subsequence(&c1, &c2),
            vec!['A', 'B', 'C', 'D']
        );
    }

    #[test]
    fn test_lcs_code_lines() {
        let program1 = vec![
            "fn main() {",
            "    println!(\"Hello program1\");",
            "    println!(\"Hello world\");",
            "    println!(\"Hello program1\");",
            "}",
        ];
        let program2 = vec![
            "fn main() {",
            "    println!(\"Hello program2\");",
            "    println!(\"Hello world\");",
            "    println!(\"Hello program2\");",
            "}",
        ];
        let expected = vec![
            "fn main() {",
            "    println!(\"Hello world\");",
            "}",
        ];
        assert_eq!(longest_common_subsequence(&program1, &program2), expected);
    }

    #[test]
    fn test_edge_cases() {
        let empty: Vec<char> = vec![];
        let letters: Vec<char> = "abc".chars().collect();
        assert_eq!(longest_common_subsequence(&empty, &empty), vec![]);

        assert_eq!(longest_common_subsequence(&empty, &letters), vec![]);

        let disjoint: Vec<char> = "xyz".chars().collect();
        assert_eq!(longest_common_subsequence(&letters, &disjoint), vec![]);
    }
}
