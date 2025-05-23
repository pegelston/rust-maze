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

const CELL_SIZE: f32 = 60.0;  // Increased cell size to fit debug text
const WALL_THICKNESS: f32 = 2.0;
const PADDING: f32 = 20.0;
const SPRITE_SIZE: f32 = 30.0;
const MOVEMENT_SPEED: f32 = 2.0;
const FONT_SIZE: f32 = 12.0;

// Define maze dimensions
const MAZE_WIDTH: usize = 15;
const MAZE_HEIGHT: usize = 15;

// Calculate window dimensions based on maze size
const WINDOW_WIDTH: f32 = PADDING * 2.0 + CELL_SIZE * MAZE_WIDTH as f32;
const WINDOW_HEIGHT: f32 = PADDING * 2.0 + CELL_SIZE * MAZE_HEIGHT as f32;

// Calculate offset to center the maze
const OFFSET_X: f32 = PADDING;
const OFFSET_Y: f32 = PADDING;

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

#[derive(Clone)]
struct CellDebugInfo {
    f_score: Option<i32>,
    g_score: Option<i32>,
    h_score: Option<i32>,
    in_closed_set: bool,
    in_open_set: bool,
    in_final_path: bool,
}

impl CellDebugInfo {
    fn new() -> Self {
        CellDebugInfo {
            f_score: None,
            g_score: None,
            h_score: None,
            in_closed_set: false,
            in_open_set: false,
            in_final_path: false,
        }
    }
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
    debug_info: HashMap<Position, CellDebugInfo>,
}

impl GameState {
    fn new(maze: OrthogonalMaze) -> Self {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: MAZE_WIDTH - 1, y: MAZE_HEIGHT - 1 };
        let (path, debug_info) = Self::find_path(&maze, start, end);
        
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
            debug_info,
        }
    }

    fn find_path(maze: &OrthogonalMaze, start: Position, goal: Position) -> (Vec<Position>, HashMap<Position, CellDebugInfo>) {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();
        let mut debug_info = HashMap::new();
        let mut closed_set = Vec::new();

        open_set.push(Node { pos: start, f_score: 0 });
        g_score.insert(start, 0);
        let h = Self::heuristic(start, goal);
        f_score.insert(start, h);

        // Initialize debug info for start
        let mut start_debug = CellDebugInfo::new();
        start_debug.g_score = Some(0);
        start_debug.h_score = Some(h);
        start_debug.f_score = Some(h);
        start_debug.in_open_set = true;
        debug_info.insert(start, start_debug);

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                let path = Self::reconstruct_path(&came_from, current.pos);
                // Mark final path in debug info
                for pos in &path {
                    if let Some(info) = debug_info.get_mut(pos) {
                        info.in_final_path = true;
                    }
                }
                return (path, debug_info);
            }

            // Update closed set info
            if let Some(info) = debug_info.get_mut(&current.pos) {
                info.in_open_set = false;
                info.in_closed_set = true;
            }
            closed_set.push(current.pos);

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
                        let h = Self::heuristic(neighbor, goal);
                        let f = tentative_g_score + h;
                        f_score.insert(neighbor, f);
                        open_set.push(Node { pos: neighbor, f_score: f });

                        // Update debug info
                        let neighbor_debug = debug_info.entry(neighbor).or_insert(CellDebugInfo::new());
                        neighbor_debug.g_score = Some(tentative_g_score);
                        neighbor_debug.h_score = Some(h);
                        neighbor_debug.f_score = Some(f);
                        neighbor_debug.in_open_set = true;
                    }
                }
            }
        }
        (vec![], debug_info) // No path found
    }

    fn heuristic(pos: Position, goal: Position) -> i32 {
        ((pos.x as i32 - goal.x as i32).abs() + (pos.y as i32 - goal.y as i32).abs()) as i32
    }

    fn reconstruct_path(came_from: &HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
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

        if (self.target_pos - self.sprite_pos).length() < MOVEMENT_SPEED {
            self.sprite_pos = self.target_pos;
            self.current_path_index += 1;
        }
    }

    fn draw(&self) {
        clear_background(WHITE);

        // Draw the maze and debug info
        for y in 0..self.maze.height() {
            for x in 0..self.maze.width() {
                let cell_x = OFFSET_X + x as f32 * CELL_SIZE;
                let cell_y = OFFSET_Y + y as f32 * CELL_SIZE;
                let pos = Position { x, y };

                // Draw cell background based on debug info
                if let Some(info) = self.debug_info.get(&pos) {
                    let bg_color = if info.in_final_path {
                        Color::new(0.8, 1.0, 0.8, 0.3) // Light green for path
                    } else if info.in_closed_set {
                        Color::new(1.0, 0.8, 0.8, 0.3) // Light red for closed set
                    } else if info.in_open_set {
                        Color::new(0.8, 0.8, 1.0, 0.3) // Light blue for open set
                    } else {
                        WHITE
                    };
                    draw_rectangle(cell_x, cell_y, CELL_SIZE, CELL_SIZE, bg_color);

                    // Draw debug text
                    let text_y_offset = FONT_SIZE + 2.0;
                    if let Some(f) = info.f_score {
                        draw_text(&format!("f={}", f), cell_x + 4.0, cell_y + text_y_offset, FONT_SIZE, BLACK);
                    }
                    if let Some(g) = info.g_score {
                        draw_text(&format!("g={}", g), cell_x + 4.0, cell_y + text_y_offset * 2.0, FONT_SIZE, BLACK);
                    }
                    if let Some(h) = info.h_score {
                        draw_text(&format!("h={}", h), cell_x + 4.0, cell_y + text_y_offset * 3.0, FONT_SIZE, BLACK);
                    }
                }

                // Draw walls
                if !self.maze.is_carved((x, y), Cell::NORTH) {
                    draw_rectangle(cell_x, cell_y, CELL_SIZE, WALL_THICKNESS, BLACK);
                }
                if !self.maze.is_carved((x, y), Cell::WEST) {
                    draw_rectangle(cell_x, cell_y, WALL_THICKNESS, CELL_SIZE, BLACK);
                }
                if !self.maze.is_carved((x, y), Cell::SOUTH) {
                    draw_rectangle(cell_x, cell_y + CELL_SIZE - WALL_THICKNESS, CELL_SIZE, WALL_THICKNESS, BLACK);
                }
                if !self.maze.is_carved((x, y), Cell::EAST) {
                    draw_rectangle(cell_x + CELL_SIZE - WALL_THICKNESS, cell_y, WALL_THICKNESS, CELL_SIZE, BLACK);
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

        // Draw legend
        let legend_x = WINDOW_WIDTH - 200.0;
        let legend_y = PADDING;
        draw_text("Legend:", legend_x, legend_y, 20.0, BLACK);
        draw_rectangle(legend_x, legend_y + 30.0, 20.0, 20.0, Color::new(0.8, 1.0, 0.8, 0.3));
        draw_text("Final Path", legend_x + 30.0, legend_y + 45.0, 16.0, BLACK);
        draw_rectangle(legend_x, legend_y + 60.0, 20.0, 20.0, Color::new(1.0, 0.8, 0.8, 0.3));
        draw_text("Closed Set", legend_x + 30.0, legend_y + 75.0, 16.0, BLACK);
        draw_rectangle(legend_x, legend_y + 90.0, 20.0, 20.0, Color::new(0.8, 0.8, 1.0, 0.3));
        draw_text("Open Set", legend_x + 30.0, legend_y + 105.0, 16.0, BLACK);
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
