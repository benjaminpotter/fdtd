use chrono::Utc;
use fdtd::Mesh;
use std::fs;

fn main() {
    let timestamp = Utc::now().format("%y-%m-%d-%H-%M-%S").to_string();
    fs::create_dir(format!["data/{}", timestamp]).expect("failed to create data directory");

    let mut mesh = Mesh::new();

    let max_steps = 1000;
    let sampling_period = 10;

    println!("-- starting simulation");
    println!("-- timestamp:  {:?}", timestamp);
    println!("-- max_steps:  {:?}", max_steps);
    println!("-- spl_period: {:?}", sampling_period);

    println!("-- begin simulation");
    for i in 0..max_steps {
        mesh.step();

        if i % sampling_period == 0 {
            let filename = format!["data/{}/{:010}.efld", timestamp, i];
            let mut fd = fs::File::create(filename).expect("failed to create efld file");
            mesh.serialize(&mut fd)
                .expect("failed to write data to efld file");

            println!("-- wrote {:010}", i);
        }
    }

    println!("-- exiting");
}
