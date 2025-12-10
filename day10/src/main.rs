use std::fs;

/// Rational number for exact arithmetic in Gaussian elimination
#[derive(Clone, Copy)]
struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "Division by zero");
        let g = gcd(num.abs(), den.abs());
        let sign = if den < 0 { -1 } else { 1 };
        Rational {
            num: sign * num / g,
            den: sign * den / g,
        }
    }

    fn zero() -> Self {
        Rational { num: 0, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn to_i64(&self) -> Option<i64> {
        if self.den == 1 || self.num % self.den == 0 {
            Some(self.num / self.den)
        } else {
            None
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl std::ops::Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}

impl std::ops::Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Rational::new(self.num * other.den, self.den * other.num)
    }
}

/// Parse buttons from line: extracts (x,y,z) patterns
fn parse_buttons(line: &str) -> Vec<Vec<usize>> {
    let mut buttons = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '(' {
            let btn_str: String = chars.by_ref().take_while(|&c| c != ')').collect();
            let indices: Vec<usize> = btn_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            buttons.push(indices);
        } else if c == '{' {
            break;
        }
    }
    buttons
}

/// Part 1: Parse indicator lights target [.##.] as bitmask
fn parse_line_part1(line: &str) -> (u64, Vec<u64>) {
    let start = line.find('[').unwrap() + 1;
    let end = line.find(']').unwrap();
    let target_str = &line[start..end];

    let target = target_str
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0u64, |acc, (i, _)| acc | (1 << i));

    let buttons: Vec<u64> = parse_buttons(line)
        .into_iter()
        .map(|indices| indices.iter().fold(0u64, |acc, &i| acc | (1 << i)))
        .collect();

    (target, buttons)
}

/// Part 2: Parse joltage targets {x,y,z}
fn parse_line_part2(line: &str) -> (Vec<i64>, Vec<Vec<usize>>) {
    let buttons = parse_buttons(line);

    let start = line.find('{').unwrap() + 1;
    let end = line.find('}').unwrap();
    let targets: Vec<i64> = line[start..end]
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    (targets, buttons)
}

/// Part 1: Brute force XOR - try all 2^n button combinations
fn solve_machine_part1(target: u64, buttons: &[u64]) -> u32 {
    let n = buttons.len();
    if n == 0 {
        return if target == 0 { 0 } else { u32::MAX };
    }

    (0..(1u64 << n))
        .filter_map(|mask| {
            let mut state = 0u64;
            let mut presses = 0u32;
            for (i, &btn) in buttons.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    state ^= btn;
                    presses += 1;
                }
            }
            if state == target { Some(presses) } else { None }
        })
        .min()
        .unwrap_or(u32::MAX)
}

/// Part 2: Gaussian elimination + free variable search
fn solve_machine_part2(targets: &[i64], buttons: &[Vec<usize>]) -> i64 {
    let n_buttons = buttons.len();
    let n_counters = targets.len();

    if n_buttons == 0 {
        return if targets.iter().all(|&t| t == 0) { 0 } else { i64::MAX };
    }

    // Build augmented matrix [A | b]
    let mut matrix: Vec<Vec<Rational>> = vec![vec![Rational::zero(); n_buttons + 1]; n_counters];
    for (j, btn) in buttons.iter().enumerate() {
        for &idx in btn {
            if idx < n_counters {
                matrix[idx][j] = Rational::new(1, 1);
            }
        }
    }
    for i in 0..n_counters {
        matrix[i][n_buttons] = Rational::new(targets[i], 1);
    }

    // Gaussian elimination to reduced row echelon form
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..n_buttons {
        if let Some(pr) = (row..n_counters).find(|&r| !matrix[r][col].is_zero()) {
            matrix.swap(row, pr);
            pivot_cols.push(col);

            let pivot = matrix[row][col];
            for r in 0..n_counters {
                if r != row && !matrix[r][col].is_zero() {
                    let factor = matrix[r][col] / pivot;
                    for c in 0..=n_buttons {
                        let sub = matrix[row][c] * factor;
                        matrix[r][c] = matrix[r][c] - sub;
                    }
                }
            }
            row += 1;
        }
    }

    // Check for inconsistency
    if (row..n_counters).any(|r| !matrix[r][n_buttons].is_zero()) {
        return i64::MAX;
    }

    // Identify free variables
    let free_vars: Vec<usize> = (0..n_buttons)
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    // Compute solution for given free variable values
    let compute_solution = |free_values: &[i64]| -> Option<(i64, Vec<i64>)> {
        let mut solution = vec![Rational::zero(); n_buttons];

        for (i, &fv) in free_vars.iter().enumerate() {
            solution[fv] = Rational::new(free_values[i], 1);
        }

        for (i, &pc) in pivot_cols.iter().enumerate() {
            let pivot = matrix[i][pc];
            if pivot.is_zero() { continue; }

            let mut val = matrix[i][n_buttons];
            for j in 0..n_buttons {
                if j != pc {
                    val = val - matrix[i][j] * solution[j];
                }
            }
            solution[pc] = val / pivot;
        }

        let mut total = 0i64;
        let mut int_solution = vec![0i64; n_buttons];
        for (i, s) in solution.iter().enumerate() {
            let v = s.to_i64()?;
            if v < 0 { return None; }
            int_solution[i] = v;
            total += v;
        }
        Some((total, int_solution))
    };

    // If no free variables, return unique solution
    if free_vars.is_empty() {
        return compute_solution(&[]).map(|(t, _)| t).unwrap_or(i64::MAX);
    }

    // Search over all free variable combinations
    let max_free = (*targets.iter().max().unwrap_or(&0) + 1) as usize;
    let mut best = i64::MAX;

    fn search(
        idx: usize,
        free_vars: &[usize],
        values: &mut Vec<i64>,
        max_free: usize,
        best: &mut i64,
        compute: &impl Fn(&[i64]) -> Option<(i64, Vec<i64>)>,
    ) {
        if idx == free_vars.len() {
            if let Some((total, _)) = compute(values) {
                *best = (*best).min(total);
            }
            return;
        }
        for v in 0..=max_free {
            values[idx] = v as i64;
            search(idx + 1, free_vars, values, max_free, best, compute);
        }
    }

    let mut values = vec![0i64; free_vars.len()];
    search(0, &free_vars, &mut values, max_free, &mut best, &compute_solution);

    best
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (target, buttons) = parse_line_part1(line);
            solve_machine_part1(target, &buttons)
        })
        .sum()
}

fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (targets, buttons) = parse_line_part2(line);
            solve_machine_part2(&targets, &buttons)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(SAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(SAMPLE), 33);
    }

    #[test]
    fn test_part1_individual() {
        // Machine 1: [.##.] -> 2 presses
        let (target, buttons) = parse_line_part1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(solve_machine_part1(target, &buttons), 2);

        // Machine 2: [...#.] -> 3 presses
        let (target, buttons) = parse_line_part1("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(solve_machine_part1(target, &buttons), 3);

        // Machine 3: [.###.#] -> 2 presses
        let (target, buttons) = parse_line_part1("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(solve_machine_part1(target, &buttons), 2);
    }

    #[test]
    fn test_part2_individual() {
        // Machine 1: {3,5,4,7} -> 10 presses
        let (targets, buttons) = parse_line_part2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(solve_machine_part2(&targets, &buttons), 10);

        // Machine 2: {7,5,12,7,2} -> 12 presses
        let (targets, buttons) = parse_line_part2("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(solve_machine_part2(&targets, &buttons), 12);

        // Machine 3: {10,11,11,5,10,5} -> 11 presses
        let (targets, buttons) = parse_line_part2("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(solve_machine_part2(&targets, &buttons), 11);
    }
}
