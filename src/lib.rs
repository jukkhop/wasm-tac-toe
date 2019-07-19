// Main file

mod board;
#[macro_use]
mod dom;
mod math;
mod utils;

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use board::{Board, CellValue};
use dom::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const BOARD_SIZE: f64 = 75.0;
const BOARD_SPACING: f64 = 25.0;
const BOARD_AMOUNT: usize = 3;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = dom::window();
    let canvas = dom::canvas();
    let ctx = dom::context();

    utils::scale_canvas(&window, &canvas, &ctx);

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    let boards = [[Board {
        cells: [[CellValue::Empty; 3]; 3],
        current_player: CellValue::O,
        move_count: 0,
        winner: CellValue::Empty,
    }; BOARD_AMOUNT]; BOARD_AMOUNT];

    // for x in 0..3 {
    //     for y in 0..3 {
    //         boards[x][y] = Board::new();
    //     }
    // }

    // let mut board = Board::default();
    let delay = 1000;

    let pointer = Rc::new(RefCell::new(None));
    let pointer_clone = pointer.clone();

    let func = Box::new(move || {
        log!("boards {:?}", boards);

        for x in 0..3 {
            for y in 0..3 {
                let mut board = boards[x][y];

                // let finished = board.is_finished();
                // log!("Finished: {}", finished);

                if board.is_finished() {
                    log!("Reseting board...");
                    board.reset();
                }

                if !board.is_finished() {
                    // log!("Playing board...");
                    board.play();
                    // log!("Board played");
                }
            }
        }

        render(&ctx, width, height, &boards);
        dom::set_timeout(pointer.borrow().as_ref().unwrap(), delay);
    });

    let closure = Closure::wrap(func as Box<dyn FnMut()>);
    *pointer_clone.borrow_mut() = Some(closure);

    dom::set_timeout(pointer_clone.borrow().as_ref().unwrap(), 0);
    Ok(())
}

fn render(
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    boards: &[[Board; BOARD_AMOUNT]; BOARD_AMOUNT],
) {
    ctx.clear_rect(0.0, 0.0, width, height);
    ctx.begin_path();

    let sq_size = BOARD_SIZE / 3.0;
    let sq_half = sq_size / 2.0;

    // Render the boards
    for x in 0..BOARD_AMOUNT {
        for y in 0..BOARD_AMOUNT {
            let offset_x = x as f64 * (BOARD_SIZE + BOARD_SPACING);
            let offset_y = y as f64 * (BOARD_SIZE + BOARD_SPACING);
            // log!("offset_x {}", offset_x);
            // log!("offset_x {}", offset_x);
            let board = boards[x][y];

            // Render lines
            for i in 1..3 {
                let inner_offset = i as f64 * sq_size;

                // Horizontal
                ctx.move_to(offset_x + 0.0, offset_y + inner_offset);
                ctx.line_to(offset_x + BOARD_SIZE, offset_y + inner_offset);

                // Vertical
                ctx.move_to(offset_x + inner_offset, offset_y + 0.0);
                ctx.line_to(offset_x + inner_offset, offset_y + BOARD_SIZE);
            }

            // Render the noughts and the crosses
            for cell_x in 0..3 {
                for cell_y in 0..3 {
                    let center_x = cell_x as f64 * sq_size + sq_half;
                    let center_y = cell_y as f64 * sq_size + sq_half;
                    let r = sq_size / 5.0;
                    let cell = board.cells[cell_x][cell_y];

                    // log!("cell {:?}", cell);

                    match cell {
                        CellValue::O => {
                            let abc = offset_x + center_x + r;
                            let cba = offset_y + center_y;

                            log!("x {}", abc);
                            log!("y {}", cba);

                            ctx.move_to(offset_x + center_x + r, offset_y + center_y);
                            ctx.arc(
                                offset_x + center_x,
                                offset_y + center_y,
                                r,
                                0.0,
                                f64::consts::PI * 2.0,
                            )
                            .unwrap();
                        }
                        CellValue::X => {
                            ctx.move_to(offset_x + center_x - r, offset_y + center_y - r);
                            ctx.line_to(offset_x + center_x + r, offset_y + center_y + r);
                            ctx.move_to(offset_x + center_x + r, offset_y + center_y - r);
                            ctx.line_to(offset_x + center_x - r, offset_y + center_y + r);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    ctx.set_stroke_style(&JsValue::from("#cccccc"));
    ctx.stroke();
}
