fn cephalopod_part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    // Find the max line length and pad all lines
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded_lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_len))
        .collect();

    // Find the operator line (last non-empty line that contains + or *)
    let op_line_idx = padded_lines
        .iter()
        .rposition(|l| l.contains('+') || l.contains('*'))
        .expect("No operator line found");

    let number_lines = &padded_lines[..op_line_idx];
    let operator_line = &padded_lines[op_line_idx];

    // Find problem boundaries by looking for columns that are all spaces
    let mut col = 0;
    let mut total: u64 = 0;

    while col < max_len {
        // Skip separator columns (all spaces)
        while col < max_len {
            let is_separator = number_lines.iter().all(|l| {
                l.chars().nth(col).map(|c| c == ' ').unwrap_or(true)
            }) && operator_line.chars().nth(col).map(|c| c == ' ').unwrap_or(true);

            if !is_separator {
                break;
            }
            col += 1;
        }

        if col >= max_len {
            break;
        }

        // Find the end of this problem (next separator column or end)
        let start_col = col;
        while col < max_len {
            let is_separator = number_lines.iter().all(|l| {
                l.chars().nth(col).map(|c| c == ' ').unwrap_or(true)
            }) && operator_line.chars().nth(col).map(|c| c == ' ').unwrap_or(true);

            if is_separator {
                break;
            }
            col += 1;
        }
        let end_col = col;

        // Extract numbers from this problem
        let mut numbers: Vec<u64> = Vec::new();
        for line in number_lines {
            let slice: String = line.chars().skip(start_col).take(end_col - start_col).collect();
            let trimmed = slice.trim();
            if !trimmed.is_empty() {
                if let Ok(num) = trimmed.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }

        // Extract operator from this problem
        let op_slice: String = operator_line.chars().skip(start_col).take(end_col - start_col).collect();
        let operator = if op_slice.contains('*') { '*' } else { '+' };

        // Calculate result
        if !numbers.is_empty() {
            let result = if operator == '*' {
                numbers.iter().product::<u64>()
            } else {
                numbers.iter().sum::<u64>()
            };
            total += result;
        }
    }

    total
}

fn cephalopod_part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    // Find the max line length and pad all lines
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded_lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_len))
        .collect();

    // Find the operator line (last non-empty line that contains + or *)
    let op_line_idx = padded_lines
        .iter()
        .rposition(|l| l.contains('+') || l.contains('*'))
        .expect("No operator line found");

    let number_lines = &padded_lines[..op_line_idx];
    let operator_line = &padded_lines[op_line_idx];

    // Convert lines to char vectors for easier column access
    let number_chars: Vec<Vec<char>> = number_lines.iter().map(|l| l.chars().collect()).collect();
    let op_chars: Vec<char> = operator_line.chars().collect();

    // Find problem boundaries by looking for columns that are all spaces
    let mut col = 0;
    let mut total: u64 = 0;

    while col < max_len {
        // Skip separator columns (all spaces)
        while col < max_len {
            let is_separator = number_chars.iter().all(|row| {
                row.get(col).map(|&c| c == ' ').unwrap_or(true)
            }) && op_chars.get(col).map(|&c| c == ' ').unwrap_or(true);

            if !is_separator {
                break;
            }
            col += 1;
        }

        if col >= max_len {
            break;
        }

        // Find the end of this problem (next separator column or end)
        let start_col = col;
        while col < max_len {
            let is_separator = number_chars.iter().all(|row| {
                row.get(col).map(|&c| c == ' ').unwrap_or(true)
            }) && op_chars.get(col).map(|&c| c == ' ').unwrap_or(true);

            if is_separator {
                break;
            }
            col += 1;
        }
        let end_col = col;

        // Part 2: Extract numbers column by column (right to left)
        // Each column forms a number (top to bottom digits, skipping spaces)
        let mut numbers: Vec<u64> = Vec::new();
        for c in (start_col..end_col).rev() {
            let mut digits = String::new();
            for row in &number_chars {
                if let Some(&ch) = row.get(c) {
                    if ch.is_ascii_digit() {
                        digits.push(ch);
                    }
                }
            }
            if !digits.is_empty() {
                if let Ok(num) = digits.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }

        // Extract operator from this problem
        let operator = op_chars[start_col..end_col]
            .iter()
            .find(|&&c| c == '*' || c == '+')
            .copied()
            .unwrap_or('+');

        // Calculate result
        if !numbers.is_empty() {
            let result = if operator == '*' {
                numbers.iter().product::<u64>()
            } else {
                numbers.iter().sum::<u64>()
            };
            total += result;
        }
    }

    total
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = cephalopod_part1(input);
    println!("Part 1: {}", result1);
    let result2 = cephalopod_part2(input);
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(cephalopod_part1(input), 4277556);
    }

    #[test]
    fn test_sample_part2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(cephalopod_part2(input), 3263827);
    }
}
