use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Generator".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);
        
        // Draw a black rectangle in the middle of the screen
        draw_rectangle(
            screen_width() / 4.0,
            screen_height() / 4.0,
            screen_width() / 2.0,
            screen_height() / 2.0,
            BLACK,
        );

        // Draw some text
        draw_text(
            "Maze Generator",
            20.0,
            40.0,
            30.0,
            BLACK,
        );
        
        next_frame().await
    }
}
