use std::collections::HashMap;

use crate::utils;

pub(crate) fn exercise_11_1() {

    utils::print_title("EXERCISE 11.1");

    let content = utils::read_to_string("EXERCISE_XI_I.txt");

    let start = utils::now();
    let stones = parse_data(&content);
    let total = calculate_stones_blinks(stones, 25);
    let end = utils::now();
    
    utils::print_result(total, start, end);

}

pub(crate) fn exercise_11_2() {

    utils::print_title("EXERCISE 11.2");

    let content = utils::read_to_string("EXERCISE_XI_I.txt");

    let start = utils::now();
    let stones = parse_data(&content);
    let total = calculate_stones_blinks(stones, 75);
    let end = utils::now();
    
    utils::print_result(total, start, end);
    
}

fn calculate_stones_blinks(stones: Vec<u64>, blinks: usize) -> u64 {
    let mut cache = HashMap::new();
    let mut result = 0;
    for stone in stones {
        result += calculate_stone_blinks(stone, blinks, &mut cache)
    }
    result
}

fn calculate_stone_blinks(stone: u64, blink: usize, cache: &mut HashMap<(usize, u64), u64>) -> u64 {
    if blink == 0 {
        return 1;
    }

    let key = (blink, stone);
    if let Some(result) = cache.get(&key) {
        return *result;
    } 
    
    if stone == 0 {
        let value = calculate_stone_blinks(1, blink - 1, cache);
        cache.insert(key, value);
        return value;
    } 
    
    let len = ((stone as f64).log10() + 1f64) as u32;
    let value = match len % 2 == 0 {
        true => {
            let left_stone = stone / 10u64.pow(len / 2);
            let right_stone = stone % 10u64.pow(len / 2);
            let result_left = calculate_stone_blinks(left_stone, blink - 1, cache);
            let result_right = calculate_stone_blinks(right_stone, blink - 1, cache);
            result_left + result_right
        },
        false => calculate_stone_blinks(stone * 2024, blink - 1, cache)
    };

    cache.insert(key, value);

    value
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> Vec<u64> {
    content.replace("\r\n", "\n")
        .split_whitespace()
        .filter(|i| !i.is_empty())
        .map(|r| r.parse::<u64>().expect("Not a number"))
        .collect::<Vec<u64>>()
}
