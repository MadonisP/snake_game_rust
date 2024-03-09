use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
// wasm-pack build --target web

struct SnakeCell(usize);

struct Snake {
    body: Vec<usize>
}

impl Snake{
    fn new(spawn_index:usize) -> Snake {
        Snake { body: vec!(SnakeCell(spawn_index)) }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World { 
            width: 8, 
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
}
