use macroquad::prelude::*;
use knossos::{
    maze::{
        Cell,
        OrthogonalMazeBuilder,
        Prim,
    },
};

const CELL_SIZE: f32 = 40.0;
const WALL_THICKNESS: f32 = 2.0;
const PADDING: f32 = 20.0;  // Smaller padding around the maze

// Define maze dimensions
const MAZE_WIDTH: usize = 15;  // Number of cells horizontally
const MAZE_HEIGHT: usize = 15; // Number of cells vertically

// Calculate window dimensions based on maze size
const WINDOW_WIDTH: f32 = PADDING * 2.0 + CELL_SIZE * MAZE_WIDTH as f32;
const WINDOW_HEIGHT: f32 = PADDING * 2.0 + CELL_SIZE * MAZE_HEIGHT as f32;

// Calculate offset to center the maze
const OFFSET_X: f32 = PADDING;
const OFFSET_Y: f32 = PADDING;

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
    let mut maze = OrthogonalMazeBuilder::new()
        .height(MAZE_HEIGHT)
        .width(MAZE_WIDTH)
        .algorithm(Box::new(Prim::new()))
        .build();

    loop {
        clear_background(WHITE);

        // Draw the maze
        for y in 0..maze.height() {
            for x in 0..maze.width() {
                // North wall
                if !maze.is_carved((x, y), Cell::NORTH) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        WALL_THICKNESS,
                        BLACK,
                    );
                }

                // West wall
                if !maze.is_carved((x, y), Cell::WEST) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + y as f32 * CELL_SIZE,
                        WALL_THICKNESS,
                        CELL_SIZE,
                        BLACK,
                    );
                }

                // South wall
                if !maze.is_carved((x, y), Cell::SOUTH) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + (y + 1) as f32 * CELL_SIZE - WALL_THICKNESS,
                        CELL_SIZE,
                        WALL_THICKNESS,
                        BLACK,
                    );
                }

                // East wall
                if !maze.is_carved((x, y), Cell::EAST) {
                    draw_rectangle(
                        OFFSET_X + (x + 1) as f32 * CELL_SIZE - WALL_THICKNESS,
                        OFFSET_Y + y as f32 * CELL_SIZE,
                        WALL_THICKNESS,
                        CELL_SIZE,
                        BLACK,
                    );
                }
            }
        }

        // Draw instructions
        draw_text(
            "Press SPACE to generate new maze",
            PADDING,
            WINDOW_HEIGHT - PADDING / 2.0,
            20.0,
            BLACK,
        );

        // Generate new maze when space is pressed
        if is_key_pressed(KeyCode::Space) {
            maze = OrthogonalMazeBuilder::new()
                .height(MAZE_HEIGHT)
                .width(MAZE_WIDTH)
                .algorithm(Box::new(Prim::new()))
                .build();
        }
        
        next_frame().await
    }
}
