// Return the start/end char indices of the next match.

trait SearchIter: Iterator<Item = (usize, usize)> {}

struct RopeSearchIter<'p, P: std::str::pattern::Pattern<'p>> {
    search_pattern: P,
    cur_index: usize, // The current char index of the search head.
}
