use itertools::Itertools;

pub fn solution1(data: String) -> i32 {
    let values = get_values(data);
    print_values(&values);

    let max_value = *values.iter().max().unwrap();

    println!("=========================");
    println!("Solution1: {max_value}");
    println!("=========================");

    max_value
}

pub fn solution2(data: String) -> i32 {
    let sorted_values = get_values(data).iter().sorted().map(|n| *n).collect();
    print_values(&sorted_values);

    let max_3_values_sum: i32 = sorted_values.iter().rev().take(3).sum();

    println!("=========================");
    println!("Solution2: {max_3_values_sum}");
    println!("=========================");

    max_3_values_sum
}

/////////////////////////////////////////////////

fn get_values(data: String) -> Vec<i32> {
    data.lines()
        .map(|n| n.parse::<i32>().unwrap_or(0))
        .coalesce(|l1, l2| {
            if l1 == 0 || l2 == 0 {
                Err((l1, l2))
            }
            else {
                Ok(l1 + l2)
            }
        })
        .filter(|n| *n > 0)
        .collect()
}

fn print_values(values: &Vec<i32>) {
    for (idx, val) in values.iter().enumerate() {
        println!("{:02} --> {}", idx, val);
    }
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
        assert_eq!(24000, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(45000, solution2(data));
    }
}
