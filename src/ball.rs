use macroquad::prelude::*;

use crate::brick;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub moving: bool,
}

impl Ball {
    pub fn new(x: f32, y: f32, vel_x: f32, vel_y: f32, radius: f32, moving: bool) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(vel_x, vel_y),
            radius,
            moving,
        }
    }

    pub fn update(&mut self, &paddle_position: &Rect, bricks: &mut Vec<brick::Brick>) {
        let delta_time = get_frame_time();
        self.pos += self.vel * delta_time; // Update position based on velocity

        // right
        if self.pos.x + self.radius > screen_width() {
            self.vel.x *= -1.0;
            self.pos.x = screen_width() - self.radius; // Correct position to prevent "clipping"
        }
        // left
        if self.pos.x - self.radius < 0.0 {
            self.vel.x *= -1.0;
            self.pos.x = self.radius;
        }
        // brick collision
        for block in bricks.iter_mut() {
            if block.health > 0 && self.collides_with_block(block) {
                self.vel.y *= -1.0; // Bounce vertically
                block.decrease_health(1);
                break; // Only collide with one brick per frame
            }
        }
        // top
        if self.pos.y - self.radius < 0.0 {
            self.vel.y *= -1.0;
            self.pos.y = self.radius;
        }
        // Paddle collision
        if self.pos.y + self.radius >= paddle_position.y
            && self.pos.y - self.radius <= paddle_position.y + paddle_position.h
            && self.pos.x + self.radius >= paddle_position.x
            && self.pos.x - self.radius <= paddle_position.x + paddle_position.w
        {
            self.vel.y *= -1.0;
            self.pos.y = paddle_position.y - self.radius; // put ball just above paddle
        }

        // Bottom (game over)
        if self.pos.y - self.radius > screen_height() {
            self.moving = false;
            println!("GAME OVER!!");
        }
    }

    fn collides_with_block(&self, block: &brick::Brick) -> bool {
        println!("colling with brick!");
        let ball_left = self.pos.x - self.radius;
        let ball_right = self.pos.x + self.radius;
        let ball_top = self.pos.y - self.radius;
        let ball_bottom = self.pos.y + self.radius;

        let block_left = block.rect.x;
        let block_right = block.rect.x + block.rect.w;
        let block_top = block.rect.y;
        let block_bottom = block.rect.y + block.rect.h;

        ball_right >= block_left
            && ball_left <= block_right
            && ball_bottom >= block_top
            && ball_top <= block_bottom
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}
