use serde::Deserialize;

#[derive(Deserialize)]
pub struct NavmeshDump {
    pub stairs: MeshData,
}

#[derive(Deserialize)]
pub struct MeshData {
    pub vertices: Vec<[f32; 2]>,
    pub triangles: Vec<[usize; 3]>,
    pub heights: Vec<f32>,
}

pub fn load_dump(path: &str) -> NavmeshDump {
    let file = std::fs::File::open(path).expect("Failed to open navmesh dump file");
    serde_json::from_reader(file).expect("Failed to parse navmesh dump JSON")
}
