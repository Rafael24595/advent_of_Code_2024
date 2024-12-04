use crate::utils;

const XMAS: &str = "XMAS";
const SAMX: &str  = "SAMX";
const MAS: &str = "MAS";
const SAM: &str = "SAM";

pub(crate) fn exercise_4_1() {

    utils::print_title("EXERCISE 4.1");

    let content = utils::read_to_string("EXERCISE_IV_I.txt");
    let matrix = make_matrix(&content);

    let total = find_xmas(matrix);

    utils::print_result(total);

}

pub(crate) fn exercise_4_2() {

    utils::print_title("EXERCISE 4.2");

    let content = utils::read_to_string("EXERCISE_IV_II.txt");
    let matrix = make_matrix(&content);

    let total = find_x_mas(&matrix);

    utils::print_result(total);

}

fn find_xmas(matrix: Vec<Vec<&str>>) -> i64 {
    let mut result = 0;
    let mut buffer = Vec::new();

    let patterns = [XMAS, SAMX];

    let horizontal = make_horizontal(&matrix);
    let eval_horizontal = evalue_group(horizontal, &patterns);

    buffer.push(("Horizontal coincidences", utils::ConsoleColors::CONSOLE_RESULT.wrap(eval_horizontal)));

    result += eval_horizontal;

    let vertical = make_vertical(&matrix);
    let eval_vertical = evalue_group(vertical, &patterns);

    buffer.push(("Vertical coincidences", utils::ConsoleColors::CONSOLE_RESULT.wrap(eval_vertical)));

    result += eval_vertical;
    
    let diagonals = make_diagonally(&matrix);
    let eval_diagonals = evalue_group(diagonals, &patterns);

    buffer.push(("Diagonals coincidences", utils::ConsoleColors::CONSOLE_RESULT.wrap(eval_diagonals)));

    result += eval_diagonals;

    print_buffer(buffer);

    result
}

fn evalue_group(group: Vec<String>, patterns: &[&str]) -> i64 {
    let mut result = 0;
    for row in group {
        result += evalue_patterns(&row, patterns);
    }
    result.try_into().unwrap()
}

fn make_diagonally(matrix: &Vec<Vec<&str>>) -> Vec<String> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = Vec::new();
    for cursor in 0..(rows + cols - 1) {
        let mut row = 0;
        if cursor >= cols {
            row = cursor - cols + 1;
        }

        let buffer_ne_sw = iterate_ne_sw(rows, row, cols, cursor, &matrix);
        let buffer_nw_se = iterate_nw_se(rows, row, cols, cursor, &matrix);
        result.push(buffer_ne_sw);
        result.push(buffer_nw_se);
    }

    result
}

fn make_horizontal<'a>(matrix: &Vec<Vec<&str>>) -> Vec<String> {
    matrix.iter()
        .map(|r| r.join(""))
        .collect::<Vec<String>>()
}

fn make_vertical<'a>(matrix: &Vec<Vec<&str>>) -> Vec<String> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = Vec::new();
    for col in 0..cols {
        let mut buffer = String::new();
        for row in 0..rows {
            buffer += matrix[row][col];    
        }
        result.push(buffer);
    }

    return result;
}



fn iterate_nw_se<'a>(rows: usize, row: usize, cols: usize, cursor: usize, matrix: &Vec<Vec<&str>>) -> String {
    let mut row = row;
    let mut col = cols - 1;
    if cursor < cols {
        col = cursor;
    }
    let mut buffer = String::new();
    while row < rows {
        buffer += matrix[row][col];
        row += 1;
        if col == 0 {
            break;
        }
        col -= 1;
    }
    buffer
}

fn iterate_ne_sw(rows: usize, row: usize, cols: usize, cursor: usize, matrix: &Vec<Vec<&str>>) -> String {
    let mut row = row;
    let mut col = 0;
    if cursor < cols {
        col = cols - 1 - cursor;
    }
    let mut buffer = String::new();
    while row < rows && col < cols {
        buffer += matrix[row][col];
        row += 1;
        col += 1;
    }
    buffer
}


/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/


fn find_x_mas(matrix: &Vec<Vec<&str>>) -> i64 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = Vec::new();
    for cursor in 0..(rows + cols - 1) {
        let mut row = 0;
        if cursor >= cols {
            row = cursor - cols + 1;
        }

        let diagonals = iterate_nw_se_complex(rows, row, cols, cursor, &matrix);

        result.push(diagonals);
    }

    evalue_x_mas(matrix, result)
}

fn iterate_nw_se_complex<'a>(rows: usize, row: usize, cols: usize, cursor: usize, matrix: &Vec<Vec<&str>>) -> Vec<(usize, usize, String)> {
    let mut row = row;
    let mut col = cols - 1;
    if cursor < cols {
        col = cursor;
    }
    let mut buffer = Vec::new();
    while row < rows {
        buffer.push((row, col, String::from(matrix[row][col])));
        row += 1;
        if col == 0 {
            break;
        }
        col -= 1;
    }
    buffer
}

fn evalue_x_mas(matrix: &Vec<Vec<&str>>, diagonals: Vec<Vec<(usize, usize, String)>>) -> i64 {
    let mut result = 0;

    let patterns = [MAS, SAM];

    for diagonal in diagonals {
        let row = diagonal.iter()
            .map(|(_, _, c)| c.as_str())
            .collect::<Vec<&str>>()
            .join("");
        
        let indexes = find_indexes(&row, &patterns);
        
        println!("{}: {} -> {}",
            utils::ConsoleColors::CONSOLE_BOLD.wrap("Row"),
            utils::ConsoleColors::CONSOLE_SUCESS.wrap(row), 
            utils::ConsoleColors::CONSOLE_RESULT.wrap(format!("{:?}", indexes)));

        for index in indexes {
            let (row, col, _) = &diagonal[index];

            println!("{}: {}", 
                utils::ConsoleColors::CONSOLE_BOLD.wrap("Location"),
                utils::ConsoleColors::CONSOLE_POWER.wrap(format!("{:?}", diagonal[index])));

            if row + 2 >= matrix.len() || col - 2 >= matrix[0].len() {
                continue;
            }

            let bottom_corner = matrix[row+2][*col];
            let middle = matrix[row+1][col-1];
            let top_corner = matrix[*row][col-2];

            let key = format!("{top_corner}{middle}{bottom_corner}");

            result += evalue_patterns(&key, &patterns);

            println!("{}: {} => {}", 
                utils::ConsoleColors::CONSOLE_BOLD.wrap("X"),
                utils::ConsoleColors::CONSOLE_SUCESS.wrap(key), 
                utils::ConsoleColors::CONSOLE_RESULT.wrap(result));
        }

        println!("\n");
    }
    result.try_into().unwrap()
}

fn find_indexes(row: &str, patterns: &[&str]) -> Vec<usize> {
    let mut indexes = Vec::new();
    
    for pattern in patterns {
        let mut start = 0;
        while let Some(index) = row[start..].find(pattern) {
            indexes.push(start + index);
            start += index + 1;
        }
    }

    indexes
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn make_matrix(content: &str) -> Vec<Vec<&str>> {
    content.split("\n")
        .map(|r| r.trim().split("").map(|r| r.trim()).filter(|i| i.len() > 0).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn evalue_patterns(key: &str, patterns: &[&str]) -> i64 {
    let mut result = 0;
    for pattern in patterns {
        let key_matches = key.matches(pattern).count();
        result += key_matches;
    }
    result.try_into().unwrap()
}

fn print_buffer(buffer: Vec<(&str, String)>) {
    let (longest, _) = buffer.iter()
        .max_by_key(|(m, _)| m.len())
        .expect("No data to print");
    for r in buffer.iter() {
        let message = r.0;
        let amount = &r.1;
        let spaces_tuple = " ".repeat(longest.len() - message.len());
        println!("{message}: {spaces_tuple}{amount}")
    }
}
