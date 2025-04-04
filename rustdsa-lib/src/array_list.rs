//
// ArryList implementation
//

pub struct ArrayList<T> {
    data: Vec<T>,
}

impl<T> ArrayList<T> {
    // Create a new ArrayList
    pub fn new() -> Self {
        ArrayList {
            data: Vec::new(),
        }
    }
    // Get the length of the ArrayList
    pub fn length(&self) -> usize {
        self.data.len()
    }
    // Check if the ArrayList is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Add an element to the end of the ArrayList
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    // Access an element from the ArrayList
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    // Remove and return the last element from the ArrayList
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() { Some(self.data.remove(index)) } else { None }
    }
    // Resize the ArrayList, increasing or decreasing its length
    pub fn resize(&mut self, new_size: usize, default_value: T) where T: Clone {
        if new_size < self.data.len() {
            for _ in self.data.len()..new_size {
                self.data.push(default_value.clone());
            }
        } else {
            self.data.truncate(new_size);
        }
    }
    // Return the entire vector data
    pub fn get_all(&self) -> &Vec<T> {
        &self.data
    }
}
