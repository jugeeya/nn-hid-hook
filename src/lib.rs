#![feature(proc_macro_hygiene)]

use skyline::{hook, install_hooks};
use skyline::nn::ro;
use skyline::nn::hid::*;
use skyline::libc::{c_void, c_int, size_t};
use skyline::from_c_str;

use parking_lot::Mutex;

type Callback = fn(*mut skyline::nn::hid::NpadHandheldState,*const u32);

static HOOKS: Mutex<Vec<Callback>> = Mutex::new(Vec::new());

macro_rules! create_nn_hid_hooks {
    (
        $(
            ($func:ident, $hook:ident)
        ),*
    ) => {
        $(
            #[allow(non_snake_case)]
            #[skyline::hook(replace = $func)]
            pub unsafe fn $hook(
                state: *mut skyline::nn::hid::NpadHandheldState,
                controller_id: *const u32,
            ) {
                original!()(state, controller_id);
                for hook in HOOKS.lock().iter() {
                    hook(state, controller_id)
                }
            }
        )*
    };
}

create_nn_hid_hooks!(
    (GetNpadHandheldState, handle_get_npad_handheld_state),
    (GetNpadFullKeyState, handle_get_npad_full_key_state),
    (GetNpadGcState, handle_get_npad_gc_state),
    (GetNpadJoyDualState, handle_get_joy_dual_state),
    (GetNpadJoyLeftState, handle_get_joy_left_state),
    (GetNpadJoyRightState, handle_get_joy_right_state));

#[skyline::main(name = "nn_hid_hook")]
pub fn main() {
    println!("[NN-HID hook] Installing NN-HID hooks...");
    install_hooks!(
        handle_get_npad_handheld_state,
        handle_get_npad_full_key_state,
        handle_get_npad_gc_state,
        handle_get_joy_dual_state,
        handle_get_joy_left_state,
        handle_get_joy_right_state
    );
    println!("[NN-HID hook] NN-HID hooks installed.");
}

#[no_mangle]
pub extern "Rust" fn add_nn_hid_hook(callback: Callback) {
    let mut hooks = HOOKS.lock();

    hooks.push(callback);
}
