use macroquad::prelude::*;
use knossos::{
    maze::{
        Cell,
        OrthogonalMaze,
        OrthogonalMazeBuilder,
        Prim,
    },
};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

const CELL_SIZE: f32 = 40.0;
const WALL_THICKNESS: f32 = 2.0;
const PADDING: f32 = 20.0;  // Smaller padding around the maze
const SPRITE_SIZE: f32 = 30.0;  // Size of the moving sprite
const MOVEMENT_SPEED: f32 = 2.0;  // Pixels per frame

// Define maze dimensions
const MAZE_WIDTH: usize = 15;  // Number of cells horizontally
const MAZE_HEIGHT: usize = 15; // Number of cells vertically

// Calculate window dimensions based on maze size
const WINDOW_WIDTH: f32 = PADDING * 2.0 + CELL_SIZE * MAZE_WIDTH as f32;
const WINDOW_HEIGHT: f32 = PADDING * 2.0 + CELL_SIZE * MAZE_HEIGHT as f32;

// Calculate offset to center the maze
const OFFSET_X: f32 = PADDING;
const OFFSET_Y: f32 = PADDING;

// A* pathfinding structures
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: Position,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct GameState {
    maze: OrthogonalMaze,
    sprite_pos: Vec2,
    target_pos: Vec2,
    path: Vec<Position>,
    current_path_index: usize,
}

impl GameState {
    fn new(maze: OrthogonalMaze) -> Self {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: MAZE_WIDTH - 1, y: MAZE_HEIGHT - 1 };
        let path = Self::find_path(&maze, start, end);
        
        GameState {
            maze,
            sprite_pos: Vec2::new(
                OFFSET_X + CELL_SIZE / 2.0 - SPRITE_SIZE / 2.0,
                OFFSET_Y + CELL_SIZE / 2.0 - SPRITE_SIZE / 2.0
            ),
            target_pos: Vec2::new(
                OFFSET_X + CELL_SIZE / 2.0 - SPRITE_SIZE / 2.0,
                OFFSET_Y + CELL_SIZE / 2.0 - SPRITE_SIZE / 2.0
            ),
            path,
            current_path_index: 0,
        }
    }

    fn find_path(maze: &OrthogonalMaze, start: Position, goal: Position) -> Vec<Position> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        open_set.push(Node { pos: start, f_score: 0 });
        g_score.insert(start, 0);
        f_score.insert(start, Self::heuristic(start, goal));

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                return Self::reconstruct_path(came_from, current.pos);
            }

            let directions = [Cell::NORTH, Cell::SOUTH, Cell::EAST, Cell::WEST];
            for &dir in &directions {
                if maze.is_carved((current.pos.x, current.pos.y), dir) {
                    let neighbor = match dir {
                        Cell::NORTH if current.pos.y > 0 => 
                            Position { x: current.pos.x, y: current.pos.y - 1 },
                        Cell::SOUTH if current.pos.y < MAZE_HEIGHT - 1 => 
                            Position { x: current.pos.x, y: current.pos.y + 1 },
                        Cell::EAST if current.pos.x < MAZE_WIDTH - 1 => 
                            Position { x: current.pos.x + 1, y: current.pos.y },
                        Cell::WEST if current.pos.x > 0 => 
                            Position { x: current.pos.x - 1, y: current.pos.y },
                        _ => continue,
                    };

                    let tentative_g_score = g_score[&current.pos] + 1;
                    if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                        came_from.insert(neighbor, current.pos);
                        g_score.insert(neighbor, tentative_g_score);
                        let f = tentative_g_score + Self::heuristic(neighbor, goal);
                        f_score.insert(neighbor, f);
                        open_set.push(Node { pos: neighbor, f_score: f });
                    }
                }
            }
        }
        vec![] // No path found
    }

    fn heuristic(pos: Position, goal: Position) -> i32 {
        ((pos.x as i32 - goal.x as i32).abs() + (pos.y as i32 - goal.y as i32).abs()) as i32
    }

    fn reconstruct_path(came_from: HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
        let mut path = vec![current];
        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }
        path.reverse();
        path
    }

    fn update(&mut self) {
        if self.current_path_index >= self.path.len() {
            return;
        }

        let target_cell = self.path[self.current_path_index];
        self.target_pos = Vec2::new(
            OFFSET_X + target_cell.x as f32 * CELL_SIZE + CELL_SIZE / 2.0 - SPRITE_SIZE / 2.0,
            OFFSET_Y + target_cell.y as f32 * CELL_SIZE + CELL_SIZE / 2.0 - SPRITE_SIZE / 2.0
        );

        let direction = (self.target_pos - self.sprite_pos).normalize_or_zero();
        self.sprite_pos += direction * MOVEMENT_SPEED;

        // Check if we've reached the target position
        if (self.target_pos - self.sprite_pos).length() < MOVEMENT_SPEED {
            self.sprite_pos = self.target_pos;
            self.current_path_index += 1;
        }
    }

    fn draw(&self) {
        clear_background(WHITE);

        // Draw the maze
        for y in 0..self.maze.height() {
            for x in 0..self.maze.width() {
                // North wall
                if !self.maze.is_carved((x, y), Cell::NORTH) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        WALL_THICKNESS,
                        BLACK,
                    );
                }

                // West wall
                if !self.maze.is_carved((x, y), Cell::WEST) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + y as f32 * CELL_SIZE,
                        WALL_THICKNESS,
                        CELL_SIZE,
                        BLACK,
                    );
                }

                // South wall
                if !self.maze.is_carved((x, y), Cell::SOUTH) {
                    draw_rectangle(
                        OFFSET_X + x as f32 * CELL_SIZE,
                        OFFSET_Y + (y + 1) as f32 * CELL_SIZE - WALL_THICKNESS,
                        CELL_SIZE,
                        WALL_THICKNESS,
                        BLACK,
                    );
                }

                // East wall
                if !self.maze.is_carved((x, y), Cell::EAST) {
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

        // Draw start and end points
        draw_rectangle(
            OFFSET_X + CELL_SIZE / 2.0 - 10.0,
            OFFSET_Y + CELL_SIZE / 2.0 - 10.0,
            20.0,
            20.0,
            GREEN,
        );
        draw_rectangle(
            OFFSET_X + (MAZE_WIDTH - 1) as f32 * CELL_SIZE + CELL_SIZE / 2.0 - 10.0,
            OFFSET_Y + (MAZE_HEIGHT - 1) as f32 * CELL_SIZE + CELL_SIZE / 2.0 - 10.0,
            20.0,
            20.0,
            RED,
        );

        // Draw the sprite
        draw_rectangle(
            self.sprite_pos.x,
            self.sprite_pos.y,
            SPRITE_SIZE,
            SPRITE_SIZE,
            BLUE,
        );

        // Draw instructions
        draw_text(
            "Press SPACE to generate new maze",
            PADDING,
            WINDOW_HEIGHT - PADDING / 2.0,
            20.0,
            BLACK,
        );
    }
}

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
    let mut game = GameState::new(
        OrthogonalMazeBuilder::new()
            .height(MAZE_HEIGHT)
            .width(MAZE_WIDTH)
            .algorithm(Box::new(Prim::new()))
            .build()
    );

    loop {
        // Generate new maze when space is pressed
        if is_key_pressed(KeyCode::Space) {
            game = GameState::new(
                OrthogonalMazeBuilder::new()
                    .height(MAZE_HEIGHT)
                    .width(MAZE_WIDTH)
                    .algorithm(Box::new(Prim::new()))
                    .build()
            );
        }

        game.update();
        game.draw();
        
        next_frame().await
    }
}
