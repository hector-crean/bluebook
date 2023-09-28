use std::marker::PhantomData;

use xi_rope::Rope;

use crate::search::SearchIter;

struct RopeSearchIter<'p, 'r, P>
where
    P: std::str::pattern::Pattern<'p>,
{
    search_pattern: &'p P, // Here, 'p is used to denote the lifetime of the search pattern.
    rope: &'r Rope,        // 'r is the lifetime of the rope.
    cur_index: usize,      // The current char index of the search head.
}

impl<'p, 'r, P> RopeSearchIter<'p, 'r, P>
where
    P: std::str::pattern::Pattern<'p>,
{
    fn new(rope: &'r Rope, search_pattern: &'p P) -> Self {
        RopeSearchIter {
            search_pattern,
            rope,
            cur_index: 0,
        }
    }
}

impl<'p, 'r, P> Iterator for RopeSearchIter<'p, 'r, P>
where
    P: std::str::pattern::Pattern<'p>,
{
    type Item = (usize, usize); // start and end indices of found pattern

    fn next(&mut self) -> Option<Self::Item> {
        None // Return None when no more patterns are found.
    }
}

impl<'p, 'r, P> SearchIter for RopeSearchIter<'p, 'r, P> where P: std::str::pattern::Pattern<'p> {}
