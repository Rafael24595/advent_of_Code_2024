use std::{collections::HashMap, env, thread, time::Duration, usize};

use crate::utils;

type Matrix = Vec<Vec<(usize, usize, usize)>>;
type Trails = Vec<(usize, usize)>;
type Position = (usize, usize, usize);
type Visited = HashMap<String, i64>;
type Status = HashMap<String, bool>;

pub(crate) fn exercise_10_1() {
    utils::print_title("EXERCISE 10.1");

    let content = utils::read_to_string("EXERCISE_X_I.txt");

    let start = utils::now();
    let (heads, trails, matrix) = parse_data(&content);
    let total = calculate_trails(heads, &trails, &matrix, &||utils::print_title("EXERCISE 10.1"));
    let end = utils::now();

    utils::print_result(total, start, end);
}

pub(crate) fn exercise_10_2() {
    utils::print_title("EXERCISE 10.2");

    let content = utils::read_to_string("EXERCISE_X_I.txt");

    let start = utils::now();
    let (heads, trails, matrix) = parse_data(&content);
    let total = calculate_trails_rating(heads, &trails, &matrix, &||utils::print_title("EXERCISE 10.2"));
    let end = utils::now();

    utils::print_result(total, start, end);
}

fn calculate_trails<T>(
    heads: usize,
    trails: &Trails,
    matrix: &Matrix,
    title: &T,
) -> i64
where
    T: Fn(),
{
    let is_print = env::var("EXERCISE_10_1_PRINT").unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false);
    let speed = env::var("EXERCISE_10_1_SPEED").unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(350);

    let print_function = match is_print {
        true => Some(make_print_function(speed, title)),
        false => None,
    };

    let mut result = 0;
    let mut status = HashMap::new();

    for trail in trails {
        let aux_heads = heads;
        let position = matrix[trail.0][trail.1];
        let (aux_result, _, _, aux_status) = calculate_trail(
            aux_heads,
            &position,
            matrix,
            HashMap::new(),
            status,
            &print_function,
        );
        result += aux_result;
        status = aux_status;
    }

    if let Some(print_function) = print_function {
        print_function(&status, matrix);
    }

    result
}

fn calculate_trails_rating<T>(
    heads: usize,
    trails: &Trails,
    matrix: &Matrix,
    title: &T,
) -> i64
where
    T: Fn(),
{
    let is_print = env::var("EXERCISE_10_2_PRINT")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false);
    let speed = env::var("EXERCISE_10_2_SPEED")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(350);

    let print_function = match is_print {
        true => Some(make_print_function(speed, title)),
        false => None,
    };

    let mut status = HashMap::new();
    let mut visited = HashMap::new();

    for trail in trails {
        let aux_heads = heads;
        let position = matrix[trail.0][trail.1];
        let (_, _, aux_visited, aux_status) = calculate_trail(
            aux_heads,
            &position,
            matrix,
            HashMap::new(),
            status,
            &print_function,
        );
        status = aux_status;
        visited = sum_maps(&aux_visited, visited);
    }

    if let Some(print_function) = print_function {
        print_function(&status, matrix);
    }

    visited.iter().map(|(_, v)| v).sum()
}

fn make_print_function<T>(
    speed: u64,
    title: T,
) -> impl Fn(&Status, &Matrix)
where
    T: Fn(),
{
    move |status: &Status, matrix: &Matrix| {
        thread::sleep(Duration::from_millis(speed));
        utils::clean_screen();
        title();
        print_data(&status, matrix);
        utils::reestore_cursor();
    }
}
fn calculate_trail<F>(
    mut heads: usize,
    position: &Position,
    matrix: &Matrix,
    mut visited: Visited,
    mut status: Status,
    print_function: &Option<F>,
) -> (i64, usize, Visited, Status)
where
    F: Fn(&Status, &Matrix),
{
    let mut result = 0;

    if heads == 0 {
        return (0, heads, visited, status);
    }

    let key = make_key(position);
    status.insert(key.clone(), true);

    if let Some(print_function) = print_function {
        print_function(&status, matrix);
    }

    if position.2 == 9 {
        let count = visited.get(&key);
        if let Some(count) = count {
            visited.insert(key, count + 1);
            return (0, heads, visited, status);
        }
        visited.insert(key, 1);
        heads -= 1;
        return (1, heads, visited, status);
    }

    if position.0 > 0 {
        let postion_aux = matrix[position.0 - 1][position.1];
        if postion_aux.2 == position.2 + 1 {
            let (aunx_result, aux_heads, aux_visited, aux_status) =
                calculate_trail(heads, &postion_aux, matrix, visited, status, print_function);
            result += aunx_result;
            heads = aux_heads;
            visited = aux_visited;
            status = aux_status;
        }
    }
    if position.1 < matrix[0].len() - 1 {
        let postion_aux = matrix[position.0][position.1 + 1];
        if postion_aux.2 == position.2 + 1 {
            let (aunx_result, aux_heads, aux_visited, aux_status) =
                calculate_trail(heads, &postion_aux, matrix, visited, status, print_function);
            result += aunx_result;
            heads = aux_heads;
            visited = aux_visited;
            status = aux_status;
        }
    }
    if position.0 < matrix.len() - 1 {
        let postion_aux = matrix[position.0 + 1][position.1];
        if postion_aux.2 == position.2 + 1 {
            let (aunx_result, aux_heads, aux_visited, aux_status) =
                calculate_trail(heads, &postion_aux, matrix, visited, status, print_function);
            result += aunx_result;
            heads = aux_heads;
            visited = aux_visited;
            status = aux_status;
        }
    }
    if position.1 > 0 {
        let postion_aux = matrix[position.0][position.1 - 1];
        if postion_aux.2 == position.2 + 1 {
            let (aunx_result, aux_heads, aux_visited, aux_status) =
                calculate_trail(heads, &postion_aux, matrix, visited, status, print_function);
            result += aunx_result;
            heads = aux_heads;
            visited = aux_visited;
            status = aux_status;
        }
    }

    status.remove(&key);

    (result, heads, visited, status)
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> (usize, Trails, Matrix) {
    let sanitized = content.replace("\r\n", "\n");
    let lines = sanitized.trim().split("\n").filter(|i| !i.is_empty());

    let mut matrix = Vec::new();
    let mut trails = Vec::new();
    let mut heads = 0;
    for (y, line) in lines.enumerate() {
        let mut row = Vec::new();
        let fragments = line.trim().split("").filter(|i| !i.is_empty());
        for (x, value) in fragments.enumerate() {
            let value = value.parse::<usize>().expect("Not a number");
            if value == 0 {
                trails.push((y, x));
            }
            if value == 9 {
                heads += 1;
            }
            row.push((y, x, value));
        }
        matrix.push(row);
    }

    (heads, trails, matrix)
}

fn print_data(status: &Status, matrix: &Matrix) {
    for row in matrix {
        for (y, x, value) in row {
            let key = make_key(&(*y, *x, *value));
            if status.contains_key(&key) {
                if *value == 9 {
                    print!("{}", utils::ConsoleColors::CONSOLE_FAIL.wrap(value));
                } else {
                    print!("{}", utils::ConsoleColors::CONSOLE_RESULT.wrap(value));
                }
            } else {
                print!("{}", utils::ConsoleColors::CONSOLE_POWER.wrap(value));
            }
        }
        println!();
    }
}

fn make_key(position: &Position) -> String {
    format!("{}#{}", position.0, position.1)
}

fn sum_maps(
    source: &Visited,
    mut target: Visited,
) -> Visited {
    for (key, value) in source {
        *target.entry(key.clone()).or_insert(0) += value;
    }
    target
}
