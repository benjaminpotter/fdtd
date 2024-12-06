use crate::error::Error;
use crate::mesh::{Mesh, MeshBuilder};

pub struct Simulation {
    steps: usize,
    sample_rate: usize,
    mesh: Mesh,
}

impl Simulation {
    pub fn start(mut self) {
        for i in 0..self.steps {
            self.mesh.step();
            println!("-- step {:010}", i);

            // Skip for now.
            // if i % sampling_period == 0 {
            //     let filename = format!["data/{}/{:010}.efld", timestamp, i];
            //     let mut fd = fs::File::create(filename).expect("failed to create efld file");
            //     mesh.serialize(&mut fd)
            //         .expect("failed to write data to efld file");

            //     println!("-- wrote {:010}", i);
            // }
        }
    }
}

pub struct SimulationBuilder {
    steps: usize,
    sample_rate: usize,
    mesh_builder: Option<MeshBuilder>,
}

impl SimulationBuilder {
    pub fn new() -> Self {
        SimulationBuilder {
            steps: 100,
            sample_rate: 10,
            mesh_builder: None,
        }
    }

    pub fn with_steps(mut self, steps: usize) -> Self {
        self.steps = steps;
        self
    }

    pub fn with_sample_rate(mut self, sample_rate: usize) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn with_mesh_builder(mut self, mesh_builder: MeshBuilder) -> Self {
        self.mesh_builder = Some(mesh_builder);
        self
    }

    pub fn build(self) -> Result<Simulation, Error> {
        let mesh = self
            .mesh_builder
            .ok_or("no mesh")
            .map_err(|e| Error::BuilderError(e.to_string()))?
            .build();

        Ok(Simulation {
            steps: self.steps,
            sample_rate: self.sample_rate,
            mesh,
        })
    }
}
