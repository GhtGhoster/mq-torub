
use egui::Slider;
use macroquad::prelude::*;

#[macroquad::main("mq-torub")]
async fn main() {
    // debugging
    // std::env::set_var("RUST_BACKTRACE", "1");

    let mut size: u8 = 4;
    let mut matrix: Vec<Vec<(Color, String)>> = generate_matrix(size);
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    loop {
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .show(egui_ctx, |ui| {
                    if ui.add(Slider::new(&mut size, 2u8..=letters.len() as u8)).changed() {
                        matrix = generate_matrix(size);
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
                let (color, string) = &matrix[i as usize][j as usize];
                draw_text(string.as_str(), left_top_x + (node_size * (j+1) as f32), left_top_y + (node_size * (i+2) as f32), node_size, *color);
            }
        }
        for i in 0..size {
            draw_text("<", left_top_x, left_top_y + (node_size * (i+2) as f32), node_size, WHITE);
            draw_text(">", left_top_x + ((size+1) as f32 * node_size), left_top_y + (node_size * (i+2) as f32), node_size, WHITE);
            draw_text("^", left_top_x + (node_size * (i+1) as f32), left_top_y + node_size, node_size, WHITE);
            draw_text("v", left_top_x + (node_size * (i+1) as f32), left_top_y + (node_size * (size + 2) as f32), node_size, WHITE);
        }


        // logic
        // draw_text(format!("{node_x}, {node_y}").as_str(), 0.0, 20.0, 20.0, WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let node_x = ((mouse_x - left_top_x) / node_size) as i32;
            let node_y = ((mouse_y - left_top_y) / node_size) as i32;
            if node_x == 0 && (1..=size as i32).contains(&node_y) {
                let index = (node_y - 1) as usize;
                let tmp = matrix[index].remove(0);
                matrix[index].push(tmp);
            }
            if node_x == (size+1) as i32 && (1..=size as i32).contains(&node_y) {
                let index = (node_y - 1) as usize;
                let tmp = matrix[index].pop().unwrap();
                matrix[index].insert(0, tmp);
            }
            if node_y == 0 && (1..=size as i32).contains(&node_x) {
                let index = (node_x - 1) as usize;
                let tmp = matrix[0].remove(index as usize);
                for i in 1..size {
                    let lil_tmp = matrix[i as usize].remove(index);
                    matrix[(i-1) as usize].insert(index, lil_tmp);
                }
                matrix[(size-1) as usize].insert(index, tmp);
            }
            if node_y == (size + 1) as i32 && (1..=size as i32).contains(&node_x) {
                let index = (node_x - 1) as usize;
                let tmp = matrix[(size-1) as usize].remove(index as usize);
                for i in (0..size-1).rev() {
                    let lil_tmp = matrix[i as usize].remove(index);
                    matrix[(i+1) as usize].insert(index, lil_tmp);
                }
                matrix[0].insert(index, tmp);
            }
        }


        egui_macroquad::draw();
        next_frame().await
    }
}

fn generate_matrix(size: u8) -> Vec<Vec<(Color, String)>>{
    let colors = [WHITE, BLUE, RED, GREEN];
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut matrix: Vec<Vec<(Color, String)>> = Vec::with_capacity(size as usize);
    for i in 0..size {
        let mut row: Vec<(Color, String)> = Vec::with_capacity(size as usize);
        for j in 0..size {
            let color_string: (Color, String) = if (size as usize) <= colors.len() {
                let string: String = letters.chars().nth(j as usize).unwrap().to_string();
                (colors[i as usize], string)
            } else {
                let mut string: String = letters.chars().nth(j as usize).unwrap().to_string();
                string += format!("{i}").as_str();
                (WHITE, string)
            };
            row.push(color_string);
        }
        matrix.push(row);
    }
    matrix
}
