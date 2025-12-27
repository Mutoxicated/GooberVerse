use std::os::raw::c_void;

use gl::{ARRAY_BUFFER, AttachShader, BindBuffer, BindVertexArray, BufferData, CompileShader, CreateProgram, CreateShader, DeleteBuffers, DeleteProgram, DeleteShader, DeleteVertexArrays, ELEMENT_ARRAY_BUFFER, EnableVertexAttribArray, FALSE, FLOAT, FRAGMENT_SHADER, GenBuffers, GenVertexArrays, LinkProgram, STATIC_DRAW, ShaderSource, UseProgram, VERTEX_SHADER, VertexAttribPointer, types::*};

use crate::NULL;

pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe{ DeleteProgram(self.id); }
    }
}

impl Shader {
    pub fn init(vertex_file:&str, fragment_file:&str) -> Self {
        let cwd = std::env::current_dir().unwrap().to_string_lossy().to_string()+"/resources/shaders/";
        let vertex_source = std::fs::read_to_string(cwd.clone()+vertex_file)
            .unwrap();
        let fragment_source = std::fs::read_to_string(cwd.clone()+fragment_file)
            .unwrap();
        unsafe{ 
            let vertex_shader:GLuint = CreateShader(VERTEX_SHADER);
            let c_string = vertex_source.as_ptr() as *const i8;
            ShaderSource(vertex_shader, 1, &c_string as *const *const i8, NULL!(i32));
            CompileShader(vertex_shader);

            let fragment_shader:GLuint = CreateShader(FRAGMENT_SHADER);
            let c_string = fragment_source.as_ptr() as *const i8;
            ShaderSource(fragment_shader, 1, &c_string as *const *const i8, NULL!(i32));
            CompileShader(fragment_shader);

            let id: GLuint = CreateProgram();
            AttachShader(id, vertex_shader);
            AttachShader(id, fragment_shader);

            LinkProgram(id);

            DeleteShader(vertex_shader);
            DeleteShader(fragment_shader);

            Self {
                id,
            }
        }
    }

    pub fn activate(&self) {
        unsafe{ UseProgram(self.id); }
    }
}

pub struct Drawer {
    pub vao: GLuint,
    vbo: GLuint,
    ebo: GLuint
}

impl Drawer {
    pub fn init(vertices:Vec<GLfloat>, indices:Vec<GLint>) -> Self {
        let vertices = vertices.as_slice();
        let indices = indices.as_slice();
        let mut instance = Self{
            vao:0,
            vbo:0,
            ebo:0,
        };

        unsafe {
            GenVertexArrays(1, &mut instance.vao);
            GenBuffers(1, &mut instance.vbo);
            GenBuffers(1, &mut instance.ebo);

            BindVertexArray(instance.vao);
            BindBuffer(ARRAY_BUFFER, instance.vbo);

            BufferData(ARRAY_BUFFER, 
                size_of_val(vertices).try_into().unwrap(), 
                vertices.as_ptr() as *const c_void, 
                STATIC_DRAW);
            BindBuffer(ELEMENT_ARRAY_BUFFER, instance.ebo);
            BufferData(ELEMENT_ARRAY_BUFFER, 
                size_of_val(indices).try_into().unwrap(), 
                indices.as_ptr() as *const c_void, 
                STATIC_DRAW);

            VertexAttribPointer(0, 3, FLOAT, FALSE, 
                0, 
                NULL!());
            EnableVertexAttribArray(0);

            BindBuffer(ARRAY_BUFFER, 0);
            BindVertexArray(0);
            BindBuffer(ELEMENT_ARRAY_BUFFER, 0);
        }

        instance
    }
}

impl Drop for Drawer {
    fn drop(&mut self) {
        unsafe{
            DeleteBuffers(1, &self.ebo);
            DeleteBuffers(1, &self.vbo);
            DeleteVertexArrays(1, &self.vao);
        }
    }
}