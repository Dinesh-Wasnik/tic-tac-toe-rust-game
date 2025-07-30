use eframe::{egui, App, Frame};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Clone, Copy, Default, Debug)]
enum Player {
    #[default]
    X,
    O,
}

#[derive(Default)]
struct GameState {
    board: [[Option<Player>; 3]; 3],
    current_turn: Player,
    winner: Option<Player>,
    game_over: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current_turn: Player::X,
            winner: None,
            game_over: false,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn make_move(&mut self, row: usize, col: usize) {
        if self.game_over || self.board[row][col].is_some() {
            return;
        }

        self.board[row][col] = Some(self.current_turn);

        if let Some(winner) = self.check_winner() {
            self.winner = Some(winner);
            self.game_over = true;
        } else if self.is_draw() {
            self.game_over = true;
        } else {
            self.switch_turn();
            if self.current_turn == Player::O {
                self.ai_move();
            }
        }
    }

    fn ai_move(&mut self) {
        let mut empty_cells = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j].is_none() {
                    empty_cells.push((i, j));
                }
            }
        }

        if let Some(&(i, j)) = empty_cells.choose(&mut thread_rng()) {
            self.make_move(i, j);
        }
    }

    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn check_winner(&self) -> Option<Player> {
        let lines = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for line in lines.iter() {
            let [a, b, c] = *line;
            if let (Some(p1), Some(p2), Some(p3)) = (
                self.board[a.0][a.1],
                self.board[b.0][b.1],
                self.board[c.0][c.1],
            ) {
                if p1 == p2 && p2 == p3 {
                    return Some(p1);
                }
            }
        }

        None
    }

    fn is_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    fn cell_to_string(cell: Option<Player>) -> &'static str {
        match cell {
            Some(Player::X) => "X",
            Some(Player::O) => "O",
            None => " ",
        }
    }
}

#[derive(Default)]
struct TicTacToeApp {
    game_state: GameState,
}

impl App for TicTacToeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tic Tac Toe (vs AI)");
            ui.add_space(10.0);

            for i in 0..3 {
                ui.horizontal(|ui| {
                    for j in 0..3 {
                        let label = GameState::cell_to_string(self.game_state.board[i][j]);
                        if ui
                            .add_sized([60.0, 60.0], egui::Button::new(label))
                            .clicked()
                        {
                            self.game_state.make_move(i, j);
                        }
                    }
                });
            }

            ui.add_space(10.0);
            if self.game_state.game_over {
                if let Some(winner) = self.game_state.winner {
                    ui.label(format!("Player {:?} wins!", winner));
                } else {
                    ui.label("It's a draw!");
                }

                if ui.button("Play Again").clicked() {
                    self.game_state.reset();
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic Tac Toe (Rust + egui)",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    )
}
