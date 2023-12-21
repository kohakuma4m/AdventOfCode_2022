use itertools::Itertools;

pub fn solution1(data: String) -> String {
    let total: usize = data.lines().map(|line| snafu_to_decimal(line)).sum();
    let result = decimal_to_snafu(&total);

    println!("=========================");
    println!("Total: {total}");
    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(_data: String) {
    println!("*** Merry Christmas ***");
}

/////////////////////////////////////////////////

const SNAFU_BASE: usize = 5;

fn snafu_to_decimal(snafu_number: &str) -> usize {
    let mut number: usize = 0;

    for (idx, char) in snafu_number.chars().rev().enumerate() {
        let multiplier: isize = SNAFU_BASE.pow(idx as u32).try_into().unwrap();
        let value: isize = match char {
            '2' => 2 * multiplier,
            '1' => 1 * multiplier,
            '0' => 0 * multiplier,
            '-' => -1 * multiplier,
            '=' => -2 * multiplier,
            _ => panic!("Invalid snafu character {char}")
        };

        number = (number as isize + value) as usize;
    }

    number
}

fn decimal_to_snafu(number: &usize) -> String {
    let mut chars: Vec<String> = vec![];

    let mut n = number.clone();
    while n > 0 {
        let remainder = n % SNAFU_BASE;
        match remainder {
            3 => {
                chars.push(String::from("=")); // -2
                n = 1 + (n - remainder) / SNAFU_BASE;
            },
            4 => {
                chars.push(String::from("-")); // -1
                n = 1 + (n - remainder) / SNAFU_BASE;
            },
            _ => {
                chars.push(format!("{remainder}")); // 0, 1, 2
                n = (n - remainder) / SNAFU_BASE;
            }
        }
    }

    chars.iter().rev().join("")
}

/////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file() -> String {
        let current_file = std::file!();
        let test_file = current_file.replace("mod.rs", "test.txt");
        return crate::read_file(&test_file).unwrap();
    }

    #[test]
    fn test_solution1() {
        let data = read_test_file();
        assert_eq!(String::from("2=-1=0"), solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!((), solution2(data));
    }
}
