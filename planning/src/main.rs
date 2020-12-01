
#[derive(Clone, Debug)]
struct CriticalPath {
    vertices: Vec<u32>,
    weight: u32,
}
impl CriticalPath {
    /*
    pub const fn empty() -> Self {
        CriticalPath {
            vertices: Vec::new(),
            weight: 0,
        }
    }
    */

    pub fn new(vertex: u32, weight: u32) -> Self {
        let mut v = Vec::new();
        v.push(vertex);
        CriticalPath {
            vertices: v,
            weight,
        }
    }

    pub fn add_vertex(&mut self, index: u32, weight: u32) {
        self.vertices.push(index);
        self.weight += weight;
    }
}

struct Graph {
    data: Vec<Vec<u32>>,
    start_vertices: Vec<u32>,
}
impl Graph {
    pub const fn new(data: Vec<Vec<u32>>, start_vertices: Vec<u32>) -> Self {
        Graph {
            data,
            start_vertices,
        }
    }

    pub fn from_file(_filename: &str) {
        println!(":TODO");
    }

    fn get_next_vertices(&self, index: u32) -> Vec<u32> {
        let mut next_vertices = Vec::new();
        let vertices = self.data[index as usize].clone();
        for i in 0..vertices.len() {
            if vertices[i] > 0 {
                next_vertices.push(i as u32);
            }
        }
        next_vertices
    }

    fn max(&self, paths: Vec<CriticalPath>) -> CriticalPath {
        let mut weight = 0;
        let mut index = 0;
        for i in 0..paths.len() {
            if paths[i].weight > weight {
                weight = paths[i].weight;
                index = i;
            }
        }
        paths[index].clone()
    }

    pub fn find_critical_path(&self) -> CriticalPath {
        let mut paths = Vec::new();
        for v in self.start_vertices.clone() {
            paths.push(CriticalPath::new(v, 0));
        }
        let mut vertices = self.start_vertices.clone();
        while vertices.len() > 0 {
            let mut next_level_vertices = Vec::new();
            let mut next_level_paths = Vec::new();
            for index in 0..vertices.len() {
                let next_vertices = self.get_next_vertices(index as u32);
                let cur_path = &paths[index];
                for v in next_vertices {
                    let mut new_path = cur_path.clone();
                    new_path.add_vertex(v, self.data[index][v as usize]);
                    next_level_paths.push(new_path);
                    next_level_vertices.push(v);
                }
            }
            vertices = next_level_vertices;
            paths = next_level_paths;
        }
        self.max(paths).clone()
    }
}

fn main() {
    println!("Good luck :)");
}
