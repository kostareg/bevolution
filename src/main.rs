use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

const INPUTS_N: usize = 3;
const INTERMEDIATES_N: usize = 10;
const OUTPUTS_N: usize = 3;

#[derive(Copy, Clone, Debug)]
enum Neuron {
    Input(usize),
    Intermediate(usize),
    Output(usize),
}

#[derive(Copy, Clone, Debug)]
struct Connection {
    from: Neuron,
    to: Neuron,
    weight: f32,
}

impl Connection {
    fn random() -> Self {
        let inp_rng = || rand::random_range(0..INPUTS_N);
        let int_rng = || rand::random_range(0..INTERMEDIATES_N);
        let out_rng = || rand::random_range(0..OUTPUTS_N);

        let from = *[Neuron::Input(inp_rng()), Neuron::Intermediate(int_rng())].choose(&mut rand::rng()).unwrap();
        let to = *[Neuron::Intermediate(int_rng()), Neuron::Output(out_rng())].choose(&mut rand::rng()).unwrap();

        Self {
            from,
            to,
            weight: rand::random_range(-10. .. 10.),
        }
    }
}

#[derive(Debug)]
struct NeuralNetwork {
    connections: [Connection; 8],
}

impl NeuralNetwork {
    fn random() -> Self {
        Self {
            connections: std::array::from_fn(|_| Connection::random()),
        }
    }
}

#[derive(Component, Debug)]
struct Blob {
    network: NeuralNetwork,
    internal_state: [f32; INTERMEDIATES_N],
}

impl Blob {
    fn random() -> Self {
        Self {
            network: NeuralNetwork::random(),
            internal_state: std::array::from_fn(|_| rand::random_range(0. .. 0.5)),
        }
    }

    fn step(&mut self, force: Vec3) -> Vec3 {
        let inputs = [force.x, force.y, force.z];
        let mut result = force;
        
        for connection in self.network.connections {
            let value = connection.weight * match connection.from {
                Neuron::Input(n) => inputs[n],
                Neuron::Intermediate(n) => self.internal_state[n],
                Neuron::Output(_) => unimplemented!(),
            };

            match connection.to {
                Neuron::Input(_) => unimplemented!(),
                Neuron::Intermediate(n) => self.internal_state[n] = value,
                Neuron::Output(n) => match n {
                    0 => result.x = value / 100.,
                    1 => result.y = value / 100.,
                    2 => result.z = value / 100.,
                    _ => unimplemented!(),
                },
            }
        }

        result.clamp(Vec3 {
            x: -1.,
            y: -1.,
            z: -1.,
        }, Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        })
    }
}

fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hx = 3.;
    let hy = 3.;
    let hz = 3.;

    let verts = vec![
        Vec3::new(-hx, -hy, -hz),
        Vec3::new( hx, -hy, -hz),
        Vec3::new( hx,  hy, -hz),
        Vec3::new(-hx,  hy, -hz),
        Vec3::new(-hx, -hy,  hz),
        Vec3::new( hx, -hy,  hz),
        Vec3::new( hx,  hy,  hz),
        Vec3::new(-hx,  hy,  hz),
    ];

    let indices = vec![
        [0,1,2], [0,2,3],
        [4,6,5], [4,7,6],
        [0,4,5], [0,5,1],
        [3,2,6], [3,6,7],
        [1,5,6], [1,6,2],
        [0,3,7], [0,7,4],
    ];

    commands.spawn((
        RigidBody::Fixed,
        Collider::trimesh(verts, indices).unwrap(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(3.)))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 144, 255))),
        Transform::from_xyz(0., -3., 0.),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-9., 9., 9.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_blobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let max_x = 25;
    let max_z = 25;

    for i in 0..(max_x * max_z * 5) {
        commands.spawn((
            Blob::random(),
            Collider::cuboid(0.06, 0.06, 0.06),
            RigidBody::Dynamic,
            GravityScale(0.),
            ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO },
            Restitution::coefficient(0.7),
            Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz((i % max_x) as f32 / 5. - 2.5, ((i / (max_x * max_z)) as f32) / 5. - 2.5, (((i / max_x) as f32) % (max_z as f32)) / 5. - 2.5),
        ));
    }
}

fn step(
    mut query: Query<(&mut Blob, &mut ExternalForce)>
) {
    for (mut blob, mut ext_force) in query.iter_mut() {
        ext_force.force = blob.step(ext_force.force);
    }
}

fn ui_example_system(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label(egui::RichText::new("world").size(10.));
    });
    Ok(())
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (spawn_environment, spawn_blobs))
        .add_systems(FixedUpdate, step)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}
