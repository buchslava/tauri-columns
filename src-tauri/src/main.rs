// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(non_snake_case)]
use std::collections::HashMap;

const WIDTH: usize = 8;
const HEIGHT: usize = 21;

static w: &str = "#ffffff";

static COLOR_RED: &str = "#f34336";
static COLOR_GREEN: &str = "#4db14f";
static COLOR_LIME: &str = "#b3ff5a";
static COLOR_YELLOW: &str = "#feed3d";
static COLOR_BROWN: &str = "#795547";
static COLOR_BLUE: &str = "#3e50b4";
static COLOR_LIGHT_BLUE: &str = "#2196f3";
static COLOR_TEAL: &str = "#008080";
static COLOR_FUCHSIA: &str = "#ff00ff";

static COLOR_INACTIVE_RED: &str = "#f1867e";
static COLOR_INACTIVE_GREEN: &str = "#93ad93";
static COLOR_INACTIVE_LIME: &str = "#e5ffc7";
static COLOR_INACTIVE_YELLOW: &str = "#fff9be";
static COLOR_INACTIVE_BROWN: &str = "#dab7aa";
static COLOR_INACTIVE_BLUE: &str = "#868eba";
static COLOR_INACTIVE_LIGHT_BLUE: &str = "#96c7ee";
static COLOR_INACTIVE_TEAL: &str = "#8c9696";
static COLOR_INACTIVE_FUCHSIA: &str = "#ffb4ff";

fn shouldDisappear(color: String) -> bool {
    let mut colorsToDisappearHash: HashMap<String, String> = HashMap::new();

    colorsToDisappearHash.insert(COLOR_RED.to_string(), COLOR_INACTIVE_RED.to_string());
    colorsToDisappearHash.insert(COLOR_GREEN.to_string(), COLOR_INACTIVE_GREEN.to_string());
    colorsToDisappearHash.insert(COLOR_LIME.to_string(), COLOR_INACTIVE_LIME.to_string());
    colorsToDisappearHash.insert(COLOR_YELLOW.to_string(), COLOR_INACTIVE_YELLOW.to_string());
    colorsToDisappearHash.insert(COLOR_BROWN.to_string(), COLOR_INACTIVE_BROWN.to_string());
    colorsToDisappearHash.insert(COLOR_BLUE.to_string(), COLOR_INACTIVE_BLUE.to_string());
    colorsToDisappearHash.insert(
        COLOR_LIGHT_BLUE.to_string(),
        COLOR_INACTIVE_LIGHT_BLUE.to_string()
    );
    colorsToDisappearHash.insert(COLOR_TEAL.to_string(), COLOR_INACTIVE_TEAL.to_string());
    colorsToDisappearHash.insert(COLOR_FUCHSIA.to_string(), COLOR_INACTIVE_FUCHSIA.to_string());

    match colorsToDisappearHash.get(&color) {
        Some(value) => {
            return true;
        }
        _ => {
            return false;
        }
    }
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
    phase: String,
    savedPhase: String,
    nextActor: [String; 3],
    score: u32,
    // scores: { [key: string]: number };
}

// ...

fn verticalMatching(board: BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    let mut row: usize = 0;
    while row < HEIGHT {
        let mut matchStartIndex: i8 = -1;

        let mut col: usize = 0;
        while col < WIDTH {
            let mut isMatch = false;
            if
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
                if (matchStartIndex as usize) >= 0 && col - (matchStartIndex as usize) > 2 {
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

fn horizontalMatching(board: BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    let mut row = 0;
    while row < HEIGHT {
        let mut matchStartIndex: i8 = -1;

        let mut col = 0;
        while col < WIDTH {
            let mut isMatch = false;
            if
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
}

fn diagonalMatch(board: &BoardType, rowDirect: i8, row: usize, colDirect: i8, col: usize) -> bool {
    return (
        board[row][col] == board[row + (rowDirect as usize)][col + (colDirect as usize)] &&
        board[row][col] != w &&
        !shouldDisappear(board[row][col].to_string())
    );
}

fn diagonalColumnRightToLeftMatching(board: BoardType, ma: &mut MatchingType) {
    let columnsQty = board[0].len();
    let rowsQty = board.len();

    let mut xCol = columnsQty - 1;
    while xCol > 1 {
        let mut startRow: i8 = -1;
        let mut startCol: i8 = -1;
        let mut row: i8 = 1;

        let mut col = xCol - 1;
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

fn matching(game: Game, level: String, mark: bool, customBoard: Option<BoardType>) -> bool {
    let mut board: BoardType = customBoard.unwrap_or(game.board);
    let mut ma: MatchingType = [[false; WIDTH as usize]; HEIGHT as usize];

    verticalMatching(board.clone(), &mut ma);
    horizontalMatching(board.clone(), &mut ma);
    diagonalColumnRightToLeftMatching(board.clone(), &mut ma);

    return false;
}

fn main() {
    tauri::Builder
        ::default()
        // .invoke_handler(tauri::generate_handler![tst])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
