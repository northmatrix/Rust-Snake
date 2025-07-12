use macroquad::prelude::*;
use std::collections::LinkedList;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 600;
const GRID_SIZE: i32 = 20;
const PIXEL: i32 = WIDTH / GRID_SIZE;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

type Coord = (i32, i32);
struct Snake {
    head: Coord,
    body: LinkedList<Coord>,
    direction: Coord,
}

impl Snake {
    fn new(position_x: i32, position_y: i32) -> Self {
        Snake {
            head: (position_x, position_y),
            body: LinkedList::new(),
            direction: (0, 1),
        }
    }
    fn move_snake(&mut self, direction: &Coord) {
        let (x, y) = self.head;
        self.direction = *direction;
        self.body.pop_back();
        self.body.push_front((x, y));
        self.head.0 += self.direction.0;
        self.head.1 += self.direction.1;
    }
    fn draw_snake(&self) {
        draw_rectangle(
            (self.head.0 * PIXEL) as f32,
            (self.head.1 * PIXEL) as f32,
            PIXEL as f32,
            PIXEL as f32,
            DARKGREEN,
        );
        for x in self.body.iter() {
            draw_rectangle(
                (x.0 * PIXEL) as f32,
                (x.1 * PIXEL) as f32,
                PIXEL as f32,
                PIXEL as f32,
                GREEN,
            );
        }
    }
    fn is_alive(&self) -> bool {
        if self.head.0 > GRID_SIZE - 1 || self.head.0 < 0 {
            return false;
        }
        if self.head.1 > GRID_SIZE - 1 || self.head.1 < 0 {
            return false;
        }
        if self.body.is_empty() {
            return true;
        }
        for body_cell in self.body.iter() {
            if self.head == *body_cell {
                return false;
            }
        }
        true
    }
    fn grow_snake(&mut self, direction: &Coord) {
        self.direction = *direction;
        let (x, y) = self.head;
        self.head.0 += self.direction.0;
        self.head.1 += self.direction.1;
        self.body.push_front((x, y))
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut timer = 0.0;
    let mut snake: Snake = Snake::new(10, 10);
    let mut grow_next: bool = false;
    let mut alive: bool = true;
    let mut direction: Coord = (1, 0);
    let mut random: Coord = (rand::gen_range(0, GRID_SIZE), rand::gen_range(0, GRID_SIZE));
    let mut score: u32 = 0;
    loop {
        clear_background(BLACK);
        if alive {
            if is_key_pressed(KeyCode::W) && snake.direction != (0, 1) {
                direction = (0, -1);
            }
            if is_key_pressed(KeyCode::A) && snake.direction != (1, 0) {
                direction = (-1, 0);
            }
            if is_key_pressed(KeyCode::D) && snake.direction != (-1, 0) {
                direction = (1, 0);
            }
            if is_key_pressed(KeyCode::S) && snake.direction != (0, -1) {
                direction = (0, 1)
            }
            if random == snake.head || snake.body.iter().any(|&y| y == random) {
                random = (rand::gen_range(0, GRID_SIZE), rand::gen_range(0, GRID_SIZE));
                grow_next = true;
                score += 1;
            }

            timer += get_frame_time();

            if timer >= 0.05 {
                if grow_next {
                    snake.grow_snake(&direction);
                    grow_next = false;
                } else {
                    snake.move_snake(&direction);
                }
                timer = 0.0;
            }

            alive = snake.is_alive();

            draw_rectangle(
                (random.0 * PIXEL) as f32,
                (random.1 * PIXEL) as f32,
                PIXEL as f32,
                PIXEL as f32,
                RED,
            );
            snake.draw_snake();
            draw_text(
                &format!("Score: {}", score),
                0.0,
                PIXEL as f32,
                PIXEL as f32 * 1.5,
                WHITE,
            );
        } else {
            draw_text(
                &format!("Score: {}", score),
                PIXEL as f32 * GRID_SIZE as f32 / 2.0 - PIXEL as f32 * 3.0,
                PIXEL as f32 * GRID_SIZE as f32 / 2.0 - PIXEL as f32,
                PIXEL as f32 * 2.0,
                WHITE,
            );
            draw_text(
                "PRESS (R) TO RESTART",
                PIXEL as f32 * GRID_SIZE as f32 / 2.0 - PIXEL as f32 * 4.0,
                PIXEL as f32 * GRID_SIZE as f32 / 2.0,
                PIXEL as f32,
                WHITE,
            );
            if is_key_pressed(KeyCode::R) {
                score = 0;
                snake.head = (10, 10);
                snake.body = LinkedList::new();
                alive = true;
                random = (rand::gen_range(0, GRID_SIZE), rand::gen_range(0, GRID_SIZE));
            }
        }
        next_frame().await;
    }
}
