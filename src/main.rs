mod graphics;

use std::os::raw::c_void;

use gl::{
    self, BindVertexArray, DrawElements, FALSE, FLOAT, TRIANGLES, UNSIGNED_INT, types::GLfloat,
};
use glfw::{Action, Context, GlfwReceiver, Key, fail_on_errors};
use graphics::Shader;

use crate::graphics::Drawer;

#[macro_export]
macro_rules! NULL {
    ($a:tt) => {
        std::ptr::null::<$a>()
    };
    () => {
        std::ptr::null::<std::os::raw::c_void>()
    };
}

pub const VERTEX_LAYOUT: (u32, i32, u32, u8, i32, *const c_void) = (
    0,
    3,
    FLOAT,
    FALSE,
    7 * size_of::<GLfloat>() as i32,
    std::ptr::null::<std::os::raw::c_void>(),
);
pub const COLOR_LAYOUT: (u32, i32, u32, u8, i32, *const c_void) = (
    1,
    4,
    FLOAT,
    FALSE,
    7 * size_of::<GLfloat>() as i32,
    (3 * size_of::<GLfloat>() as i32) as *const c_void,
);

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    // glfw: initialize and configure
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol).unwrap() as *const _);

    let test_vertices = vec![
        -0.5, -0.5, 0.0,   1.0, 0.0, 0.0, 1.0, // 0
        0.5, -0.5, 0.0,    1.0, 0.0, 0.0, 1.0, // 1
        0.0, 0.5, 0.0,     0.0, 0.0, 1.0, 1.0, // 2
        -0.25, 0.0, 0.0,   0.5, 0.0, 0.5, 1.0, // 3
        0.0, -0.5, 0.0,    1.0, 0.0, 0.0, 1.0, // 4
        0.25, 0.0, 0.0,    0.5, 0.0, 0.5, 1.0, // 5
    ];

    /*
               2


           3       5



       0       4       1

    */

    let test_indices = [0, 3, 4, 3, 2, 5, 4, 5, 1];

    let basic_shader = Shader::init("default.vert", "default.frag");
    let drawer = Drawer::init(&test_vertices, &test_indices);

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        basic_shader.activate();
        unsafe {
            BindVertexArray(drawer.vao);
            DrawElements(TRIANGLES, 9, UNSIGNED_INT, NULL!(c_void));
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
