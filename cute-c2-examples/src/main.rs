use crow::{
    glutin::{
        dpi::{PhysicalSize, Size},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context,
};

use cute_c2 as c2;
use cute_c2::detection::*;

fn main() -> Result<(), crow::Error> {
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(
        WindowBuilder::new().with_inner_size(Size::Physical(PhysicalSize::new(800, 600))),
        &event_loop,
    )?;

    let aabb = c2::AABB::new(c2::Vec2::new(250.0, 250.0), c2::Vec2::new(550.0, 350.0));

    let poly = c2::Poly::from_slice(&[
        c2::Vec2::new(0.0, -100.0),
        c2::Vec2::new(100.0, 0.0),
        c2::Vec2::new(50.0, 100.0),
        c2::Vec2::new(-50.0, 100.0),
        c2::Vec2::new(-100.0, 0.0),
    ]);

    let mut transformation = c2::Transformation::new(c2::Vec2::new(400.0, 500.0), 0.0);
    let mut rotation = 0.0;

    let mut velocity = (0.2, 0.2);

    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                rotation += 0.001;
                transformation.r = c2::Rotation::new(rotation);
                transformation.p.x += velocity.0;
                transformation.p.y += velocity.1;
                if transformation.p.x < 100.0 || transformation.p.x > 700.0 {
                    velocity.0 *= -1.0;
                }
                if transformation.p.y < 100.0 || transformation.p.y > 500.0 {
                    velocity.1 *= -1.0;
                }
                let mut surface = ctx.surface();
                ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0));
                let colour = if aabb.collides_with(&(poly, transformation)) {
                    (1.0, 0.0, 0.0, 1.0)
                } else {
                    (1.0, 1.0, 1.0, 1.0)
                };
                ctx.debug_rectangle(
                    &mut surface,
                    (aabb.min.x as i32, aabb.min.y as i32),
                    (aabb.max.x as i32, aabb.max.y as i32),
                    colour,
                );
                let transform = |mut vert: c2::Vec2| {
                    let x = vert.x;
                    let y = vert.y;
                    let c = transformation.r.c;
                    let s = transformation.r.s;
                    vert.x = x * c + y * s;
                    vert.y = x * -s + y * c;
                    (
                        (vert.x + transformation.p.x) as i32,
                        (vert.y + transformation.p.y) as i32,
                    )
                };
                for i in 0..(poly.count as usize) {
                    let from = transform(poly.verts[i]);
                    let next = (i + 1) % poly.count as usize;
                    let to = transform(poly.verts[next]);
                    ctx.debug_line(&mut surface, from, to, colour);
                }
                ctx.present(surface).unwrap();
            }
            _ => (),
        },
    )
}
