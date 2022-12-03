pub fn solution1(data: String) {
    println!("{}", data);

    println!("=========================");
    println!("Solution1: ");
    println!("=========================");
}

pub fn solution2(data: String) {
    println!("{}", data);

    println!("=========================");
    println!("Solution2: ");
    println!("=========================");
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
        assert_eq!((), solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!((), solution2(data));
    }
}
