use macroquad::prelude::*;

mod ball;
mod brick;
mod paddle;

#[macroquad::main("Brick Breaker")]
async fn main() {
    let mut paddle: paddle::Paddle = initialize_paddle();
    let mut ball: ball::Ball = initialize_ball();
    let mut bricks: Vec<brick::Brick> = initialize_bricks(17, 5);

    loop {
        if ball.moving {
            paddle.update();
            ball.update(&*paddle.get_position());
            /* for brick in &mut bricks {
                brick.update();
            } */
            clear_background(BLACK);
            paddle.draw();
            ball.draw();
            for brick in &bricks {
                brick.draw();
            }
            next_frame().await
        } else {
            // game over - restart game ?
            break;
        }
    }
}

fn initialize_bricks(amount: i32, amount_per_row: i32) -> Vec<brick::Brick> {
    let start_y = 50.0;
    let block_height = 20.0;
    let padding = 5.0;
    let mut blocks = Vec::new();

    if amount == 0 || amount_per_row == 0 {
        return blocks;
    }

    // Calculate block width so all fit evenly with padding
    let block_width =
        (screen_width() - padding * (amount_per_row as f32 + 1.0)) / amount_per_row as f32;

    for i in 0..amount {
        let row = i / amount_per_row;
        let col = i % amount_per_row;

        let brick = brick::Brick::new(
            padding + col as f32 * (block_width + padding),
            start_y + row as f32 * (block_height + padding),
            block_width,
            block_height,
            1,
            BLUE,
        );
        blocks.push(brick);
    }
    blocks
}

fn initialize_paddle() -> paddle::Paddle {
    let paddle_width: f32 = 100.0;
    let paddle_height: f32 = 20.0;
    let paddle_speed: f32 = 500.0; // Pixels per second
    let paddle_x: f32 = (screen_width() - paddle_width) / 2.0;
    let paddle_y: f32 = screen_height() - 50.0;

    return paddle::Paddle::new(
        paddle_x,
        paddle_y,
        paddle_width,
        paddle_height,
        paddle_speed,
    );
}

fn initialize_ball() -> ball::Ball {
    let ball_radius: f32 = 10.0;
    let ball_x: f32 = screen_width() / 2.0;
    let ball_y: f32 = screen_height() / 2.0;
    let ball_velocity_x: f32 = -400.0;
    let ball_velocity_y: f32 = -400.0;

    return ball::Ball::new(
        ball_x,
        ball_y,
        ball_velocity_x,
        ball_velocity_y,
        ball_radius,
        true,
    );
}
