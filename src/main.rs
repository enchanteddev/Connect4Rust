use std::collections::HashMap;

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


fn count_horizontal(board: &Vec<Vec<u8>>, player: u8)  -> HashMap<u8, Vec<[u8; 4]>>{
    let mut map: HashMap<u8, Vec<[u8; 4]>> = HashMap::new();
    for (rowi, row) in board.iter().enumerate() {
        let mut count: u8 = 0;
        let mut start: i8 = -1;
        let mut end: i8 = -1;
        for (i, elem) in row.iter().enumerate() {
            if elem == &player {
                if start == -1 {start = i as i8;}
                count += 1;
            } else {
                if count == 0 {continue;}
                end = i as i8;
                let hdrop_start = height_drop(board, rowi as i8, start - 1);
                let hdrop_end = height_drop(board, rowi as i8, end);

                let count_data_option = map.get(&count);
                let mut count_data: Vec<[u8; 4]> = match count_data_option {
                    Some(x) => x.clone(),
                    None => vec![]
                };
                
                let array_map = if count == 3 {
                    [0, hdrop_start, hdrop_end, 0]
                } else if count == 2 {
                    let hdrop_start2 = height_drop(board, rowi as i8, start - 2);
                    let hdrop_end2 = height_drop(board, rowi as i8, end + 1);
                    [hdrop_start2, hdrop_start, hdrop_end, hdrop_end2]
                } else if count == 4 {
                    [0, 0, 0, 0]
                } else {
                    continue;
                };

                count_data.push(array_map);
                map.insert(count, count_data.clone());
                count = 0;
                start = -1;
                end = -1;
            }
        }
    }
    map
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


fn pattern_counter(board: &Vec<Vec<u8>>, player: u8, patterns: Vec<Pattern>) -> f64{
    let mut result_map: HashMap<[u8; 4], u8> = patterns.iter().map(|e| (e.pattern, 0)).collect();
    for rowi in 0..board.len(){
        let mut map: HashMap<[u8; 4], usize> = patterns.iter().map(|e| (e.pattern, 0)).collect();
        for coli in 0..board[0].len(){
            let mut marked: HashMap<[u8; 4], bool> = patterns.iter().map(|e| (e.pattern, false)).collect();
            for p in patterns.iter(){
                let curr_p_index = map.get(&p.pattern).unwrap_or(&0).to_owned();
                
                if (p.pattern[curr_p_index] != 0 && board[rowi][coli] == player) || 
                    (p.pattern[curr_p_index] == 0 && board[rowi][coli] == 0) {
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
    }
    println!("{:?}", result_map);
    let mut resultant_value = 0.0;
    for p in patterns {
        resultant_value += p.weight * result_map.get(&p.pattern).unwrap_or(&0).to_owned() as f64;
    }
    resultant_value
}


fn evaluation_function(board: Vec<Vec<u8>>, player: u8) -> i64{
    count_horizontal(&board, player)[&3][0][2] as i64
}


fn main() {
    let board: Vec<Vec<u8>> = vec![
        vec![1, 0, 0, 1, 1, 0, 1],
        vec![0, 0, 0, 2, 2, 0, 0],
        vec![0, 0, 2, 2, 2, 0, 0],
        vec![1, 0, 1, 1, 1, 0, 0],
        vec![2, 0, 2, 2, 1, 1, 1],
        vec![1, 2, 2, 1, 1, 1, 2]
    ];
    println!("{:?}", pattern_counter(&board, 1, vec![
            Pattern{pattern: [1, 1, 0, 1], weight: 1.0},
            Pattern{pattern: [1, 0, 0, 1], weight: 1.0},
            Pattern{pattern: [1, 1, 1, 1], weight: 1.0},
            Pattern{pattern: [1, 0, 1, 1], weight: 1.0}
        ]));
    println!("{:?}", pattern_counter(&transpose(&board), 1, vec![
            Pattern{pattern: [1, 1, 0, 1], weight: 1.0},
            Pattern{pattern: [1, 0, 0, 1], weight: 1.0},
            Pattern{pattern: [1, 1, 1, 1], weight: 1.0},
            Pattern{pattern: [1, 0, 1, 1], weight: 1.0}
        ]));
}
