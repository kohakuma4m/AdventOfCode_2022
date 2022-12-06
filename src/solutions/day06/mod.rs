use std::collections::HashSet;

pub fn solution1(data: String) -> usize {
    let result = match find_marker_start(data, 4) {
        Some(result) => result,
        None => panic!("No marker found!")
    };

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let result = match find_marker_start(data, 14) {
        Some(result) => result,
        None => panic!("No marker found!")
    };

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

fn find_marker_start(data: String, marker_size: usize) -> Option<usize> {
    let buffer: Vec<&str> = data.split("").filter(|c| *c != "").collect();
    let buffer_len = buffer.len();

    let mut i: usize = marker_size;
    while i <= buffer_len && HashSet::<&&str>::from_iter(&buffer[i - marker_size..i]).len() < marker_size {
        i += 1;
    }

    if i > buffer_len {
        return None; // No valid marker found
    }

    Some(i)
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
        assert_eq!(7, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(19, solution2(data));
    }
}
