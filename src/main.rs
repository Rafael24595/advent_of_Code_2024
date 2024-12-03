use std::env;

use dotenv::dotenv;

pub mod  utils;

pub mod exercise_1;
pub mod exercise_2;
pub mod exercise_3;

fn main() {
    dotenv().ok();

    let exercises_string = env::var("EXERCISES").unwrap_or_default();
    let exercises = exercises_string.split(",")
        .filter(|e| !e.is_empty())
        .collect::<Vec<&str>>();

    /* 01/12/2024 */
    try_execute_group(&exercises, "1", exercise_1::exercise_1_1, exercise_1::exercise_1_2);

    /* 02/12/2024 */
    try_execute_group(&exercises, "2", exercise_2::exercise_2_1, exercise_2::exercise_2_2);

    /* 03/12/2024 */
    try_execute_group(&exercises, "3", exercise_3::exercise_3_1, exercise_3::exercise_3_2);

}

fn try_execute_group(exercises: &Vec<&str>, group: &str, function_1: fn(), function_2: fn()) {
    try_execute(&exercises, group, &format!("{}.1", group), function_1);
    try_execute(&exercises, group, &format!("{}.2", group), function_2);
}

fn try_execute(exercises: &Vec<&str>, group: &str, position: &str, function: fn()) {
    if exercises.is_empty() || exercises.contains(&group) || exercises.contains(&position) {
        function()
    }
}