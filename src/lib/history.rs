#[derive(Debug)]
pub struct History {
    current: String,
    back_stack: Vec<String>,
    fwd_stack: Vec<String>,
}

impl History {
    pub fn new(initial_path: &str) -> Self {
        History {
            current: initial_path.to_string(),
            back_stack: vec![],
            fwd_stack: vec![],
        }
    }

    pub fn add(&mut self, path: &str) {
        if path != self.current {
            self.back_stack.push(self.current.clone());
            self.current = path.to_string();
        }
    }

    pub fn get_previous(&mut self) -> Option<String> {
        if let Some(previous) = self.back_stack.pop() {
            self.fwd_stack.push(self.current.clone());
            self.current = previous;
            return Some(self.current.clone());
        }
        None
    }

    pub fn get_next(&mut self) -> Option<String> {
        if let Some(next) = self.fwd_stack.pop() {
            self.back_stack.push(self.current.clone());
            self.current = next;
            return Some(self.current.clone());
        }
        None
    }
}
