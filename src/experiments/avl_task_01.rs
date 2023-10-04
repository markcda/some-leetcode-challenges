use crate::structs::avl_tree::{AVLNode, AVLPtrOps, ANode};
use crate::tools::MResult;

pub fn avl_tree_task_01_generate_tree() -> MResult {
  let mut tree: ANode<i32> = AVLNode::new(16)
      .insert(17)
      .insert(15)
      .insert(1)
      .insert(20)
      .insert(21)
      .insert(14);
  println!("{:?}", tree);
  tree = tree.remove(16);
  println!("{:?}", tree);
  Ok(())
}
