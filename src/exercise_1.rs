use std::collections::HashMap;

use crate::utils;

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    utils::print_title("EXERCISE 1.1");

    let content = utils::read_to_string("EXERCISE_I_I.txt");

    let (mut list_a, mut list_b) = make_lists(&content);

    list_a.sort();
    list_b.sort();

    let mut total = 0;
    for (i, cursor) in list_a.iter().enumerate() {
        let abs = (cursor - list_b[i]).abs();
        total += abs;
    }

    utils::print_result(total);
}

fn exercise_1_2() {

    utils::print_title("EXERCISE 1.2");

    let content = utils::read_to_string("EXERCISE_I_I.txt");

    let (list_a, list_b) = make_lists(&content);

    let mut counter: HashMap<i64, i64> = HashMap::new();
    for id in list_b.iter() {
        let count = match counter.get(id) {
            Some(count) => count + 1,
            None => 1
        };
        counter.insert(*id, count);
    }

    let mut total = 0;
    for id in list_a.iter() {
        let count = counter.get(id).unwrap_or(&0);
        total += id * count;
    }

    utils::print_result(total);

}

fn make_lists(content: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = content.split("\n");

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    for line in lines {
        let tuple = line.split("   ").collect::<Vec<&str>>();
        if tuple.len() > 0 {
            list_a.push(tuple[0].trim().parse::<i64>().expect("Not a number"));
        }
        if tuple.len() > 1 {
            list_b.push(tuple[1].trim().parse::<i64>().expect("Not a number"));
        }
    }

    return (list_a, list_b);
}