use std::collections::HashMap;

fn column_height(board: [[u8; 7]; 6], column: i8) -> Option<u8>{
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

fn height_drop(board: [[u8; 7]; 6], row: i8, column: i8) -> u8{
    let start_height = column_height(board, column).unwrap_or(6 as u8);
    let rowi = 6 - row;
    if start_height < rowi as u8 {
        rowi as u8 - start_height
    } else {0}
}


fn count_horizontal(board: [[u8; 7]; 6], player: u8)  -> HashMap<u8, Vec<[u8; 4]>>{
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


fn evaluation_function(board: [[u8; 7]; 6], player: u8) -> i64{
    count_horizontal(board, player)[&3][0][2] as i64
}


fn main() {
    let board: [[u8; 7]; 6] = [
        [0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 2, 2, 0, 0],
        [0, 0, 2, 2, 2, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [2, 0, 2, 2, 1, 1, 1],
        [1, 2, 2, 1, 1, 1, 2]
    ];
    println!("{:?}", count_horizontal(board, 1));
}
