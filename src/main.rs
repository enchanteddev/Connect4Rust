use std::{collections::HashMap, f64::INFINITY, vec};

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
    let map1: Vec<usize> = vec![0; patterns.len()];
    let (r1, r2) = if transpose {
        (0..board[0].len(), 0..board.len())
    } else {
        (0..board.len(), 0..board[0].len())
    };
    for rowi in r1{
        let mut map: Vec<usize> = map1.clone();
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


fn apply_pattern_checking(patterns: &Vec<Pattern>, map: &mut Vec<usize>, cell_value: u8, player: u8, result_map: &mut HashMap<[u8; 4], u8>) {
    // let mut marked: HashMap<[u8; 4], bool> = patterns.iter().map(|e| (e.pattern, false)).collect();
    let mut marked = vec![false; patterns.len()];
    for (i, p) in patterns.iter().enumerate(){
        // let curr_p_index = map.get(&p.pattern).unwrap_or(&0).to_owned();
        let curr_p_index = map[i].to_owned();
        if (p.pattern[curr_p_index] != 0 && cell_value == player) || 
            (p.pattern[curr_p_index] == 0 && cell_value == 0) {
            // marked.insert(p.pattern, true);
            marked[i] = true;

            if curr_p_index + 1 == 4 {
                result_map.insert(p.pattern, result_map.get(&p.pattern).unwrap_or(&0) + 1);
                // map.insert(p.pattern, 0);
                map[i] = 0;
            } else {
                map[i] = curr_p_index + 1;
            }
        }
    }
    for (arr, val) in marked.iter().enumerate(){
        if !val {
            // map.insert(patterns[arr].pattern, 0);
            map[arr] = 0;
        }
    }
}


fn pattern_counter_diagonal(board: &Vec<Vec<u8>>, player: u8, patterns: &Vec<Pattern>) -> f64{
    let mut result_map: HashMap<[u8; 4], u8> = patterns.iter().map(|e| (e.pattern, 0)).collect();

    let mut rowi = 0;
    for colistart in 0..board[0].len(){
        let mut coli = colistart;
        let mut map: Vec<usize> = vec![0; patterns.len()];
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
        let mut map: Vec<usize> = vec![0; patterns.len()];
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
        let mut map: Vec<usize> = vec![0; patterns.len()];
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
        let mut map: Vec<usize> = vec![0; patterns.len()];
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


fn evfn(board: &Vec<Vec<u8>>, player: u8) -> f64{
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

        Pattern{pattern: [1, 1, 1, 1], weight: 1000000.0}
    ];
    // let transposed = rot90(board);
    let horiz = pattern_counter(board, player, &patterns, false);
    let verti = pattern_counter(board, player, &patterns, true);
    let diag1 = pattern_counter_diagonal(&board, player, &patterns);
    // let diag2 = pattern_counter_diagonal(&transposed, player, &patterns);
    
    horiz + verti + diag1// + diag2
}


fn evaluation_function(board: &Vec<Vec<u8>>, player: u8, enemy_factor: f64) -> f64{
    evfn(board, player) - enemy_factor * evfn(board,  if player == 1 {2} else {1})
}


fn expand_board(board: &Vec<Vec<u8>>, player: u8) -> Vec<(Vec<Vec<u8>>, i8)> {
    let mut next_moves: Vec<(Vec<Vec<u8>>, i8)> = vec![];
    for moves in 0..board[0].len(){
        if board[0][moves] != 0 {continue;}
        let mut pushed = false;
        for r in 1..board.len(){
            if board[r][moves] != 0 {
                let mut new_board = board.clone();
                new_board[r - 1][moves] = player;
                next_moves.push((new_board, moves as i8));
                pushed = true;
                break;
            }
        }
        if !pushed {
            let mut new_board = board.clone();
            new_board[board.len() - 1][moves] = player;
            next_moves.push((new_board, moves as i8));
        }
    }
    next_moves
}

static EF: f64 = 2.0;

fn is_terminal(board: &Vec<Vec<u8>>, player: u8) -> bool{
    let patterns = vec![
        Pattern{pattern: [1, 1, 1, 1], weight: 1.0}
    ];
    let horiz = pattern_counter(board, player, &patterns, false);
    let verti = pattern_counter(board, player, &patterns, true);
    let diag1 = pattern_counter_diagonal(&board, player, &patterns);

    horiz == 1.0 || verti == 1.0 || diag1 == 1.0
}

pub fn get_max_move(board: &Vec<Vec<u8>>, level: i32, player: u8) -> (f64, i8){
    let next_states = expand_board(board, player);
    // println!("{:?}", next_states);
    if next_states.len() == 0{
        return (evaluation_function(board, player, EF), -1);
    }
    if level == 0 {
        return next_states.iter().enumerate().map(|(i, s)| (evaluation_function(&s.0, player, EF), i as i8)).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap();
    }
    let mut currmax = -INFINITY;
    let mut maxmove = -1;
    for (state, movei) in next_states.iter(){
        if is_terminal(state, player) {
            return (evaluation_function(state, player, EF), *movei);
        }
    }
    for (state, movei) in next_states.iter() {
        let (minef, _) = get_min_move(&state, level - 1, player, currmax);
        if minef > currmax {
            currmax = minef;
            maxmove = *movei;
        }
    }
    (currmax, maxmove)
}


fn get_min_move(board: &Vec<Vec<u8>>, level: i32, player: u8, currmax: f64) -> (f64, i8){
    let enemy = if player == 1 {2} else {1};
    let next_states = expand_board(board, enemy);
    if next_states.len() == 0{
        return (evaluation_function(board, player, EF), -1);
    }
    if level == 0 {
        return next_states.iter().enumerate().map(|(i, s)| (evaluation_function(&s.0, player, EF), i as i8)).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap();
    }
    let mut currmin = INFINITY;
    let mut minmove = -1;
    for (state, movei) in next_states {
        let (maxef, _) = get_max_move(&state, level - 1, player);
        if maxef < currmin {
            currmin = maxef;
            minmove = movei as i8;
        }
        if maxef < currmax {
            return  (currmin, movei as  i8);
        }
    }
    (currmin, minmove)
}

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn main(){
    read!(player as u8);
    let mut board: Vec<Vec<u8>> = vec![];
    for i in 0..6{
        read_vec!(r1 as u8);
        board.push(r1);
    }
    println!("{:?}", get_max_move(&board, 6, player).1);
}

fn main2() {
    let board: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
    ];
    // let board: Vec<Vec<u8>> = vec![
    //     vec![1, 0, 0, 1, 1, 0, 1],
    //     vec![1, 0, 0, 2, 1, 0, 0],
    //     vec![0, 0, 1, 2, 2, 0, 0],
    //     vec![1, 0, 1, 1, 1, 0, 1],
    //     vec![2, 0, 2, 1, 1, 1, 1],
    //     vec![1, 2, 2, 1, 1, 1, 2]
    // ];
    // for i in 0..7*7*7*7*7{
    //     evfn(&board, 1);
    //     evfn(&board, 2);
    // }
    // println!("{}", evaluation_function(&board, 1));
    // println!("{}", evaluation_function(&board, 2));
    // println!("{}", pattern_counter_diagonal(&board, 1, &vec![Pattern{pattern: [1, 1, 0, 1], weight: 1.0}]));
    println!("{:?}", get_max_move(&board, 5, 1));
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
