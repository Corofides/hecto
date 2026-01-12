pub struct Buffer {
    buffer: Vec<String>,
}

impl Buffer {
   pub const fn default() -> Self {
       let buffer = Vec::new();

       Self {
           buffer,
       }
   }
   pub fn get_row(&self, row: usize) -> Option<&str> {
       if row < self.buffer.len() {
           return Some(&self.buffer[row]);
       }

       None
   }
}
