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

fn rot90(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{ // TODO
    let mut nboard = vec![vec![0; 6]; 7];
    for row in 0..board.len(){
        for col in 0..board[0].len(){
            nboard[board[0].len() - col - 1][row] = board[row][col];
        }
    }
    nboard
}


fn pattern_counter(board: &Vec<Vec<u8>>, player: u8, patterns: &Vec<Pattern>, transpose: bool) -> f64{
    let mut result_map: HashMap<[u8; 4], u8> = patterns.iter().map(|e| (e.pattern, 0)).collect();
    let map1: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
    let (r1, r2) = if transpose {
        (0..board[0].len(), 0..board.len())
    } else {
        (0..board.len(), 0..board[0].len())
    };
    for rowi in r1{
        let mut map: HashMap<[u8; 4], usize> = map1.clone();
        for coli in r2.clone(){
            let cell = if transpose {
                board[coli][rowi]
            } else {
                board[rowi][coli]
            };
            apply_pattern_checking(&patterns, &mut map, cell, player, &mut result_map);
        }
    }
    // println!("{:?}", result_map);
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
    let mut rowi = 0;
    for colistart in (0..board[0].len()).rev(){
        let mut coli = colistart;
        let mut map: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
        while rowi < board.len() {
            apply_pattern_checking(&patterns, &mut map, board[rowi][coli], player, &mut result_map);
            rowi += 1;
            if coli == 0 {
                break;
            }
            coli -= 1;
        }
        rowi = 0;
    }
    let mut coli = board[0].len() - 1;
    for rowistart in 1..board.len(){
        let mut rowi = rowistart;
        let mut map: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
        while rowi < board.len() {
            apply_pattern_checking(&patterns, &mut map, board[rowi][coli], player, &mut result_map);
            rowi += 1;
            if coli == 0 {
                break;
            }
            coli -= 1;
        }
        coli = board[0].len() - 1;
    }
    // println!("{:?}", result_map);
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
    // let transposed = rot90(board);
    let horiz = pattern_counter(board, player, &patterns, false);
    let verti = pattern_counter(board, player, &patterns, true);
    let diag1 = pattern_counter_diagonal(&board, player, &patterns);
    // let diag2 = pattern_counter_diagonal(&transposed, player, &patterns);
    
    horiz + verti + diag1// + diag2
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
    for i in 0..7*7*7*7*7{
        evaluation_function(&board, 1);
        evaluation_function(&board, 2);
    }
    // println!("{}", evaluation_function(&board, 1));
    // println!("{}", evaluation_function(&board, 2));
    // println!("{}", pattern_counter_diagonal(&board, 1, &vec![Pattern{pattern: [1, 1, 0, 1], weight: 1.0}]));
    print!("DONE!")
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
