use rayfork::ffi;
use rayfork::prelude::*;

fn main() {
    let builder = RayforkBuilder::new().build(|_, _, _, rf, rt| {
        unsafe {
            ffi::rf_begin_drawing();
            ffi::rf_clear_background(ffi::rf_color {
                r: 0,
                g: 254,
                b: 254,
                a: 254,
            });
            ffi::rf_draw_circle(
                800 / 4,
                120,
                35.0,
                ffi::rf_color {
                    r: 0,
                    g: 0,
                    b: 254,
                    a: 254,
                },
            );
            ffi::rf_end_drawing();
        }
        rf.swap_buffers();
    });
}
