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

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

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
        .height(WINDOW_HEIGHT / CELL_SIZE as usize)
        .width(WINDOW_WIDTH / CELL_SIZE as usize)
        .algorithm(Box::new(Prim::new()))
        .build();

    println!("{}", &maze);

    // loop {
    //     clear_background(WHITE);
        
    //     let offset_x = 50;  // Padding from left
    //     let offset_y = 50;  // Padding from top

    //     // Draw the maze
    //     for y in 0..maze.height() {
    //         for x in 0..maze.width() {
    //             let cell_x = offset_x + x as usize * 40;
    //             let cell_y = offset_y + y as usize * 40;

    //             let coords = (cell_x, cell_y);

    //             // Draw walls if there's no passage
    //             // North wall
    //             if !maze.is_carved(coords, Cell::NORTH) {
    //                 draw_rectangle(
    //                     cell_x as f32,
    //                     cell_y as f32,
    //                     CELL_SIZE,
    //                     WALL_THICKNESS,
    //                     BLACK,
    //                 );
    //             }

    //             // West wall
    //             if !maze.is_carved(coords, Cell::WEST) {
    //                 draw_rectangle(
    //                     cell_x as f32,
    //                     cell_y as f32,
    //                     WALL_THICKNESS,
    //                     CELL_SIZE,
    //                     BLACK,
    //                 );
    //             }

    //             // // Draw East wall if we're at the last column
    //             // if x == maze.width() - 1 {
    //             //     draw_rectangle(
    //             //         cell_x + CELL_SIZE,
    //             //         cell_y,
    //             //         WALL_THICKNESS,
    //             //         CELL_SIZE,
    //             //         BLACK,
    //             //     );
    //             // }

    //             // // Draw South wall if we're at the last row
    //             // if y == maze.height() - 1 {
    //             //     draw_rectangle(
    //             //         cell_x,
    //             //         cell_y + CELL_SIZE,
    //             //         CELL_SIZE + WALL_THICKNESS,
    //             //         WALL_THICKNESS,
    //             //         BLACK,
    //             //     );
    //             // }
    //         }
    //     }

    //     // // Generate new maze when space is pressed
    //     // if is_key_pressed(KeyCode::Space) {
    //     //     maze = OrthogonalMazeBuilder::new()
    //     //         .height(10)
    //     //         .width(10)
    //     //         .algorithm(Box::new(Prim::new()))
    //     //         .build();
    //     // }
        
    //     next_frame().await
    // }
}
