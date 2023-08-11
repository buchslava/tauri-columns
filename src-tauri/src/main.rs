// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(non_snake_case)]
use std::collections::HashMap;
use phf::{ phf_map };

const WIDTH: usize = 7;
const HEIGHT: usize = 21;

static w: &str = "#ffffff";

static COLOR_INACTIVE_RED: &str = "#f1867e";
static COLOR_INACTIVE_GREEN: &str = "#93ad93";
static COLOR_INACTIVE_LIME: &str = "#e5ffc7";
static COLOR_INACTIVE_YELLOW: &str = "#fff9be";
static COLOR_INACTIVE_BROWN: &str = "#dab7aa";
static COLOR_INACTIVE_BLUE: &str = "#868eba";
static COLOR_INACTIVE_LIGHT_BLUE: &str = "#96c7ee";
static COLOR_INACTIVE_TEAL: &str = "#8c9696";
static COLOR_INACTIVE_FUCHSIA: &str = "#ffb4ff";

// static COLORS_TO_DISAPPEAR_HASH: phf::Map<
//     &'static str,
//     &'static str
// > = phf_map! {
//   "#f34336" => COLOR_INACTIVE_RED,
//   "#4db14f" => COLOR_INACTIVE_GREEN,
//   "#b3ff5a" => COLOR_INACTIVE_LIME,
//   "#feed3d" => COLOR_INACTIVE_YELLOW,
//   "#795547" => COLOR_INACTIVE_BROWN,
//   "#3e50b4" => COLOR_INACTIVE_BLUE,
//   "#2196f3" => COLOR_INACTIVE_LIGHT_BLUE,
//   "#008080" => COLOR_INACTIVE_TEAL,
//   "#ff00ff" => COLOR_INACTIVE_FUCHSIA,
// };

fn colorsToDisappear(color: String) -> String {
    if color == "#f34336" {
        return COLOR_INACTIVE_RED.to_string();
    }
    if color == "#4db14f" {
        return COLOR_INACTIVE_GREEN.to_string();
    }
    if color == "#b3ff5a" {
        return COLOR_INACTIVE_LIME.to_string();
    }
    if color == "#feed3d" {
        return COLOR_INACTIVE_YELLOW.to_string();
    }
    if color == "#795547" {
        return COLOR_INACTIVE_BROWN.to_string();
    }
    if color == "#3e50b4" {
        return COLOR_INACTIVE_BLUE.to_string();
    }
    if color == "#2196f3" {
        return COLOR_INACTIVE_LIGHT_BLUE.to_string();
    }
    if color == "#008080" {
        return COLOR_INACTIVE_TEAL.to_string();
    }
    if color == "#ff00ff" {
        return COLOR_INACTIVE_FUCHSIA.to_string();
    }
    return "".to_string();
}

fn shouldDisappear(color: String) -> bool {
    let cl = colorsToDisappear(color);
    return cl != "";
}

// ---
// #[derive(Clone, serde::Serialize, serde::Deserialize)]
// pub struct Foobar {
//     name: String,
//     val: i16,
//     board: [[String;WIDTH];HEIGHT],
// }

// #[tauri::command]
// async fn tst(foo: Foobar) -> Foobar {
//   let mut result = foo.clone();
//   result.name = "Processed".to_owned() + &foo.name;
//   result.board[0][0] = "First".to_owned();

//   for (i, row) in result.board.iter().enumerate() {
//     for (j, col) in row.iter().enumerate() {
//         println!("[row={}][col={}]={}", i, j, col);
//     }
//   }

//   return result;
// }
// ---

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Actor {
    state: [String; 3],
    column: usize,
    row: usize,
}

pub type BoardType = [[String; WIDTH as usize]; HEIGHT as usize];
pub type MatchingType = [[bool; WIDTH as usize]; HEIGHT as usize];

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    board: BoardType,
    actor: Actor,
    phase: u8,
    savedPhase: u8,
    nextActor: [String; 3],
    score: u32,
    // scores: { [key: string]: number };
}

// ...

fn verticalMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();
    let mut row: usize = 0;
    
    while row < HEIGHT {
        let mut matchStartIndex: i8 = -1;

        let mut col: usize = 0;
        while col < WIDTH {
            let mut isMatch = false;
            if
                (col as i8) - 1 >= 0 &&
                board[row][col] == board[row][col - 1] &&
                board[row][col] != w &&
                !shouldDisappear(board[row][col].to_string())
            {
                isMatch = true;
            }
            if isMatch == true {
                if matchStartIndex == -1 {
                    matchStartIndex = (col - 1) as i8;
                } else if col == columnsQty - 1 && col - (matchStartIndex as usize) >= 2 {
                    for c in matchStartIndex as usize..col + 1 {
                        ma[row][c] = true;
                    }
                }
            } else {
                if (matchStartIndex as usize) >= 0 && (col as i8) - matchStartIndex > 2 {
                    for c in matchStartIndex as usize..col {
                        ma[row][c] = true;
                    }
                }
                matchStartIndex = -1;
            }
            col = col + 1;
        }
        row = row + 1;
    }
}

fn horizontalMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    let mut row = 0;
    while row < HEIGHT {
        let mut matchStartIndex: i8 = -1;

        let mut col = 0;
        while col < WIDTH {
            let mut isMatch = false;
            if
                (row as i8) - 1 >= 0 &&
                board[row][col] == board[row - 1][col] &&
                board[row][col] != w &&
                !shouldDisappear(board[row][col].to_string())
            {
                isMatch = true;
            }
            if isMatch == true {
                if matchStartIndex == -1 {
                    matchStartIndex = (row - 1) as i8;
                } else if row == rowsQty - 1 && row - (matchStartIndex as usize) >= 2 {
                    for r in matchStartIndex as usize..row + 1 {
                        ma[r][col] = true;
                    }
                }
            } else {
                if matchStartIndex >= 0 && row - (matchStartIndex as usize) > 2 {
                    for r in matchStartIndex as usize..row {
                        ma[r][col] = true;
                    }
                }
                matchStartIndex = -1;
            }
            col = col + 1;
        }
        row = row + 1;
    }
    // println!("{:?}", ma);
}

fn diagonalMatch(board: &BoardType, rowDirect: i8, row: usize, colDirect: i8, col: usize) -> bool {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    if
        row >= 0 &&
        row < rowsQty &&
        col >= 0 &&
        col < columnsQty &&
        (row as i8) + rowDirect >= 0 &&
        (row as i8) + rowDirect < (rowsQty as i8) &&
        (col as i8) + colDirect >= 0 &&
        (col as i8) + colDirect < (columnsQty as i8)
    {
        return false;
    }
    return (
        board[row][col] == board[row + (rowDirect as usize)][col + (colDirect as usize)] &&
        board[row][col] != w &&
        !shouldDisappear(board[row][col].to_string())
    );
}

fn diagonalColumnRightToLeftMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();
    let mut xCol = columnsQty - 1;

    while xCol > 1 {
        let mut startRow: i8 = -1;
        let mut startCol: i8 = -1;
        let mut row: i8 = 1;
        let mut col: i8 = (xCol as i8) - 1;

        while col >= 0 {
            if diagonalMatch(&board, -1, row as usize, 1, col as usize) == true {
                if startRow == -1 && startCol == -1 {
                    startRow = (row as i8) - 1;
                    startCol = (col as i8) + 1;
                } else if col == 0 && row - startRow >= 2 {
                    let mut r = startRow as usize;
                    let mut c = startCol as usize;
                    while r <= row.try_into().unwrap() {
                        ma[r][c] = true;
                        r = r + 1;
                        c = c - 1;
                    }
                }
            } else {
                if startRow >= 0 && startCol >= 0 && row - startRow > 2 {
                    let mut r = startRow;
                    let mut c = startCol;
                    while r < row {
                        ma[r as usize][c as usize] = true;
                        r = r + 1;
                        c = c - 1;
                    }
                }
                startRow = -1;
                startCol = -1;
            }
            col = col - 1;
            row = row + 1;
        }
        xCol = xCol - 1;
    }
}

fn diagonalRowRightToLeftMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();
    let mut xRow = 0;

    while xRow <= rowsQty {
        let mut startRow: i8 = -1;
        let mut startCol: i8 = -1;
        let mut row: i8 = (xRow as i8) + 1;
        let mut col: i8 = (columnsQty as i8) - 2;

        while row < (rowsQty as i8) && col >= 0 {
            if diagonalMatch(&board, -1, row as usize, 1, col as usize) == true {
                if startRow == -1 && startCol == -1 {
                    startRow = (row as i8) - 1;
                    startCol = (col as i8) + 1;
                } else if (row == (rowsQty as i8) - 1 || col == 0) && row - startRow >= 2 {
                    let mut r = startRow;
                    let mut c = startCol;
                    while r <= row.try_into().unwrap() {
                        ma[r as usize][c as usize] = true;
                        r = r + 1;
                        c = c - 1;
                    }
                }
            } else {
                if startRow >= 0 && startCol >= 0 && row - startRow > 2 {
                    let mut r = startRow;
                    let mut c = startCol;
                    while r < row.try_into().unwrap() {
                        ma[r as usize][c as usize] = true;
                        r = r + 1;
                        c = c - 1;
                    }
                }
                startRow = -1;
                startCol = -1;
            }
            row = row + 1;
            col = col - 1;
        }
        xRow = xRow + 1;
    }
}

fn diagonalColumnLeftToRightMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();
    let mut xCol = 0;

    while xCol <= columnsQty {
        let mut startRow: i8 = -1;
        let mut startCol: i8 = -1;
        let mut col = xCol + 1;
        let mut row = 1;

        while col < columnsQty && row < rowsQty {
            if diagonalMatch(&board, -1, row as usize, -1, col as usize) == true {
                if startRow == -1 && startCol == -1 {
                    startRow = (row as i8) - 1;
                    startCol = (col as i8) - 1;
                } else if
                    (col == columnsQty - 1 || row == rowsQty - 1) &&
                    row - (startRow as usize) >= 2
                {
                    let mut r = startRow;
                    let mut c = startCol;

                    while r <= row.try_into().unwrap() {
                        ma[r as usize][c as usize] = true;
                        r = r + 1;
                        c = c + 1;
                    }
                }
            } else {
                if startRow >= 0 && startCol >= 0 && row - (startRow as usize) > 2 {
                    let mut r = startRow;
                    let mut c = startCol;
                    while r < row.try_into().unwrap() {
                        ma[r as usize][c as usize] = true;
                        r = r + 1;
                        c = c + 1;
                    }
                }
                startRow = -1;
                startCol = -1;
            }
            col = col + 1;
            row = row + 1;
        }
        xCol = xCol + 1;
    }
}

fn diagonalRowLeftToRightMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();
    let mut xRow = 0;

    while xRow < rowsQty {
        let mut startRow: i8 = -1;
        let mut startCol: i8 = -1;
        let mut row = xRow + 1;
        let mut col = 1;

        while row < rowsQty && col < columnsQty {
            if diagonalMatch(&board, -1, row as usize, -1, col as usize) == true {
                if startRow == -1 && startCol == -1 {
                    startRow = (row as i8) - 1;
                    startCol = (col as i8) - 1;
                } else if row == rowsQty - 1 || col == columnsQty - 1 {
                    if row - (startRow as usize) >= 2 {
                        let mut r = startRow;
                        let mut c = startCol;
                        while r <= row.try_into().unwrap() {
                            ma[r as usize][c as usize] = true;
                            r = r + 1;
                            c = c + 1;
                        }
                    }
                }
            } else {
                if startRow >= 0 && startCol >= 0 && row - (startRow as usize) > 2 {
                    let mut r = startRow;
                    let mut c = startCol;
                    while r < row.try_into().unwrap() {
                        ma[r as usize][c as usize] = true;
                        r = r + 1;
                        c = c + 1;
                    }
                }
                startRow = -1;
                startCol = -1;
            }
            row = row + 1;
            col = col + 1;
        }
        xRow = xRow + 1;
    }
}

fn squareMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    for row in 1..rowsQty {
        for col in 1..columnsQty {
            if
                board[row][col - 1] == board[row][col] &&
                board[row - 1][col - 1] == board[row][col] &&
                board[row - 1][col] == board[row][col] &&
                board[row][col] != w &&
                !shouldDisappear(board[row][col].to_string())
            {
                ma[row][col] = true;
                ma[row][col - 1] = true;
                ma[row - 1][col - 1] = true;
                ma[row - 1][col] = true;
            }
        }
    }
}

fn crossSquareMatching(board: &BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    for row in 1..rowsQty {
        for col in 1..columnsQty {
            if
                board[row][col] == board[row - 1][col - 1] &&
                board[row][col - 1] == board[row - 1][col] &&
                board[row][col] != w &&
                !shouldDisappear(board[row][col].to_string())
            {
                ma[row][col] = true;
                ma[row][col - 1] = true;
                ma[row - 1][col - 1] = true;
                ma[row - 1][col] = true;
            }
        }
    }
}

fn checkCollapsed(board: &mut BoardType, ma: &mut MatchingType, mark: bool) -> bool {
    let columnsQty = board[0].len();
    let rowsQty = board.len();
    let mut result = false;

    for row in 1..rowsQty {
        for col in 1..columnsQty {
            if ma[row][col] == true {
                let cl = colorsToDisappear(board[row][col].to_string());
                if mark {
                    board[row][col] = cl;
                    // game.score += game.scores[level];
                }
                result = true;
            }
        }
    }
    return result;
}

#[tauri::command]
fn matching(game: Game, level: String, mark: bool, customBoard: Option<BoardType>) -> bool {
    let mut board: BoardType = customBoard.unwrap_or(game.board);
    let mut ma: MatchingType = [[false; WIDTH as usize]; HEIGHT as usize];

    verticalMatching(&board, &mut ma);
    horizontalMatching(&board, &mut ma);
    diagonalColumnRightToLeftMatching(&board, &mut ma);
    diagonalRowRightToLeftMatching(&board, &mut ma);
    diagonalColumnLeftToRightMatching(&board, &mut ma);
    diagonalRowLeftToRightMatching(&board, &mut ma);
    squareMatching(&board, &mut ma);
    crossSquareMatching(&board, &mut ma);

    return checkCollapsed(&mut board, &mut ma, mark);
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![matching])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
