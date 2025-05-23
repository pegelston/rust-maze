use macroquad::prelude::*;
use knossos::{
    maze::{
        Cell,
        OrthogonalMazeBuilder,
        Prim,
    },
};

const CELL_SIZE: f32 = 40.0;  // Made cells bigger for better visibility
const WALL_THICKNESS: f32 = 2.0;

const OFFSET_X: f32 = 50.0;  // Padding from left
const OFFSET_Y: f32 = 50.0;  // Padding from top

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 1000.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Generator".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let maze = OrthogonalMazeBuilder::new()
        .height((WINDOW_HEIGHT / CELL_SIZE) as usize)
        .width((WINDOW_WIDTH / CELL_SIZE) as usize)
        .algorithm(Box::new(Prim::new()))
        .build();

    loop {
        clear_background(WHITE);

        for y in 0..maze.height() {
            for x in 0..maze.width() {
                if !maze.is_carved((x, y), Cell::NORTH) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        WALL_THICKNESS,
                        BLACK,
                    );
                }
            }
        }
        
        next_frame().await
    }
}
