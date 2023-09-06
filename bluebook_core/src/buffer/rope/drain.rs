use ropey::Rope;

pub struct RopeDrain<'rope> {
    rope: &'rope Rope,
    start: usize,
    end: usize,
}

// impl<'rope> RopeDrain<'rope> {
//     fn new(rope: &'rope mut Rope, start: usize, end: usize) -> Self {
//         Self { rope, start, end }
//     }
// }

// impl<'rope> Iterator for RopeDrain<'rope> {
//     type Item = char;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.start < self.end && self.start < self.rope.len_chars() {
//             let char = self.rope.char(self.start);
//             self.rope.remove(self.start..(self.start + 1));
//             self.end -= 1; // Adjust the end due to removal
//             Some(char)
//         } else {
//             None
//         }
//     }
// }
