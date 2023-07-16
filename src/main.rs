use axum::{routing::post, Router, http, Json};
use serde::{Deserialize, Serialize};

const SIZE: usize = 9;

#[derive(Serialize, Deserialize)]
struct Sudoku {
    board: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;
        let mut is_empty = false;

        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    row = i;
                    col = j;
                    is_empty = true;
                    break;
                }
            }
            if is_empty {
                break;
            }
        }

        if !is_empty {
            return true;
        }

        for num in 1..=SIZE {
            if self.is_safe(row, col, num as u8) {
                self.board[row][col] = num as u8;
                if self.solve() {
                    return true;
                }
                self.board[row][col] = 0;
            }
        }

        false
    }

    fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..SIZE {
            if self.board[row][i] == num {
                return false;
            }
        }

        for i in 0..SIZE {
            if self.board[i][col] == num {
                return false;
            }
        }

        let start_row = row - row % 3;
        let start_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }

        true
    }
}

// fn solve_sudoku(sudoku: &mut Sudoku) -> bool {
//     let mut row = 0;
//     let mut col = 0;
//     let mut is_empty = false;
//
//     for i in 0..SIZE {
//         for j in 0..SIZE {
//             if sudoku.board[i][j] == 0 {
//                 row = i;
//                 col = j;
//                 is_empty = true;
//                 break;
//             }
//         }
//         if is_empty {
//             break;
//         }
//     }
//
//     if !is_empty {
//         return true;
//     }
//
//     for num in 1..=SIZE {
//         if is_safe(sudoku, row, col, num as u8) {
//             sudoku.board[row][col] = num as u8;
//             if solve_sudoku(sudoku) {
//                 return true;
//             }
//             sudoku.board[row][col] = 0;
//         }
//     }
//
//     false
// }
//
// fn is_safe(sudoku: &Sudoku, row: usize, col: usize, num: u8) -> bool {
//     for i in 0..SIZE {
//         if sudoku.board[row][i] == num {
//             return false;
//         }
//     }
//
//     for i in 0..SIZE {
//         if sudoku.board[i][col] == num {
//             return false;
//         }
//     }
//
//     let start_row = row - row % 3;
//     let start_col = col - col % 3;
//
//     for i in 0..3 {
//         for j in 0..3 {
//             if self.board[i + start_row][j + start_col] == num {
//                 return false;
//             }
//         }
//     }
//
//     true
// }

async fn solve(Json(mut sudoku): Json<Sudoku>) -> Result<Json<Sudoku>, http::StatusCode> {
    if sudoku.solve() {
        Ok(Json(sudoku))
    } else {
        Err(http::StatusCode::BAD_REQUEST)
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/solve", post(solve));
    Ok(router.into())
}
