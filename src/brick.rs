use macroquad::prelude::*;

pub struct Brick {
    pub rect: Rect,
    pub health: i32,
    pub color: Color,
}

impl Brick {
    pub fn new(x: f32, y: f32, width: f32, height: f32, health: i32, color: Color) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            health,
            color,
        }
    }

    /* pub fn get_position(&self) -> &Rect {
        return &self.rect;
    } */

    pub fn decrease_health(&mut self, damage: i32) {
        self.health -= damage;
    }

    /* pub fn update(&mut self) {} */

    pub fn draw(&self) {
        if self.health > 0 {
            draw_rectangle(
                self.rect.x,
                self.rect.y,
                self.rect.w,
                self.rect.h,
                self.color,
            );
        }
    }
}
