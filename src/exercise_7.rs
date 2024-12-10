use std::env;

use crate::utils;

pub(crate) fn exercise_7_1() {

    utils::print_title("EXERCISE 7.1");

    let content = utils::read_to_string("EXERCISE_VII_I.txt");

    let start = utils::now();
    let tests = parse_data(&content);
    let total = evalue_tests(tests);
    let end = utils::now();

    utils::print_result(total, start, end);

}

pub(crate) fn exercise_7_2() {

    utils::print_title("EXERCISE 7.2");

    let content = utils::read_to_string("EXERCISE_VII_I.txt");

    let start = utils::now();
    let tests = parse_data(&content);
    let total = evalue_tests_complex(tests);
    let end = utils::now();

    utils::print_result(total, start, end);

}

fn evalue_tests(tests: Vec<(i128, Vec<i128>)>) -> i128 {
    let mut total = 0;
    let optimized = env::var("EXERCISE_7_1_OPTIMIZED").unwrap_or_default().parse::<bool>().unwrap_or(true);
    let mut buffer = Vec::new();
    for test in tests {
        let result = test.0;
        let cursor = test.1[0];
        let components = &test.1[1..];
        if !optimized {
            if evaluator_parametrized(result, cursor, components, &scenario_maker) {
                total += result;
                buffer.push(format!("{} => {}", 
                    utils::ConsoleColors::CONSOLE_SUCESS.wrap(result),
                    utils::ConsoleColors::CONSOLE_RESULT.wrap(total),
                ));
            } else {
                buffer.push(format!("{} => {}", 
                    utils::ConsoleColors::CONSOLE_FAIL.wrap(result),
                    utils::ConsoleColors::CONSOLE_RESULT.wrap(total),
                ));
            }
        } else {
            if evaluator_without_concat(result, cursor, components) {
                total += result;
            }
        }
    }
    if !optimized {
        print_data(&buffer);
    }
    total
}

fn evalue_tests_complex(tests: Vec<(i128, Vec<i128>)>) -> i128 {
    let mut total = 0;
    let optimized = env::var("EXERCISE_7_2_OPTIMIZED").unwrap_or_default().parse::<bool>().unwrap_or(true);
    let mut buffer = Vec::new();
    for test in tests {
        let result = test.0;
        let cursor = test.1[0];
        let components = &test.1[1..];
        if !optimized {
            if evaluator_parametrized(result, cursor, components, &scenario_maker) {
                total += result;
                buffer.push(format!("{} {} => {}", 
                    utils::ConsoleColors::CONSOLE_SUCESS.wrap(result),
                    utils::ConsoleColors::CONSOLE_POWER.wrap("*"),
                    utils::ConsoleColors::CONSOLE_RESULT.wrap(total),
                ));
            } else if evaluator_parametrized(result, cursor, components, &scenario_maker_complex)  {
                total += result;
                buffer.push(format!("{} => {}", 
                    utils::ConsoleColors::CONSOLE_SUCESS.wrap(result),
                    utils::ConsoleColors::CONSOLE_RESULT.wrap(total),
                ));
            } else {
                buffer.push(format!("{} => {}", 
                    utils::ConsoleColors::CONSOLE_FAIL.wrap(result),
                    utils::ConsoleColors::CONSOLE_RESULT.wrap(total),
                ));
            }
        } else {
            if evaluator_without_concat(result, cursor, components) 
            || evaluator_with_concat(result, cursor, components)  {
                total += result;
            }
        }
    }
    if !optimized {
        print_data(&buffer);
    }
    total
}

fn evaluator_parametrized<F>(result: i128, cursor: i128, components: &[i128], scenario_maker: &F) -> bool where F: Fn(i128, i128) -> Vec<i128> {
    if cursor > result {
        return false;
    }

    if components.is_empty() {
        return cursor == result;
    }

    let next_cursor = components[0];
    let next_components = &components[1..];

    let scenarios = scenario_maker(cursor, next_cursor);

    for scenario in scenarios {
        if evaluator_parametrized(result, scenario, next_components, scenario_maker) {
            return true;
        }
    }

    return false;
}

fn scenario_maker(cursor: i128, next_cursor: i128) -> Vec<i128> {
    vec![
        cursor + next_cursor, 
        cursor * next_cursor,
    ]
}

fn scenario_maker_complex(cursor: i128, next_cursor: i128) -> Vec<i128> {
    let zeroes = next_cursor.to_string().len() as u32;
    let concatenated = cursor * 10i128.pow(zeroes) + next_cursor;
    vec![
        cursor + next_cursor, 
        cursor * next_cursor,
        concatenated
    ]
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> Vec<(i128, Vec<i128>)> {
    let sanitized = content.replace("\r\n", "\n");
    let lines = sanitized
        .trim()
        .split("\n");

    let mut tests = Vec::new();
    for line in lines {
        let fragments = line.split(": ").collect::<Vec<&str>>();
        if fragments.len() != 2 {
            panic!("Bad format");
        }

        let result = fragments.get(0)
            .expect("Cannot ocurs")
            .trim()
            .parse::<i128>()
            .expect("Not a number");

        let components = fragments.get(1)
            .expect("Cannot ocurs")
            .trim()
            .split(" ")
            .map(|n| n.parse::<i128>().expect("Not a number"))
            .collect::<Vec<i128>>();

        tests.push((result, components));
    }

    tests
}

fn evaluator_without_concat(result: i128, cursor: i128, components: &[i128]) -> bool {
    if cursor > result {
        return false;
    }

    if components.is_empty() {
        return cursor == result;
    }

    let next_cursor = components[0];
    let next_components = &components[1..];

    if evaluator_without_concat(result, cursor + next_cursor, next_components) {
        return true;
    }

    if evaluator_without_concat(result, cursor * next_cursor, next_components) {
        return true;
    }

    return false;
}

fn evaluator_with_concat(result: i128, cursor: i128, components: &[i128]) -> bool {
    if cursor > result {
        return false;
    }

    if components.is_empty() {
        return cursor == result;
    }

    let next_cursor = components[0];
    let next_components = &components[1..];

    if evaluator_with_concat(result, cursor + next_cursor, next_components) {
        return true;
    }

    if evaluator_with_concat(result, cursor * next_cursor, next_components) {
        return true;
    }

    let zeroes = next_cursor.to_string().len() as u32;
    let concatenated = cursor * 10i128.pow(zeroes) + next_cursor;

    if evaluator_with_concat(result, concatenated, next_components) {
        return true;
    }

    return false;
}

fn print_data(buffer: &Vec<String> ) {
    for line in buffer {
        println!("{line}")
    }
}
