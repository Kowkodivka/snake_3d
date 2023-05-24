use std::collections::LinkedList;

use macroquad::prelude::*;

const SQUARES: i16 = 20;
const CUBE_SIZE: Vec3 = vec3(1., 1., 1.);

const LEFT: Vec3 = vec3(0., 0., -1.);
const RIGHT: Vec3 = vec3(0., 0., 1.);

const UP: Vec3 = vec3(1., 0., 0.);
const DOWN: Vec3 = vec3(-1., 0., 0.);

struct Grid {
    size: i16,
    cell_size: f32,
}

struct Snake {
    dir: Vec3,
    head: Vec3,
    body: LinkedList<Vec3>,
}

#[macroquad::main("Snake")]
async fn main() {
    let grid = Grid {
        size: SQUARES,
        cell_size: 1.0,
    };

    let mut snake = Snake {
        dir: vec3(1., 0., 0.),
        head: vec3(0., 1., 0.),
        body: LinkedList::new(),
    };

    let mut fruit = generate_fruit();
    let mut score = 0;
    let mut speed = 0.7;
    let mut last_update = get_time();
    let mut gameover = false;

    loop {
        if !gameover {
            handle_input(&mut snake);

            if get_time() - last_update > speed {
                last_update = get_time();

                update_snake_position(
                    &mut snake,
                    &mut gameover,
                    &mut fruit,
                    &mut score,
                    &mut speed,
                );

                check_collision(&mut snake, &mut gameover);
            }
        }

        if !gameover {
            clear_background(LIGHTGRAY);

            set_camera(&Camera3D {
                position: vec3(-10., 10., -5.),
                up: vec3(0., 1., 0.),
                target: snake.head,
                ..Default::default()
            });

            draw_grid(&grid, BLACK);
            draw_snake(&snake);
            draw_fruit(fruit);

            set_default_camera();

            draw_score(score);
        }

        if gameover {
            if is_key_pressed(KeyCode::R) {
                gameover = false;

                snake = Snake {
                    dir: vec3(1., 0., 0.),
                    head: vec3(0., 1., 0.),
                    body: LinkedList::new(),
                };

                fruit = generate_fruit();
                score = 0;
                speed = 0.7;
                last_update = get_time();
            }

            draw_gameover();
        }

        next_frame().await
    }
}

fn generate_fruit() -> Vec3 {
    vec3(
        rand::gen_range(0, SQUARES) as f32,
        1.,
        rand::gen_range(0, SQUARES) as f32,
    )
}

fn handle_input(snake: &mut Snake) {
    if is_key_pressed(KeyCode::A) {
        snake.dir = LEFT;
    } else if is_key_pressed(KeyCode::D) {
        snake.dir = RIGHT;
    } else if is_key_pressed(KeyCode::W) {
        snake.dir = UP;
    } else if is_key_pressed(KeyCode::S) {
        snake.dir = DOWN;
    }
}

fn update_snake_position(
    snake: &mut Snake,
    gameover: &mut bool,
    fruit: &mut Vec3,
    score: &mut i32,
    speed: &mut f64,
) {
    snake.body.push_front(snake.head);
    snake.head += snake.dir;

    if snake.head == *fruit {
        *fruit = generate_fruit();
        *score += 1;
        *speed -= 0.1;
    } else {
        snake.body.pop_back();
    }

    if snake.head.x < 0.
        || snake.head.y < 0.
        || snake.head.z < 0.
        || snake.head.x >= SQUARES as f32
        || snake.head.y >= SQUARES as f32
        || snake.head.z >= SQUARES as f32
    {
        *gameover = true;
    }

    for segment in &snake.body {
        if *segment == snake.head {
            *gameover = true;
            break;
        }
    }
}

fn check_collision(snake: &mut Snake, gameover: &mut bool) {
    if snake.body.contains(&snake.head) {
        *gameover = true;
    }
}

fn draw_grid(grid: &Grid, color: Color) {
    for i in 1..grid.size {
        let pos = i as f32 * grid.cell_size;

        draw_line_3d(
            vec3(pos, 0.0, 0.0),
            vec3(pos, 0.0, grid.size as f32 * grid.cell_size),
            color,
        );

        draw_line_3d(
            vec3(0.0, 0.0, pos),
            vec3(grid.size as f32 * grid.cell_size, 0.0, pos),
            color,
        );
    }
}

fn draw_snake(snake: &Snake) {
    for segment in &snake.body {
        draw_cube(
            vec3(segment.x, segment.y, segment.z),
            CUBE_SIZE,
            None,
            Color::new(0.0, 0.8, 0.0, 1.0),
        );
    }

    draw_cube(
        vec3(snake.head.x, snake.head.y, snake.head.z),
        CUBE_SIZE,
        None,
        Color::new(0.0, 1.0, 0.0, 1.0),
    );
}

fn draw_fruit(fruit: Vec3) {
    draw_cube(
        vec3(fruit.x, fruit.y, fruit.z),
        CUBE_SIZE,
        None,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );
}

fn draw_gameover() {
    draw_text(
        "Game Over",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        48.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );
}

fn draw_score(score: i32) {
    let score_text = format!("Score: {}", score);
    draw_text(
        &score_text,
        30.0,
        30.0,
        32.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
}
