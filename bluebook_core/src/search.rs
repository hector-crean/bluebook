// Return the start/end char indices of the next match.

pub trait SearchIter: Iterator<Item = (usize, usize)> {}
