use macroquad::prelude::*;

pub struct Paddle {
    pub rect: Rect,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            speed,
        }
    }

    pub fn get_position(&self) -> &Rect {
        return &self.rect;
    }

    pub fn update(&mut self) {
        // Handles player input to move the paddle
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Left) {
            self.rect.x -= self.speed * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            self.rect.x += self.speed * delta_time;
        }

        // keep it inside the window
        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        }
        if self.rect.x + self.rect.w > screen_width() {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
