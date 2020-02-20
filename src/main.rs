extern crate glium;

struct Window {
    height: isize,
    width: isize,
    display: glium::Display,
}

impl Window {
    fn new(w: isize, h: isize, d: glium::Display) -> Window {
        Window {
            height: h,
            width: w,
            display: d,
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

fn main() {
    //make window
    use glium::{glutin, Surface};
    let mut event_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);

    let window = Window::new(
        720,
        720,
        glium::Display::new(wb.with_title("Convergence"), cb, &event_loop).unwrap(),
    );

    //make quad to draw fractals on
    glium::implement_vertex!(Vertex, position);
    let vertex1 = Vertex {
        position: [-1.0, -1.0],
    };
    let vertex2 = Vertex {
        position: [-1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [1.0, 1.0],
    };
    let vertex4 = Vertex {
        position: [1.0, -1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&window.display, &shape).unwrap();
    let palate = vec![(0.0f32, 0.0, 1.0), (0.0, 1.0, 0.0), (0.0, 0.0, 0.0)];
    let texture =
        glium::texture::srgb_texture1d::SrgbTexture1d::new(&window.display, palate).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        out vec2 Pos;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            Pos = position;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 Pos;
        out vec4 color;
        uniform sampler1D palate;

        float random (vec2 st) {
            return fract(sin(dot(st.xy, vec2(12.9898,78.233)))* 43758.5453123);
        }

        float map(float x, float in_min, float in_max, float out_min, float out_max)
        {
            return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
        }

        void main() {
            //scale pos to fit frame
            float x0 = map(Pos.x, -1.0, 1.0, -2.5, 1.0);
            float y0 = map(Pos.y, -1.0, 1.0, -1.0, 1.0);

            float x = 0;
            float y = 0;
            int iteration = 0;
            int max_iteration = 10000;
            while(x * x + y * y <= 2 * 2 && iteration < max_iteration) {
                float x_temp = x*x - y*y + x0;
                y = 2*x*y + y0;
                x = x_temp;
                iteration++;
            }

            color = texture(palate, float(iteration) / float(max_iteration));
            //color = vec4(float(iteration) / float(max_iteration), float(iteration) / float(max_iteration), float(iteration) / float(max_iteration), 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &window.display,
        vertex_shader_src,
        fragment_shader_src,
        None,
    )
    .unwrap();

    //main loop
    let mut closed = false;
    while !closed {
        event_loop.poll_events(|ev| {
            // Handle events ourself.
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    // Handle window close event.
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        let mut target = window.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target
            .draw(
                &vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                &program,
                &glium::uniform! {palate: texture.sampled().minify_filter(glium::uniforms::MinifySamplerFilter::Linear)},
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    println!("Hello, world!");
}
