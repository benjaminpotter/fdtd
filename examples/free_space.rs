use chrono::Utc;
use fdtd::mesh::MeshBuilder;
use fdtd::simulation::SimulationBuilder;
use std::fs;

fn main() {
    let timestamp = Utc::now().format("%y-%m-%d-%H-%M-%S").to_string();
    fs::create_dir(format!["data/{}", timestamp]).expect("failed to create data directory");

    let mesh_builder = MeshBuilder::new().with_dx(1.).with_dy(1.);

    let sim_builder = SimulationBuilder::new()
        .with_steps(1000)
        .with_sample_rate(10)
        .with_mesh_builder(mesh_builder);

    println!("-- starting simulation");
    println!("-- timestamp:   {:?}", timestamp);
    println!("-- steps:       {:?}", 1000);
    println!("-- sample_rate: {:?}", 10);

    println!("-- begin simulation");
    let sim = sim_builder.build().unwrap();
    sim.start();

    println!("-- exiting");
}
