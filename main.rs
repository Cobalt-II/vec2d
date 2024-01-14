pub struct vec2d {
    grid: Vec<Vec<i32>>
}

impl vec2d {
    fn new(length: usize, width: usize) -> vec2d {
        let grid = vec![vec![0; width]; length];
        vec2d { grid }
    }
    fn set_index(&mut self, ptrx: usize, ptry: usize, set: i32) {
        self.grid[ptrx][ptry] = set;
        vec2d::broadcast(self);
    }
    fn remove_index(&mut self, ptrx: usize, ptry: usize) {
        self.grid[ptrx].remove(ptry);
        vec2d::broadcast(self);
    }
    fn insert_index(&mut self, ptrx: usize, ptry: usize, set: i32) {
        self.grid[ptrx].insert(ptry, set);
        vec2d::broadcast(self);
    }
    fn set_column(&mut self, ptr: usize, set: i32) {
        for i in 0..self.grid[ptr].len() {
        self.grid[ptr][i] = set;
        }
        vec2d::broadcast(self);
    }
    fn remove_column(&mut self, ptr: usize) {
        self.grid.remove(ptr);
        vec2d::broadcast(self);
    }
    fn insert_column(&mut self, ptr: usize, set: i32) {
        self.grid.insert(ptr, vec![set; self.grid[0].len()]);
        vec2d::broadcast(self);
    }
    fn set_row(&mut self, ptr: usize, set: i32) {
        for i in 0..self.grid.len() {
            self.grid[i][ptr] = set;
        }
        vec2d::broadcast(self);
    }
    fn remove_row(&mut self, ptr: usize) {
        for i in 0..self.grid.len() {
            self.grid[i].remove(ptr);
        }
        vec2d::broadcast(self);
    }
    fn insert_row(&mut self, ptr: usize, set: i32) {
        for i in 0..self.grid.len() {
            self.grid[i].insert(ptr, set);
        }
        vec2d::broadcast(self);
    }
    fn pad(&mut self, set: i32) {
        vec2d::insert_column(self, 0, set);
        vec2d::insert_column(self, self.grid.len(), set);
        vec2d::insert_row(self, 0, set);
        vec2d::insert_row(self, self.grid[0].len(), set);
    }
    fn broadcast(&mut self) {
        println!("");
        for i in &self.grid {
        println!("{:?}", i);
        }
    }
}
fn main () {
let mut world = vec2d::new(5, 5);
world.broadcast();
}


