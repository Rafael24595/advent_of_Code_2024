use std::env;

use dotenv::dotenv;

pub mod  utils;

pub mod exercise_1;
pub mod exercise_2;
pub mod exercise_3;
pub mod exercise_4;
pub mod exercise_5;
pub mod exercise_6;
pub mod exercise_7;
pub mod exercise_8;
pub mod exercise_9;

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

    /* 04/12/2024 */
    try_execute_group(&exercises, "4", exercise_4::exercise_4_1, exercise_4::exercise_4_2);

    /* 05/12/2024 */
    try_execute_group(&exercises, "5", exercise_5::exercise_5_1, exercise_5::exercise_5_2);

    /* 06/12/2024 */
    try_execute_group(&exercises, "6", exercise_6::exercise_6_1, exercise_6::exercise_6_2);

    /* 07/12/2024 */
    try_execute_group(&exercises, "7", exercise_7::exercise_7_1, exercise_7::exercise_7_2);

    /* 08/12/2024 */
    try_execute_group(&exercises, "8", exercise_8::exercise_8_1, exercise_8::exercise_8_2);

    /* 09/12/2024 */
    try_execute_group(&exercises, "9", exercise_9::exercise_9_1, exercise_9::exercise_9_2);

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