#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod nadk;

use crate::nadk::display::{Color565, SCREEN_RECT, ScreenPoint, draw_string, push_rect_uniform};
use crate::nadk::keyboard::{Key, is_key_down};
use crate::nadk::storage::{CalculatorModel, get_calculator_model};
use crate::nadk::utils::wait_ok_released;
use crate::nadk::time::{get_current_time_millis};

// C string + null terminator
configure_app!(b"Descartes\0", 10, "../target/icon.nwi", 4811);

setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    // You must call setup_allocator!() before
    init_heap!();
    wait_ok_released();

    push_rect_uniform(SCREEN_RECT, Color565::from_rgb888(255, 255, 255));
    draw_string(
        "Let there be usefulness...",
        ScreenPoint::new(20, 20),
        true,
        Color565::from_rgb888(0, 0, 0),
        Color565::new(255, 255, 255),
    );
    match get_calculator_model() {
        CalculatorModel::Upsilon => draw_string(
            "You're using Upsilon!",
            ScreenPoint::new(20, 50),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        ),
        CalculatorModel::Simulator => draw_string(
            "You're using a Simulator!",
            ScreenPoint::new(20, 50),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        ),
        _ => draw_string(
            "You're using Epsilon!",
            ScreenPoint::new(20, 50),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        ),
    };
    while !is_key_down(Key::Ok) {
        let mut buf = [0u8; 20];
        let time_str = format_u64(get_current_time_millis(), &mut buf);
       draw_string(
            time_str,
            ScreenPoint::new(20,80),
            true,
            Color565::from_rgb888(0, 0, 0),
            Color565::new(255, 255, 255),
        );
    }
}

fn format_u64(mut num: u64, buf: &mut [u8]) -> &str {
    let mut i = buf.len();
    if num == 0 {
        buf[i - 1] = b'0';
        return unsafe { core::str::from_utf8_unchecked(&buf[i - 1..]) };
    }
    while num > 0 && i > 0 {
        i -= 1;
        buf[i] = (b'0' + (num % 10) as u8);
        num /= 10;
    }
    unsafe { core::str::from_utf8_unchecked(&buf[i..]) }
}
