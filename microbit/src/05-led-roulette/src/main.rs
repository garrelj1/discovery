#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let pattern = [
        (0, 0), (0, 1), (0, 2), (0, 3),
        (0, 4), (1, 4), (2, 4), (3, 4),
        (4, 4), (4, 3), (4, 2), (4, 1),
        (4, 0), (3, 0), (2, 0), (1, 0)
    ];

    loop {
        for coordinates in pattern {
            let board_lights = board_with_led_lit_at(coordinates.0, coordinates.1);
            display.show(&mut timer, board_lights, 30);
            timer.delay_ms(10_u32);
        }
    }
}

fn board_with_led_lit_at(row: usize, column: usize) -> [[u8; 5]; 5] {
    let mut board = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    board[row][column] = 1;

    return board
}