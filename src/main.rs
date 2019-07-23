// Main file

use crate::board::{Board, CellValue};
use crate::dom;
use crate::fps::FpsCounter;
use crate::profiler::Profiler;
use crate::utils;

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::{Closure, JsValue};
use web_sys::CanvasRenderingContext2d;

const BOARD_AMOUNT: usize = 25;
const ANIM_DURATION: f64 = 900.0;
const UPDATE_DELAY: f64 = 1000.0;

pub fn run() {
    let window = dom::window();
    let canvas = dom::canvas();
    let ctx = dom::context();

    utils::scale_canvas(&window, &canvas, &ctx);

    let width = dom::window_width();
    let height = dom::window_height();
    let board_spacing_x = (width / BOARD_AMOUNT as f64) * 0.125;
    let board_spacing_y = (height / BOARD_AMOUNT as f64) * 0.125;
    let total_spacing_x = BOARD_AMOUNT as f64 * board_spacing_x + board_spacing_x;
    let total_spacing_y = BOARD_AMOUNT as f64 * board_spacing_y + board_spacing_y;
    let board_width = ((width - total_spacing_x) / BOARD_AMOUNT as f64).floor();
    let board_height = ((height - total_spacing_y) / BOARD_AMOUNT as f64).floor();
    let board_dimensions = (board_width, board_height, board_spacing_x, board_spacing_y);
    let pointer = Rc::new(RefCell::new(None));
    let pointer_clone = pointer.clone();

    let mut last_update = 0.0;
    let mut last_render = 0.0;
    let mut fps_counter = FpsCounter::new();
    let mut boards = [[Board::default(); BOARD_AMOUNT]; BOARD_AMOUNT];
    let mut profiler = Profiler::new();
    let func = Box::new(move || {
        let now = dom::timestamp();
        let update_delta = now - last_update;
        let render_delta = now - last_render;
        let do_update = update_delta > UPDATE_DELAY;
        let progress_incr = render_delta / ANIM_DURATION;
        profiler.start();

        for x in 0..BOARD_AMOUNT {
            for y in 0..BOARD_AMOUNT {
                let mut board = boards[x][y];

                if do_update {
                    let was_finished = board.is_finished();
                    let was_crossed = board.is_crossed;
                    let had_winner = board.winner != CellValue::Empty;

                    if was_finished && had_winner && !was_crossed {
                        board.is_crossed = true;
                    }

                    if was_finished && had_winner && was_crossed {
                        board = Board::default();
                    }

                    if was_finished && !had_winner {
                        board = Board::default();
                    }

                    if !board.is_finished() {
                        board.play();
                    }

                    last_update = now;
                }

                for cell_x in 0..3 {
                    for cell_y in 0..3 {
                        let cell = board.cells[cell_x][cell_y];
                        let mut progress = board.progress[cell_x][cell_y];

                        if cell != CellValue::Empty && progress < 1.0 {
                            progress += progress_incr;
                        }

                        board.progress[cell_x][cell_y] = if progress < 1.0 { progress } else { 1.0 }
                    }
                }

                if board.is_crossed {
                    let progress = board.cross_progress + progress_incr;
                    board.cross_progress = if progress < 1.0 { progress } else { 1.0 }
                }

                boards[x][y] = board;
            }
        }

        profiler.stop();

        let fps = fps_counter.tick();
        let um = profiler.mean();

        render(&ctx, width, height, board_dimensions, &boards, fps, um);
        last_render = now;
        dom::request_animation_frame(pointer.borrow().as_ref().unwrap());
    });

    let closure = Closure::wrap(func as Box<dyn FnMut()>);
    *pointer_clone.borrow_mut() = Some(closure);
    dom::request_animation_frame(pointer_clone.borrow().as_ref().unwrap());
}

fn render(
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    board_dimensions: (f64, f64, f64, f64),
    boards: &[[Board; BOARD_AMOUNT]; BOARD_AMOUNT],
    fps: f64,
    update_mean: f64,
) {
    let (board_width, board_height, board_spacing_x, board_spacing_y) = board_dimensions;

    ctx.clear_rect(0.0, 0.0, width, height);
    ctx.begin_path();

    let sq_width = board_width / 3.0;
    let sq_height = board_height / 3.0;

    // Render the boards
    for x in 0..BOARD_AMOUNT {
        for y in 0..BOARD_AMOUNT {
            let offset_x = x as f64 * (board_width + board_spacing_x) + board_spacing_x;
            let offset_y = y as f64 * (board_height + board_spacing_y) + board_spacing_y;
            let board = boards[x][y];

            // Render the lines
            for i in 1..3 {
                let inner_offset_x = i as f64 * sq_width;
                let inner_offset_y = i as f64 * sq_height;
                ctx.move_to(offset_x + 0.0, offset_y + inner_offset_y);
                ctx.line_to(offset_x + board_width, offset_y + inner_offset_y);
                ctx.move_to(offset_x + inner_offset_x, offset_y + 0.0);
                ctx.line_to(offset_x + inner_offset_x, offset_y + board_height);
            }

            // Render the Os and the Xs
            for cell_x in 0..3 {
                for cell_y in 0..3 {
                    let center_x = cell_x as f64 * sq_width + (sq_width / 2.0) + offset_x;
                    let center_y = cell_y as f64 * sq_height + (sq_height / 2.0) + offset_y;
                    let r = sq_height / 5.0;
                    let cell = board.cells[cell_x][cell_y];
                    let progress = board.progress[cell_x][cell_y];

                    match cell {
                        CellValue::O => {
                            ctx.move_to(center_x + r, center_y);
                            ctx.arc(center_x, center_y, r, 0.0, f64::consts::PI * 2.0 * progress)
                                .unwrap();
                        }
                        CellValue::X => {
                            let origin_x = center_x - r;
                            let origin_y = center_y - r;
                            let target_x = center_x + r;
                            let target_y = center_y + r;
                            let inner_progress = if progress < 0.5 { progress * 2.0 } else { 1.0 };
                            let delta_x = (target_x - origin_x) * inner_progress;
                            let delta_y = (target_y - origin_y) * inner_progress;

                            ctx.move_to(origin_x, origin_y);
                            ctx.line_to(origin_x + delta_x, origin_y + delta_y);

                            let origin_x = center_x - r;
                            let origin_y = center_y + r;
                            let target_x = center_x + r;
                            let target_y = center_y - r;
                            let inner_progress = if progress > 0.5 {
                                (progress - 0.5) * 2.0
                            } else {
                                0.0
                            };
                            let delta_x = (target_x - origin_x) * inner_progress;
                            let delta_y = (target_y - origin_y) * inner_progress;

                            ctx.move_to(origin_x, origin_y);
                            ctx.line_to(origin_x + delta_x, origin_y + delta_y);
                        }
                        CellValue::Empty => {}
                    }
                }
            }

            // Render the cross-overs / strikethroughs
            if board.is_crossed {
                let result = board.get_win_row();
                let progress = board.cross_progress;

                match result {
                    Some(row) => {
                        let (x1, y1, x2, y2) = row;

                        let origin_x = x1 as f64 * sq_width + (sq_width / 2.0) + offset_x;
                        let origin_y = y1 as f64 * sq_height + (sq_height / 2.0) + offset_y;
                        let target_x = x2 as f64 * sq_width + (sq_width / 2.0) + offset_x;
                        let target_y = y2 as f64 * sq_height + (sq_height / 2.0) + offset_y;
                        let delta_x = (target_x - origin_x) * progress;
                        let delta_y = (target_y - origin_y) * progress;

                        ctx.move_to(origin_x, origin_y);
                        ctx.line_to(origin_x + delta_x, origin_y + delta_y);
                    }
                    None => {}
                }
            }
        }
    }

    ctx.set_stroke_style(&JsValue::from("#cccccc"));
    ctx.stroke();

    // Render metrics
    ctx.set_font("20px monospace");
    ctx.fill_text(&format!("FPS {:.0}", fps), 5.0, 20.0)
        .unwrap();

    ctx.fill_text(&format!("Update mean {:.2}", update_mean), 100.0, 20.0)
        .unwrap();
}
