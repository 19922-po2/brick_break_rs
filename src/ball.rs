use macroquad::prelude::*;

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

    pub fn update(&mut self, &paddle_position: &Rect) {
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

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}
