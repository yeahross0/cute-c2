use crow::{
    glutin::{
        dpi::{PhysicalSize, Size},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context,
};

use c2::{prelude::*, Poly, Rotation, Transformation, Vec2, AABB};

fn main() -> Result<(), crow::Error> {
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(
        WindowBuilder::new().with_inner_size(Size::Physical(PhysicalSize::new(800, 600))),
        &event_loop,
    )?;

    let aabb = AABB::new([250.0, 250.0], [550.0, 350.0]);

    let poly = Poly::from_slice(&[
        [0.0, -100.0],
        [100.0, 0.0],
        [50.0, 100.0],
        [-50.0, 100.0],
        [-100.0, 0.0],
    ]);

    let mut transformation = Transformation::new([400.0, 500.0], Rotation::zero());
    let mut rotation = 0.0;

    let mut velocity = Vec2::new(0.2, 0.2);

    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                rotation += 0.001;
                transformation.set_position(Vec2::new(
                    transformation.position().x() + velocity.x(),
                    transformation.position().y() + velocity.y(),
                ));
                transformation.set_rotation(Rotation::radians(rotation));
                if transformation.position().x() < 100.0 || transformation.position().x() > 700.0 {
                    velocity.set_x(-velocity.x());
                }
                if transformation.position().y() < 100.0 || transformation.position().y() > 500.0 {
                    velocity.set_y(-velocity.y());
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
                    (aabb.min().x() as i32, aabb.min().y() as i32),
                    (aabb.max().x() as i32, aabb.max().y() as i32),
                    colour,
                );
                let transform = |mut vert: c2::Vec2| {
                    let x = vert.x();
                    let y = vert.y();
                    let c = transformation.rotation().cos();
                    let s = transformation.rotation().sin();
                    vert = Vec2::new(x * c + y * s, x * -s + y * c);
                    let transformed_vert = Vec2::new(
                        vert.x() + transformation.position().x(),
                        vert.y() + transformation.position().y(),
                    );
                    (transformed_vert.x() as i32, transformed_vert.y() as i32)
                };
                for i in 0..(poly.count()) {
                    let from = transform(poly.get_vert(i));
                    let next = (i + 1) % poly.count();
                    let to = transform(poly.get_vert(next));
                    ctx.debug_line(&mut surface, from, to, colour);
                }
                ctx.present(surface).unwrap();
            }
            _ => (),
        },
    )
}
