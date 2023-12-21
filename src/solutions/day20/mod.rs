use itertools::Itertools;
use std::rc::Rc;

pub fn solution1(data: String) -> isize {
    let values = read_values(data);

    #[cfg(test)]
    println!("{:?}", values);

    let decryp_values = decrypt(&values, 1, None);

    #[cfg(test)]
    println!("{:?}", decryp_values);

    let (zero_idx, _) = decryp_values.iter().find_position(|v| **v == 0).unwrap();
    let groove_coordinates: Vec<isize> = vec![1000, 2000, 3000].iter().map(|offset| decryp_values[(zero_idx + *offset) % decryp_values.len()]).collect();
    let result = groove_coordinates.iter().sum();

    println!("=========================");
    println!("Zero index: {zero_idx}");
    println!("Groove coordinates: {:?}", groove_coordinates);
    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> isize {
    let values = read_values(data);

    #[cfg(test)]
    println!("{:?}", values);

    let decryp_values = decrypt(&values, 10, Some(DECRYPTION_KEY));

    #[cfg(test)]
    println!("{:?}", decryp_values);

    let (zero_idx, _) = decryp_values.iter().find_position(|v| **v == 0).unwrap();
    let groove_coordinates: Vec<isize> = vec![1000, 2000, 3000].iter().map(|offset| decryp_values[(zero_idx + *offset) % decryp_values.len()]).collect();
    let result = groove_coordinates.iter().sum();

    println!("=========================");
    println!("Zero index: {zero_idx}");
    println!("Groove coordinates: {:?}", groove_coordinates);
    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

const DECRYPTION_KEY: isize = 811589153;

fn read_values(data: String) -> Vec<isize> {
    data.lines().map(|line| line.parse::<isize>().unwrap()).collect()
}

fn decrypt(values: &Vec<isize>, nb_rounds: usize, decryption_key: Option<isize>) -> Vec<isize> {
    let d_key = match decryption_key {
        Some(key) => key,
        None => 1
    };

    // Since values are not unique, we must wrap them inside unique pointer during process to be able to find them by reference...
    let value_pointers: Vec<Rc<isize>> = values.iter().map(|v| Rc::new(*v * d_key)).collect();

    // Circular list with no start/end position (i.e: value positions could move around, but relative ordering will be conserved)
    let mut new_values: Vec<Rc<isize>> = value_pointers.iter().map(|vp| Rc::clone(vp)).collect();

    println!("Decrypting...");
    let nb_values = values.len();
    for round_idx in 0..nb_rounds {
        println!("Round #{}", round_idx + 1);

        for idx in 0..values.len() {
            // Finding current value to move from original array in at most O(n)
            let (current_idx, current_value) = new_values.iter().find_position(|vp| Rc::ptr_eq(vp, &value_pointers[idx])).unwrap();

            // Minimum number of swap move to make without looping around the list more than once
            let nb_swaps = current_value.as_ref() % (nb_values - 1) as isize; // To get back to starting position require 1 swap less than number of values

            // Swapping current value to the right or left (n = value) time in at most O(n)
            let mut current = current_idx;
            if nb_swaps >= 0 {
                for _ in 0..nb_swaps {
                    let next = (current + 1) % nb_values;
                    new_values.swap(current, next);
                    current = next;
                }
            }
            else {
                for _ in 0..-nb_swaps {
                    let previous = if current == 0 {
                        nb_values - 1
                    }
                    else {
                        current - 1
                    };
                    new_values.swap(current, previous);
                    current = previous;
                }
            }
        }
    }

    // Unwrapping list pointer values
    new_values.iter().map(|vp| *vp.as_ref()).collect()
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
        assert_eq!(3, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(1623178306, solution2(data));
    }
}
