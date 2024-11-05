use chrono::Utc;
use fdtd::Mesh;
use std::fs;

fn main() {
    let timestamp = Utc::now().format("%y-%m-%d-%H-%M-%S").to_string();
    fs::create_dir(format!["data/{}", timestamp]).expect("failed to create data directory");

    let mut mesh = Mesh::new();
    for i in 0..10 {
        for _ in 0..10 {
            mesh.step();
        }

        let filename = format!["data/{}/{:010}.efld", timestamp, i * 10];
        let mut fd = fs::File::create(filename).expect("failed to create efld file");
        mesh.serialize(&mut fd)
            .expect("failed to write data to efld file");
    }
}
