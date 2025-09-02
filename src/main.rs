use macroquad::prelude::*;

mod ball;
mod paddle;

#[macroquad::main("Brick Breaker")]
async fn main() {
    /* Players paddle */
    let paddle_width: f32 = 100.0;
    let paddle_height: f32 = 20.0;
    let paddle_speed: f32 = 500.0; // Pixels per second
    let paddle_x: f32 = (screen_width() - paddle_width) / 2.0;
    let paddle_y: f32 = screen_height() - 50.0;

    let mut paddle = paddle::Paddle::new(
        paddle_x,
        paddle_y,
        paddle_width,
        paddle_height,
        paddle_speed,
    );

    /* Ball */
    let ball_radius: f32 = 10.0;
    let ball_x: f32 = screen_width() / 2.0;
    let ball_y: f32 = screen_height() / 2.0;
    let ball_velocity_x: f32 = -400.0;
    let ball_velocity_y: f32 = -400.0;

    let mut ball = ball::Ball::new(
        ball_x,
        ball_y,
        ball_velocity_x,
        ball_velocity_y,
        ball_radius,
        true,
    );

    loop {
        if ball.moving {
            paddle.update();
            ball.update(&*paddle.get_position());
            clear_background(BLACK);
            paddle.draw();
            ball.draw();
            next_frame().await
        } else {
            // game over - restart game ?
            break;
        }
    }
}
