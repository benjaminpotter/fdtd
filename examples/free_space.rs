use fdtd::mesh::MeshBuilder;
use fdtd::simulation::SimulationBuilder;

fn main() {
    let mesh_builder = MeshBuilder::new()
        .with_dx(1e-3)
        .with_dy(1e-3)
        .with_width(500)
        .with_height(500)
        .with_point_source(100, 100, 20.0, 10.0)
        .with_point_source(400, 400, 20.0, 10.0);

    let sim_builder = SimulationBuilder::new()
        .with_steps(1000)
        .with_sample_rate(10)
        .with_mesh_builder(mesh_builder);

    let sim = sim_builder.build().unwrap();
    sim.start().unwrap();
}
