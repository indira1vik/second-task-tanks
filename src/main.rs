use three_d::renderer::Gm;
use three_d::*;

#[allow(unused_imports)]
use self::cuboid::cuboid;
#[allow(unused_imports)]
use self::cube::cube;

#[path = "./tank/cuboid.rs"]
mod cuboid;

#[path = "./tank/cube.rs"]
mod cube;

#[allow(dead_code)]
pub fn main() {

    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Trial".to_string(),
        max_size: Some((480, 480)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 1.75, 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let fl_ht = 1.85;
    let choose = if fl_ht >= 1.8 {
        Color::new(255,0,0,200)
    } else if fl_ht >= 1.0 {
        Color::new(255,165,0,200)
    } else {
        Color::new(0,255,0,200)
    };

    // let mesh0 = cube(2.0, 2.0);
    // let mesh1 = cube(fl_ht, 2.0);

    let mesh0 = cuboid(2.0, 1.0, 3.0);
    let mesh1 = cuboid(fl_ht, 1.0, 3.0);

    let mut one_mesh = Gm::new(
        Mesh::new(&context, &mesh0),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 100,
                },
                ..Default::default()
            },
        ),
    );
    let mut two_mesh = Gm::new(
        Mesh::new(&context, &mesh1),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: choose,
                ..Default::default()
            },
        ),
    );

    one_mesh.set_animation(|time| Mat4::from_angle_y(radians(time * 0.0002)));
    one_mesh.set_transformation(Mat4::from_scale(1.0));
    two_mesh.set_animation(|time| Mat4::from_angle_y(radians(time * 0.0002)));
    two_mesh.set_transformation(
        Mat4::from_translation(vec3(0.0, -1.0 + fl_ht / 2.0, 0.0)) * Mat4::from_scale(1.0),
    );

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(1.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    // Start the main render loop
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Animation
        one_mesh.animate(frame_input.accumulated_time as f32);
        two_mesh.animate(frame_input.accumulated_time as f32);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                one_mesh.into_iter().chain(&two_mesh),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}
