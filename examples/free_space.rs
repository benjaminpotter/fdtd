use fdtd::mesh::MeshBuilder;
use fdtd::simulation::SimulationBuilder;

fn main() {
    let mesh_builder = MeshBuilder::new().with_dx(1.).with_dy(1.);

    let sim_builder = SimulationBuilder::new()
        .with_steps(1000)
        .with_sample_rate(10)
        .with_mesh_builder(mesh_builder);

    let sim = sim_builder.build().unwrap();
    sim.start().unwrap();
}
