fn password_part1(commands: Vec<String>) -> (i32, i32) {
    let mut dial = 50;
    let mut zero_count = 0;

    for cmd in commands {
        let direction = cmd.chars().next().unwrap();
        let amount: i32 = cmd[1..].parse().unwrap();

        match direction {
            'R' => dial = (dial + amount) % 100,
            'L' => dial = (dial - amount % 100 + 100) % 100,
            _ => {}
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    (dial, zero_count)
}

fn password_part2(commands: Vec<String>) -> (i32, i32) {
    let mut dial = 50;
    let mut zero_count = 0;

    for cmd in commands {
        let direction = cmd.chars().next().unwrap();
        let amount: i32 = cmd[1..].parse().unwrap();
        let old_dial = dial;

        match direction {
            'R' => dial = (dial + amount) % 100,
            'L' => dial = (dial - amount % 100 + 100) % 100,
            _ => {}
        }

        // 0을 거친 횟수 계산
        let crosses = match direction {
            'R' => (old_dial + amount) / 100,
            'L' => {
                if old_dial == 0 {
                    amount / 100
                } else {
                    (amount - old_dial + 100) / 100
                }
            }
            _ => 0,
        };
        zero_count += crosses;
    }

    (dial, zero_count)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let commands: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let (final1, zero1) = password_part1(commands.clone());
    println!("Part1 - Final: {}, Zero count: {}", final1, zero1);

    let (final2, zero2) = password_part2(commands);
    println!("Part2 - Final: {}, Zero count: {}", final2, zero2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_commands() -> Vec<String> {
        vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ]
    }

    #[test]
    fn test_password_part1() {
        assert_eq!(password_part1(test_commands()), (32, 3));
    }

    #[test]
    fn test_password_part2() {
        assert_eq!(password_part2(test_commands()), (32, 6));
    }
}
