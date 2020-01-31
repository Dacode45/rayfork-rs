use glutin::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    Api, ContextBuilder, GlProfile, GlRequest, NotCurrent, PossiblyCurrent, WindowedContext,
};
use std::mem::MaybeUninit;

use crate::ffi;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// This token is used to ensure certain functions are only running on the same
/// thread Rayfork was initialized from. This is useful for architectures like macos
/// where cocoa can only be called from one thread.
#[derive(Clone, Debug)]
pub struct RayforkThread(PhantomData<*const ()>);

/// The main interface into the Rayfork API.
///
/// This is the way in which you will use the vast majority of Rayfork's functionality. A `RayforkHandle` can be constructed using the [`init_window`] function or through a [`RayforkBuilder`] obtained with the [`init`] function.
///
/// [`init_window`]: fn.init_window.html
/// [`RayforkBuilder`]: struct.RayforkBuilder.html
/// [`init`]: fn.init.html
// inner field is private, preventing manual construction
pub struct RayforkHandle {
    rf_ctx: ffi::rf_context,
    window_ctx: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
}

impl Drop for RayforkHandle {
    fn drop(&mut self) {
        IS_INITIALIZED.swap(false, Ordering::Release);
    }
}

#[derive(Debug, Default)]
pub struct RayforkBuilder {}

impl RayforkBuilder {
    pub fn new() -> RayforkBuilder {
        RayforkBuilder {}
    }

    pub fn build<F>(&self, mut frame: F) -> !
    where
        F: FnMut(
                glutin::event::Event<()>,
                &glutin::event_loop::EventLoopWindowTarget<()>,
                &mut glutin::event_loop::ControlFlow,
                &mut RayforkHandle,
                &RayforkThread,
            ) + 'static,
    {
        // Prevent this from happening twice
        if IS_INITIALIZED.compare_and_swap(false, true, Ordering::Relaxed) {
            panic!("Attempted to initialize raylib-rs more than once");
        }
        // Setup glutin
        let (event_loop, windowed_context, _shader_version) = {
            let el = glutin::event_loop::EventLoop::new();
            let wb = glutin::window::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let windowed_context = glutin::ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
                .with_gl_profile(GlProfile::Core)
                .with_multisampling(0 as u16)
                .with_double_buffer(Some(true))
                .build_windowed(wb, &el)
                .unwrap();
            let windowed_context = unsafe { windowed_context.make_current().unwrap() };

            let result = ffi::loading::loadGL(|s| {
                windowed_context.get_proc_address(s) as *mut std::ffi::c_void
            });
            println!("result: {}", result);

            (el, windowed_context, "#version 330")
        };

        // Setup rayfork
        let mut rf_ctx = MaybeUninit::uninit();
        let size = windowed_context.window().inner_size();
        let rf_ctx = unsafe {
            ffi::rf_context_init(rf_ctx.as_mut_ptr(), size.width as i32, size.height as i32);
            rf_ctx.assume_init()
        };

        let mut rf = RayforkHandle {
            rf_ctx: rf_ctx,
            window_ctx: windowed_context,
        };
        let thread = RayforkThread(PhantomData);
        event_loop.run(move |event, target, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                // If we close the window or press escape, quit the main loop (i.e. quit the application).
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                _ => (),
            }
            frame(event, target, control_flow, &mut rf, &thread);
        });
    }
}

impl RayforkHandle {
    pub fn swap_buffers(&mut self) {
        self.window_ctx.swap_buffers().expect("failed swap");
    }
}
