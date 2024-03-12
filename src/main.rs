
use egui::Slider;
use macroquad::prelude::*;

enum Move{
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

const LETTERS: &str = "ABCDEFGH";
const COLORS: [Color; 8] = [WHITE, BLUE, RED, GREEN, ORANGE, YELLOW, MAGENTA, GRAY];

#[macroquad::main("mq-torub")]
async fn main() {
    // TODO:
    // implement serialization for desktop version
    // optimize moves list (3-M-# to 1-'M-#; 4+#-M-# to #mod4-M-#; M-#, 'M-# to nothing;...)
    // transpozition of algos/movesets to different rows/columns
    // random discovery of algorithms
    // inverse version of the puzzle for both visualization as well solution-finding purposes

    // debugging
    // std::env::set_var("RUST_BACKTRACE", "1");

    let mut solved: bool = false;
    let mut moves: Vec<Move> = vec![];
    let mut size: u8 = 4;
    let mut matrix: Vec<Vec<(Option<Color>, char, u8)>> = generate_matrix(size);

    loop {
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .show(egui_ctx, |ui| {
                    if ui.add(Slider::new(&mut size, 2u8..=LETTERS.len() as u8)).changed() {
                        matrix = generate_matrix(size);
                        moves = vec![];
                        solved = false;
                    }
                    if ui.button("Reset [R]").clicked() {
                        matrix = generate_matrix(size);
                        moves = vec![];
                        solved = false;
                    }
                    ui.label(format!("Moves: {}", moves.len()));

                    if solved {
                        ui.separator();
                        ui.label("Please contact me on discord (username: GhtGhoster) with this solution:");
                        ui.text_edit_multiline(&mut moves_to_string(&moves));
                    }
                }
            );
        });

        // rendering
        clear_background(BLACK);
        let matrix_size = screen_height().min(screen_height());
        let node_size = matrix_size / (size as f32 + 2.0);
        let left_top_x = (screen_width()-matrix_size) / 2.0;
        let left_top_y = (screen_height()-matrix_size) / 2.0;
        for i in 0..size {
            for j in 0..size {
                let (color_option, character, number) = &matrix[i as usize][j as usize];
                if let Some(color) = color_option {
                    draw_text(
                        format!("{character}").as_str(),
                        left_top_x + (node_size * (j+1) as f32),
                        left_top_y + (node_size * (i+2) as f32),
                        node_size,
                        *color,
                    );
                } else {
                    draw_text(
                        format!("{character}{number}").as_str(),
                        left_top_x + (node_size * (j+1) as f32),
                        left_top_y + (node_size * (i+2) as f32),
                        node_size,
                        WHITE,
                    );
                }
            }
        }
        for i in 0..size {
            draw_text("<", left_top_x, left_top_y + (node_size * (i+2) as f32), node_size, WHITE);
            draw_text(">", left_top_x + ((size+1) as f32 * node_size), left_top_y + (node_size * (i+2) as f32), node_size, WHITE);
            draw_text("^", left_top_x + (node_size * (i+1) as f32), left_top_y + node_size, node_size, WHITE);
            draw_text("v", left_top_x + (node_size * (i+1) as f32), left_top_y + (node_size * (size + 2) as f32), node_size, WHITE);
        }


        // logic
        if is_key_pressed(KeyCode::R) {
            matrix = generate_matrix(size);
            moves = vec![];
            solved = false;
        }

        // moves
        if !solved {
            let mut button_clicked: bool = false;
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                let node_x = ((mouse_x - left_top_x) / node_size) as i32;
                let node_y = ((mouse_y - left_top_y) / node_size) as i32;

                //  move left
                if node_x == 0 && (1..=size as i32).contains(&node_y) {
                    let index = (node_y - 1) as usize;
                    let tmp = matrix[index].remove(0);
                    matrix[index].push(tmp);

                    moves.push(Move::Left(node_y as u8));
                    button_clicked = true;
                }

                // move right
                if node_x == (size+1) as i32 && (1..=size as i32).contains(&node_y) {
                    let index = (node_y - 1) as usize;
                    let tmp = matrix[index].pop().unwrap();
                    matrix[index].insert(0, tmp);

                    moves.push(Move::Right(node_y as u8));
                    button_clicked = true;
                }

                // move up
                if node_y == 0 && (1..=size as i32).contains(&node_x) {
                    let index = (node_x - 1) as usize;
                    let tmp = matrix[0].remove(index as usize);
                    for i in 1..size {
                        let lil_tmp = matrix[i as usize].remove(index);
                        matrix[(i-1) as usize].insert(index, lil_tmp);
                    }
                    matrix[(size-1) as usize].insert(index, tmp);

                    moves.push(Move::Up(node_x as u8));
                    button_clicked = true;
                }

                // move down
                if node_y == (size + 1) as i32 && (1..=size as i32).contains(&node_x) {
                    let index = (node_x - 1) as usize;
                    let tmp = matrix[(size-1) as usize].remove(index as usize);
                    for i in (0..size-1).rev() {
                        let lil_tmp = matrix[i as usize].remove(index);
                        matrix[(i+1) as usize].insert(index, lil_tmp);
                    }
                    matrix[0].insert(index, tmp);

                    moves.push(Move::Down(node_x as u8));
                    button_clicked = true;
                }
            }
            if button_clicked {
                if is_solved(size, &matrix) {
                    solved = true;
                }
            }
        }


        egui_macroquad::draw();
        next_frame().await
    }
}

fn generate_matrix(size: u8) -> Vec<Vec<(Option<Color>, char, u8)>>{
    let mut matrix: Vec<Vec<(Option<Color>, char, u8)>> = Vec::with_capacity(size as usize);
    for i in 0..size {
        let mut row: Vec<(Option<Color>, char, u8)> = Vec::with_capacity(size as usize);
        for j in 0..size {
            let character: char = LETTERS.chars().nth(j as usize).unwrap();
            let item: (Option<Color>, char, u8) = if (size as usize) <= COLORS.len() {
                (Some(COLORS[i as usize]), character, i)
            } else {
                (None, character, i)
            };
            row.push(item);
        }
        matrix.push(row);
    }
    matrix
}


/// Slow and naive interpretation of verifying the solution, not worth to optimize for these small sizes
/// Optimization could probably done in the form of 2 Hashsets for chars and u8s, then comparing their length to size
fn is_solved(size: u8, matrix: &Vec<Vec<(Option<Color>, char, u8)>>) -> bool {
    for (y, row) in matrix.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            // deconstruct item
            let (_, character, number) = item;

            // check row (iterate over columns)
            for i in 0..size as usize {
                if i == x {
                    // skip current item in the row
                    continue;
                } else {
                    // deconstruct compared item
                    let (_, character_comp, number_comp) = &matrix[y][i];

                    // check if either number or character is the same as item
                    if character_comp == character || number_comp == number {
                        return false;
                    }
                }
            }

            // check column (iterate over rows)
            for i in 0..size as usize {
                if i == y {
                    // skip current item in the row
                    continue;
                } else {
                    // deconstruct compared item
                    let (_, character_comp, number_comp) = &matrix[i][x];

                    // check if either number or character is the same as current item
                    if character_comp == character || number_comp == number {
                        return false;
                    }
                }
            }
        }
    }
    // no same number or character found for any of the matrix nodes in either rows or columns, must be solved
    true
}

fn moves_to_string(moves: &Vec<Move>) -> String {
    let mut string: String = String::new();

    for move_item in moves {
        match move_item {
            Move::Up(number) => {
                string += format!("U{number}").as_str();
            }
            Move::Left(number) => {
                string += format!("L{number}").as_str();
            }
            Move::Down(number) => {
                string += format!("D{number}").as_str();
            }
            Move::Right(number) => {
                string += format!("R{number}").as_str();
            }
        }
    }

    string
}