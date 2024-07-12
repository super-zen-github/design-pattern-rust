/// Composite design pattern [structure]

pub trait Node {
    fn get_path(&self);
    fn number_of_files(&self) -> i64;
}

pub struct File {
    path: String,
}

impl File {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
}

impl Node for File {
    fn get_path(&self) {
        println!("{}", self.path);
    }

    fn number_of_files(&self) -> i64 {
        1
    }
}

pub struct Directory<'a> {
    path: String,
    sub_nodes: Vec<&'a dyn Node>,
}

impl<'a> Directory<'a> {
    pub fn new(path: String) -> Self {
        let v = Vec::new();
        Self {
            path,
            sub_nodes: v,
        }
    }

    pub fn add_sub_node(&mut self, node: &'a dyn Node) {
        self.sub_nodes.push(node);
    }
}

impl<'a> Node for Directory<'a> {
    fn get_path(&self) {
        println!("{}", self.path);
    }

    fn number_of_files(&self) -> i64 {
        let mut ret = 0i64;
        for item in &self.sub_nodes {
            ret += item.number_of_files();
        }
        ret
    }
}
