use std::collections::HashMap;

#[derive(Clone, Debug)]
struct CriticalPath {
    vertices: Vec<u32>,
    weight: u32,
}
impl CriticalPath {
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
    times: Vec<u32>,
    start_vertices: Vec<u32>,
}
impl Graph {
    pub const fn new(data: Vec<Vec<u32>>, times: Vec<u32>, start_vertices: Vec<u32>) -> Self {
        Graph {
            data,
            times,
            start_vertices,
        }
    }

    pub fn from_file(_filename: &str) {
        println!(":TODO");
    }

    pub fn v_count(&self) -> u32 {
        let mut count: u32 = 0;
        for v in self.times.clone() {
            if v > 0 {
                count += 1;
            }
        }
        count
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
        // println!("{:?}", paths);
        let mut weight = 0;
        let mut path = CriticalPath::new(0, 0);
        for p in paths {
            if p.weight > weight {
                weight = p.weight;
                path = p;
            }
        }
        path
    }

    pub fn find_critical_path(&self) -> CriticalPath {
        let mut paths = HashMap::new();
        for v in self.start_vertices.clone() {
            paths.insert(v, CriticalPath::new(v, self.times[v as usize]));
        }
        let mut vertices = self.start_vertices.clone();
        println!("{:?}", vertices.clone());
        while vertices.len() > 0 {
            let mut next_level_vertices = Vec::new();
            let mut next_level_paths: HashMap<u32, CriticalPath> = HashMap::new();
            // println!("nlv: {:?}", vertices.clone());
            // println!("nlp: {:?}", paths.clone());
            for index in vertices.clone() {
                let next_vertices = self.get_next_vertices(index as u32);
                // println!("{} next {:?}", index, next_vertices.clone());
                if next_vertices.len() == 0 {
                    next_level_vertices = vertices;
                    next_level_vertices.remove(0);
                    next_level_paths = paths;
                    break;
                }
                let cur_path = match paths.get(&(index as u32)) {
                    Some(path) => path,
                    None => panic!("No path for vertex {}", index),
                };
                for v in next_vertices {
                    let mut new_path = cur_path.clone();
                    new_path.add_vertex(v, self.data[index as usize][v as usize] + self.times[v as usize]);
                    next_level_vertices.push(v);
                    match next_level_paths.get_mut(&v) {
                        Some(cp) => {
                            if cp.weight <= new_path.weight {
                                next_level_paths.insert(v, new_path);
                            }
                        },
                        None => {
                            next_level_paths.insert(v, new_path);
                        },
                    };
                }
            }
            vertices = next_level_vertices;
            paths = next_level_paths;
        }
        // println!("{:?}", paths);
        self.max(paths.values().cloned().collect()).clone()
    }

    pub fn delete_vertices(&mut self, mut vertices: Vec<u32>) {
        // vertices.remove(0);
        let mut index = 0;
        for i in 0..self.start_vertices.len() {
            if self.start_vertices[i] == vertices[0] {
                index = i;
            }
        }
        if index < self.start_vertices.len() {
            self.start_vertices.remove(index);
        }
        for vertex in vertices {
            for i in 0..self.data.len() {
                self.data[i][vertex as usize] = 0;
                self.data[vertex as usize][i] = 0;
            }
            self.times[vertex as usize] = 0;
        }
    }
}

fn main() {
    println!("Good luck :)");
    let mut graph = Graph::new(
        vec![
    //       0  1  2  3  4  5  6  7  8  9 10 11 12 13 14
        vec![0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 2, 1, 3, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        vec![2, 3, 3, 1, 2, 1, 2, 4, 1, 2, 3, 2, 3, 1, 2],
        vec![0, 1, 2, 3, 4, 5]
        );
    // while graph.v_count() > 0  {
    for _i in 0..7 {
        println!("v_count: {}", graph.v_count());
        let path = graph.find_critical_path();
        println!("path: {:?}", path.clone());
        graph.delete_vertices(path.vertices);
    }
}
