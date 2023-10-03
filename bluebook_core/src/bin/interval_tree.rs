use bluebook_core::span::augmented_avl_tree::IntervalTree;
fn main() {
    let mut tree: IntervalTree<i64, String> = IntervalTree::new();
    tree.insert(10..20, "10:20".to_string());
    tree.insert(30..40, "30:40".to_string());
    tree.insert(50..60, "50:60".to_string());

    tree.update_interval(8..34, 8..8);

    println!("{:?}", &tree);
}
