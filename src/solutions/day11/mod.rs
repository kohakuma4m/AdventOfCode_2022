use itertools::Itertools;

pub fn solution1(data: String) -> i64 {
    let mut monkeys = read_monkey_data(data);
    play_monkey_rounds(20, &mut monkeys, WorryReducingMethod::Factor(3), true);

    // Computing monkey business value
    let nb_inspects: Vec<i64> = monkeys.iter().map(|m| m.nb_inspects).collect();
    let result = nb_inspects.into_iter().sorted().rev().take(2).product();

    print_monkey_business(&monkeys);

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> i64 {
    let mut monkeys = read_monkey_data(data);
    play_monkey_rounds(10000, &mut monkeys, WorryReducingMethod::Modulo, false);

    print_monkey_items(&monkeys);

    // Computing monkey business value
    let nb_inspects: Vec<i64> = monkeys.iter().map(|m| m.nb_inspects).collect();
    let result = nb_inspects.into_iter().sorted().rev().take(2).product();

    print_monkey_business(&monkeys);

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash)]
struct Item {
    worry_level: i64
}

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> usize>,
    test_divisor: i64,
    nb_inspects: i64
}

enum WorryReducingMethod {
    Modulo,
    Factor(i64)
}

fn read_monkey_data(data: String) -> Vec<Monkey> {
    let mut monkeys = vec![];

    let mut lines_iter = data.lines().filter(|l| l.trim() != "");
    while let Some(line) = lines_iter.next() {
        if line.starts_with("Monkey") {
            // Starting items
            let items =
                lines_iter.next().unwrap().replace("  Starting items: ", "").split(", ").map(|l| Item { worry_level: l.parse::<i64>().unwrap() }).collect();

            // Operation
            let op_line = lines_iter.next().unwrap().replace("  Operation: new = old ", "");
            let (op, value) = op_line.split(" ").collect_tuple().unwrap();
            let operation: Box<dyn Fn(i64) -> i64> = match value {
                "old" => match op {
                    "+" => Box::new(move |old: i64| -> i64 { old + old }),
                    "*" => Box::new(move |old: i64| -> i64 { old * old }),
                    _ => panic!("Invalid operation '{op}'")
                },
                _ => {
                    let val = value.parse::<i64>().unwrap();
                    match op {
                        "+" => Box::new(move |old: i64| -> i64 { old + val }),
                        "*" => Box::new(move |old: i64| -> i64 { old * val }),
                        _ => panic!("Invalid operation '{op}'")
                    }
                }
            };

            // Test
            let divisor = lines_iter.next().unwrap().replace("  Test: divisible by ", "").parse::<i64>().unwrap();
            let m1_idx = lines_iter.next().unwrap().replace("    If true: throw to monkey ", "").parse::<usize>().unwrap();
            let m2_idx = lines_iter.next().unwrap().replace("    If false: throw to monkey ", "").parse::<usize>().unwrap();
            let test_divisor = divisor.clone();
            let test = Box::new(move |current: i64| -> usize {
                match current % divisor == 0 {
                    true => m1_idx,
                    false => m2_idx
                }
            });

            monkeys.push(Monkey { items, operation, test, test_divisor, nb_inspects: 0 });
        }
    }

    monkeys
}

fn play_monkey_rounds(nb_rounds: i64, monkeys: &mut Vec<Monkey>, manage_worry_level: WorryReducingMethod, show_monkey_items: bool) {
    let lower_worry_level_fn: Box<dyn Fn(i64) -> i64> = match manage_worry_level {
        WorryReducingMethod::Modulo => {
            // Modulo least common test divisor won't affect test
            let least_common_divisor: i64 = monkeys.iter().map(|m| m.test_divisor).product();
            Box::new(move |val: i64| -> i64 { val % least_common_divisor })
        },
        WorryReducingMethod::Factor(factor) => Box::new(move |val: i64| -> i64 { val / factor })
    };

    for n in 0..nb_rounds {
        if show_monkey_items || (n + 1) % (nb_rounds / 10) == 0 {
            println!("Playing round #{}...", n + 1);
        }

        for current_monkey_idx in 0..monkeys.len() {
            let monkey = &mut monkeys[current_monkey_idx];

            // Temp map to store items to throw to other monkeys
            let mut thrown_items: Vec<(Item, usize)> = vec![];

            while let Some(mut item) = monkey.items.pop() {
                // Inspecting item and lowering worry level
                item.worry_level = lower_worry_level_fn((monkey.operation)(item.worry_level));
                monkey.nb_inspects += 1;

                // Testing item
                let other_monkey_idx = (monkey.test)(item.worry_level);
                thrown_items.push((item, other_monkey_idx));
            }

            // Throwing item at other monkeys last because we can't borrow other mutable monkey at the same time...
            while let Some((item, other_monkey_idx)) = thrown_items.pop() {
                let other_monkey = &mut monkeys[other_monkey_idx];
                other_monkey.items.push(item);
            }
        }

        if show_monkey_items {
            print_monkey_items(&monkeys);
        }
    }
}

fn print_monkey_items(monkeys: &Vec<Monkey>) {
    println!("----------------------------------------");
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", idx + 1, monkey.items.iter().map(|i| i.worry_level).collect::<Vec<i64>>());
    }
    println!("----------------------------------------");
}

fn print_monkey_business(monkeys: &Vec<Monkey>) {
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", idx + 1, monkey.nb_inspects);
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
        assert_eq!(10605, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(2713310158, solution2(data));
    }
}
