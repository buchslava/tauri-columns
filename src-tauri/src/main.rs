// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
  colorsToDisappearHash.insert(COLOR_LIGHT_BLUE.to_string(), COLOR_INACTIVE_LIGHT_BLUE.to_string());
  colorsToDisappearHash.insert(COLOR_TEAL.to_string(), COLOR_INACTIVE_TEAL.to_string());
  colorsToDisappearHash.insert(COLOR_FUCHSIA.to_string(), COLOR_INACTIVE_FUCHSIA.to_string());

  match colorsToDisappearHash.get(&color) {
    Some(value) => return true,
    _ => return false,
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
    state: [String;3],
    column: u16,
    row: u16,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
  board: [[String;WIDTH];HEIGHT],
  actor: Actor,
  phase: String,
  savedPhase: String,
  nextActor: [String;3],
  score: u32,
  // scores: { [key: string]: number };
}

// ...

fn verticalMatching(board: [[String;WIDTH];HEIGHT], ma: &mut [[bool;WIDTH];HEIGHT]) {
  let columnsQty = board[0].len();
  let rowsQty = board.len();

  let mut row = 0;
  while row < HEIGHT {
    let mut matchStartIndex: usize = usize::MAX;

    let mut col = 0;
    while col < WIDTH {
      let mut isMatch = false;
      if board[row][col] == board[row][col - 1] && board[row][col] != w && !shouldDisappear(board[row][col].to_string()) {
        isMatch = true;
      }
      if isMatch == true {
        if matchStartIndex == usize::MAX {
          matchStartIndex = col - 1;
        } else if col == columnsQty - 1 && col - matchStartIndex >= 2 {
          for c in matchStartIndex..col + 1 {
            ma[row][c] = true;
          }
        }
      } else {
        if matchStartIndex >= 0 && col - matchStartIndex > 2 {
          for c in matchStartIndex..col {
            ma[row][c] = true;
          }
        }
        matchStartIndex = usize::MAX;
      }
      col = col + 1;
    }
    row = row + 1;
  }
}

fn horizontalMatching(board: [[String;WIDTH];HEIGHT], ma: &mut [[bool;WIDTH];HEIGHT]) {
  let columnsQty = board[0].len();
  let rowsQty = board.len();

  let mut row = 0;
  while row < HEIGHT {
    let mut matchStartIndex: usize = usize::MAX;

    let mut col = 0;
    while col < WIDTH {
      let mut isMatch = false;
      if board[row][col] == board[row - 1][col] && board[row][col] != w && !shouldDisappear(board[row][col].to_string()) {
        isMatch = true;
      }
      if isMatch == true {
        if matchStartIndex == usize::MAX {
          matchStartIndex = row - 1;
        } else if row == rowsQty - 1 && row - matchStartIndex >= 2 {
          for r in matchStartIndex..row + 1 {
            ma[r][col] = true;
          }
        }
      } else {
        if matchStartIndex >= 0 && row - matchStartIndex > 2 {
          for r in matchStartIndex..row {
            ma[r][col] = true;
          }
        }
        matchStartIndex = usize::MAX;
      }
      col = col + 1;
    }
    row = row + 1;
  }
}

fn matching(game: Game, level: String, mark: bool, customBoard: Option<[[String;WIDTH];HEIGHT]>) -> bool {
  let mut board: [[String;WIDTH];HEIGHT] = customBoard.unwrap_or(game.board);
  let mut ma: [[bool;WIDTH];HEIGHT] = [[false;WIDTH];HEIGHT];

  verticalMatching(board.clone(), &mut ma);
  horizontalMatching(board.clone(), &mut ma);

  return false;
}




fn main() {
  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![tst])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
