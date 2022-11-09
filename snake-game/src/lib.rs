#[cfg(feature = "console_error_panic_hook")]
mod utils;

use std::collections::VecDeque;
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, KeyboardEvent};


// use wee_alloc as the global memory allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    utils::set_panic_hook();
}


#[wasm_bindgen]
pub struct Game {
    width: usize, // width of the canvas(cells)
    snake: Snake,
    movements: VecDeque<Direction>, // user's commands
    direction: Direction, // current direction
    apple: Cell,
    score: usize,
}

#[wasm_bindgen]
impl Game {
    // Getters
    fn snake(&self) -> &Snake {
        &self.snake
    }

    fn apple(&self) -> &Cell {
        &self.apple
    }

    fn movements_mut(&mut self) -> &mut VecDeque<Direction> {
        &mut self.movements
    }

    fn movements(&self) -> &VecDeque<Direction> {
        &self.movements
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn score(&self) -> usize {
        self.score
    }
    
    pub fn head_position(&self) -> Cell {
        self.snake().cells()[0]
    }

    pub fn tail_position(&self) -> Cell {
        let len = self.snake().cells().len();
        self.snake().cells()[len - 1]
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn tail_direction(&self) -> Direction {
        let snake = self.snake().cells();
        let snake_len = snake.len();
        snake[snake_len - 2] - snake[snake_len - 1] // Second last cell - last cell gives the direction of the last cell
    }

    // Creating a new game
    pub fn new(width: usize) -> Self {
        let width = width as i16;
        let direction = Direction::Right;
        let snake_tail = (
            0,
            width / 2,
        );
        let snake = Snake::new(snake_tail.0, snake_tail.1, direction);

        // Keep generating an apple until it doesn't overlap with the snake
        let mut rng =  rand::thread_rng();
        let apple = loop {
            let apple = Cell(
                rng.gen_range(0..=width - 1),
                rng.gen_range(0..=width - 1),
            );
            if !snake.cells().contains(&apple) {
                break apple
            }
        };

        Self { width: width as usize, snake, movements: VecDeque::new(), direction, apple, score: 0}
    }
    
    // Initialize the game, quite the same as `new()`
    pub fn initialize(&mut self) {
        let width = self.width() as i16;
        let direction = Direction::Right;
        let snake_tail = (
            0,
            width / 2,
        );
        let snake = Snake::new(snake_tail.0, snake_tail.1, direction);
        let mut rng =  rand::thread_rng();
        let apple = loop {
            let apple = Cell(
                rng.gen_range(0..=width - 1),
                rng.gen_range(0..=width - 1),
            );
            if !snake.cells().iter().take(2).any(|x| x == &apple) {
                break apple
            }
        };
        self.score = 0;
        self.snake = snake;
        self.movements = VecDeque::new();
        self.direction = direction;
        self.apple = apple;
    }

    // the `update` method, return true if game will carry on
    pub fn update_and_check_continue(&mut self) -> bool {
        let width = self.width() as i16;
        self.direction = self.movements_mut().pop_front().unwrap_or_else(|| self.direction());

        let snake = self.snake.cells_mut();
        snake.pop(); // Here we pop the tail first so that we can check if new head hit new body

        let new_head = snake[0] + self.direction;
        // If hit wall or hit body, return false
        if !(0..width).contains(&new_head.x()) || !(0..width).contains(&new_head.y()) || snake.iter().any(|x| x == &new_head) {
            return false;
        }

        snake.insert(0, new_head);// Add new head

        let snake = self.snake.cells();
        let apple = self.apple();

        // If apple eaten, increase snake length and score by 1 
        if &snake[0] == apple {
            self.apple = self.new_apple();
            let snake = self.snake.cells_mut();
            snake.push(snake[snake.len() - 1] + (snake[snake.len() - 1] - snake[snake.len() - 2]));
            self.score += 1;
        }
        true
    }

    pub fn handle_keystroke(&mut self, key: KeyboardEvent) {
        let movement: Direction = match key.key().as_str() {
            "ArrowUp" | "k" | "w" => Direction::Up,
            "ArrowRight" | "l" | "d" => Direction::Right,
            "ArrowDown" | "j" | "s" => Direction::Down,
            "ArrowLeft" | "h" | "a" => Direction::Left,
            _ => return,
        };

        // Ignore the move if it is redundant or illegal(opposite to current direction)
        if self.movements().is_empty(){
            if movement != self.direction && !movement.is_opposite_to(&self.direction()) {
                self.movements_mut().push_back(movement);
            }
        } else {
            let last_movement = self.movements()[self.movements().len() - 1];
            if movement != last_movement && !movement.is_opposite_to(&last_movement) {
                self.movements_mut().push_back(movement);
            }
        }
    }

    fn new_apple(&self) -> Cell {
        let mut rng = rand::thread_rng();
        let snake = self.snake.cells();
        loop {
            let apple = Cell(
                rng.gen_range(0..=self.width() - 1) as i16,
                rng.gen_range(0..=self.width() - 1) as i16,
            );
            if !snake.iter().take(snake.len()).any(|x| x == &apple) {
                break apple
            }
        }
        
    }

    pub fn draw_apple(&self, context: &CanvasRenderingContext2d, cell_size: f64) {
        let apple_color = "#FFD700".into();
        context.set_fill_style(&apple_color);
        context.fill_rect(
            cell_size * self.apple().x() as f64,
            cell_size * self.apple().y() as f64,
            cell_size as f64,
            cell_size as f64,
        )
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Cell(i16, i16);

// Get difference of two cells
impl std::ops::Sub for Cell {
    type Output = Direction;
    fn sub(self, rhs: Self) -> Self::Output {
        match self.x() - rhs.x() {
            1 => Direction::Right,
            -1 => Direction::Left,
            0 => match self.y() - rhs.y() {
                1 => Direction::Down,
                -1 => Direction::Up,
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }
}

impl std::ops::Add<Direction> for Cell {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Cell(self.x(), self.y() - 1),
            Direction::Down => Cell(self.x(), self.y() + 1),
            Direction::Right => Cell(self.x() + 1, self.y()),
            Direction::Left => Cell(self.x() - 1, self.y()),
        }
    }
}

#[wasm_bindgen]
impl Cell {
    pub fn x(&self) -> i16 {
        self.0
    }
    pub fn y(&self) -> i16 {
        self.1
    }
}


struct Snake {
    cells: Vec<Cell>,
}

impl Snake {
    fn new(x: i16, y: i16, direction: Direction) -> Self {
        let tail = Cell(x, y);
        Self { cells: vec![tail + direction, tail] }
    }

    fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }
    fn cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// For easy generation of random direction
impl From<u8> for Direction {
    fn from(d: u8) -> Self {
        assert!(d < 4);
        match d {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn is_opposite_to(&self, rhs: &Direction) -> bool {
        matches!((self, rhs),
            (&Direction::Left, &Direction::Right) |
            (&Direction::Right, &Direction::Left) |
            (&Direction::Up, &Direction::Down) |
            (&Direction::Down, &Direction::Up)
        )
    }
}


#[wasm_bindgen]
// Drawing part the head and the tail
// This function will be called multiple times with increasing `l` to acheive smooth transition
pub fn draw(context: &CanvasRenderingContext2d, head: &Cell, head_dir: Direction, tail: &Cell, tail_dir: Direction, cell_size: f64, l: usize) {
    let l = l as f64;
    let (head_x, head_y): (f64, f64) = (head.x() as f64 * cell_size, head.y() as f64 * cell_size);
    let (tail_x, tail_y): (f64, f64) = (tail.x() as f64 * cell_size, tail.y() as f64 * cell_size);

    // Drawing the head
    context.set_fill_style(&"#2596be".into());
    match head_dir {
        Direction::Up  => context.fill_rect(head_x, head_y + cell_size - l, cell_size, l),
        Direction::Right  => context.fill_rect(head_x, head_y, l, cell_size),
        Direction::Down  => context.fill_rect(head_x, head_y, cell_size, l),
        Direction::Left  => context.fill_rect(head_x + cell_size - l, head_y, l, cell_size),
    };

    // Calculate the background colour of the cell
    let  box_color = match (tail_x as usize / cell_size as usize + tail_y as usize / cell_size as usize) % 2 {
        1 => "#18212f",
        _ => "#1e293b",
    };
    // Opposite to the head where the portion of snake body in the cell increases
    // We fill the cell with snake body first, and add background colour too acheive the effect of
    // snake leaving the cell
    context.fill_rect(tail_x, tail_y, cell_size, cell_size);

    context.set_fill_style(&box_color.into());
    match tail_dir {
        Direction::Up  => context.fill_rect(tail_x, tail_y + cell_size - l, cell_size, l),
        Direction::Right  => context.fill_rect(tail_x, tail_y, l, cell_size),
        Direction::Down  => context.fill_rect(tail_x, tail_y, cell_size, l),
        Direction::Left  => context.fill_rect(tail_x + cell_size - l, tail_y, l, cell_size),
    }
}


#[wasm_bindgen]
pub fn draw_init(game: &Game, cell_size: f64, context: &CanvasRenderingContext2d) {

    let dark_cell_color = "#18212f".into();
    let light_cell_color = "#1e293b".into();

    context.begin_path();
    for i in 0..game.width() {
        for j in 0..game.width() {
            context.set_fill_style(match (i + j) % 2 {
                1 => &dark_cell_color,
                _ => &light_cell_color
            });
            context.fill_rect(
                cell_size * i as f64 + 1.0,
                cell_size * j as f64 + 1.0,
                cell_size as f64,
                cell_size as f64,
            );
        }
    }
    let snake_color = "#2596be".into();
    context.set_fill_style(&snake_color);
    let body = game.snake().cells();
    for Cell(x, y) in body.iter().take(body.len() - 1) {
        context.fill_rect(
            cell_size * *x as f64,
            cell_size * *y as f64,
            cell_size as f64,
            cell_size as f64,
        )
    }
    let apple_color = "#FFD700".into();
    context.set_fill_style(&apple_color);
    context.fill_rect(
        cell_size * game.apple().x() as f64,
        cell_size * game.apple().y() as f64,
        cell_size as f64,
        cell_size as f64,
    )
}

