use std::{collections::HashMap, vec};

struct Pattern {
    pattern: [u8; 4],
    weight: f64
}

fn column_height(board: &Vec<Vec<u8>>, column: i8) -> Option<u8>{
    if column < 0 || column > 6 {
        return None;
    }
    for i in 0..6{
        if board[i][column as usize] != 0 {
            return Some(6 - i as u8);
        }
    }
    Some(6)
    
}

fn height_drop(board: &Vec<Vec<u8>>, row: i8, column: i8) -> u8{
    let start_height = column_height(board, column).unwrap_or(6 as u8);
    let rowi = 6 - row;
    if start_height < rowi as u8 {
        rowi as u8 - start_height
    } else {0}
}

fn transpose(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let mut nboard = vec![vec![0; 6]; 7];
    for row in 0..board.len(){
        for col in 0..board[0].len(){
            nboard[col][row] = board[row][col];
        }
    }
    nboard
}


fn pattern_counter(board: &Vec<Vec<u8>>, player: u8, patterns: &Vec<Pattern>) -> f64{
    let mut result_map: HashMap<[u8; 4], u8> = patterns.iter().map(|e| (e.pattern, 0)).collect();
    for rowi in 0..board.len(){
        let mut map: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
        for coli in 0..board[0].len(){
            apply_pattern_checking(&patterns, &mut map, board[rowi][coli], player, &mut result_map);
        }
    }
    println!("{:?}", result_map);
    let mut resultant_value = 0.0;
    for p in patterns {
        resultant_value += p.weight * result_map.get(&p.pattern).unwrap_or(&0).to_owned() as f64;
    }
    resultant_value
}

fn apply_pattern_checking(patterns: &Vec<Pattern>, map: &mut HashMap<[u8; 4], usize>, cell_value: u8, player: u8, result_map: &mut HashMap<[u8; 4], u8>) {
    let mut marked: HashMap<[u8; 4], bool> = patterns.iter().map(|e| (e.pattern, false)).collect();
    for p in patterns.iter(){
        let curr_p_index = map.get(&p.pattern).unwrap_or(&0).to_owned();
    
        if (p.pattern[curr_p_index] != 0 && cell_value == player) || 
            (p.pattern[curr_p_index] == 0 && cell_value == 0) {
            marked.insert(p.pattern, true);

            if curr_p_index + 1 == 4 {
                result_map.insert(p.pattern, result_map.get(&p.pattern).unwrap_or(&0) + 1);
                map.insert(p.pattern, 0);
            } else {
                map.insert(p.pattern, curr_p_index + 1);
            }
        }
    }
    for (arr, val) in marked{
        if !val {
            map.insert(arr, 0);
        }
    }
}


fn pattern_counter_diagonal(board: &Vec<Vec<u8>>, player: u8, patterns: &Vec<Pattern>) -> f64{
    let mut result_map: HashMap<[u8; 4], u8> = patterns.iter().map(|e| (e.pattern, 0)).collect();

    let mut rowi = 0;
    for colistart in 0..board[0].len(){
        let mut coli = colistart;
        let mut map: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
        while rowi < board.len() && coli < board[0].len() {
            apply_pattern_checking(&patterns, &mut map, board[rowi][coli], player, &mut result_map);
            rowi += 1;
            coli += 1;
        }
        rowi = 0;
    }
    let mut coli = 0;
    for rowistart in 1..board.len(){
        let mut rowi = rowistart;
        let mut map: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
        while rowi < board.len() && coli < board[0].len() {
            apply_pattern_checking(&patterns, &mut map, board[rowi][coli], player, &mut result_map);
            rowi += 1;
            coli += 1;
        }
        coli = 0;
    }
    println!("{:?}", result_map);
    let mut resultant_value = 0.0;
    for p in patterns {
        resultant_value += p.weight * result_map.get(&p.pattern).unwrap_or(&0).to_owned() as f64;
    }
    resultant_value
}

fn evaluation_function(board: &Vec<Vec<u8>>, player: u8) -> f64{
    let patterns = vec![
        Pattern{pattern: [1, 1, 0, 0], weight: 10.0},
        Pattern{pattern: [1, 0, 1, 0], weight: 10.0},
        Pattern{pattern: [1, 0, 0, 1], weight: 10.0},
        Pattern{pattern: [0, 0, 1, 1], weight: 10.0},
        Pattern{pattern: [0, 1, 0, 1], weight: 10.0},
        Pattern{pattern: [0, 1, 1, 0], weight: 10.0},

        Pattern{pattern: [1, 1, 1, 0], weight: 100.0},
        Pattern{pattern: [0, 1, 1, 1], weight: 100.0},
        Pattern{pattern: [1, 1, 0, 1], weight: 100.0},
        Pattern{pattern: [1, 0, 1, 1], weight: 100.0},

        Pattern{pattern: [1, 1, 1, 1], weight: 10000.0}
    ];
    let transposed = transpose(board);
    let horiz = pattern_counter(board, player, &patterns);
    let verti = pattern_counter(&transposed, player, &patterns);
    let diag1 = pattern_counter_diagonal(&transposed, player, &patterns);
    let diag2 = pattern_counter_diagonal(&transposed, player, &patterns);
    
    horiz + verti + diag1 + diag2
}


fn main() {
    let board: Vec<Vec<u8>> = vec![
        vec![1, 0, 0, 1, 1, 0, 1],
        vec![1, 0, 0, 2, 1, 0, 0],
        vec![0, 0, 1, 2, 2, 0, 0],
        vec![1, 0, 1, 1, 1, 0, 1],
        vec![2, 0, 2, 1, 1, 1, 1],
        vec![1, 2, 2, 1, 1, 1, 2]
    ];
    println!("{}", evaluation_function(&board, 1));
    println!("{}", evaluation_function(&board, 2));
    // println!("{:?}", pattern_counter(&board, 1, &vec![
    //         Pattern{pattern: [1, 1, 0, 1], weight: 1.0},
    //         Pattern{pattern: [1, 0, 0, 1], weight: 1.0},
    //         Pattern{pattern: [1, 1, 1, 1], weight: 1.0},
    //         Pattern{pattern: [1, 0, 1, 1], weight: 1.0}
    //     ]));
    // println!("{:?}", pattern_counter(&transpose(&board), 1, &vec![
    //         Pattern{pattern: [1, 1, 0, 1], weight: 1.0},
    //         Pattern{pattern: [1, 0, 0, 1], weight: 1.0},
    //         Pattern{pattern: [1, 1, 1, 1], weight: 1.0},
    //         Pattern{pattern: [1, 0, 1, 1], weight: 1.0}
    //     ]));
    // println!("{:?}", pattern_counter_diagonal(&board, 1, &vec![
    //         Pattern{pattern: [1, 1, 0, 1], weight: 1.0},
    //         Pattern{pattern: [1, 0, 0, 1], weight: 1.0},
    //         Pattern{pattern: [1, 1, 1, 1], weight: 1.0},
    //         Pattern{pattern: [1, 0, 1, 1], weight: 1.0}
    //     ]));
}
