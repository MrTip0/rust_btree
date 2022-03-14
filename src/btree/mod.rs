mod nodo;
pub mod btree;

mod tests {
    use super::btree::BTree;

    #[test]
    fn create_empty() {
        let empty_tree: BTree<i32> = BTree::empty();
        let v: Vec<i32> = Vec::new();
        assert_eq!(v, empty_tree.to_vec());
    }

    #[test]
    fn create_empty_str() {
        let empty_tree: BTree<&str> = BTree::empty();
        let v: Vec<&str> = Vec::new();
        assert_eq!(v, empty_tree.to_vec());
    }

    #[test]
    fn empty_and_insert() {
        let mut tree: BTree<i32> = BTree::empty();
        tree.add(3);
        tree.add(1);
        tree.add(4);
        tree.add(6);
        tree.add(2);
        tree.add(5);
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(v, tree.to_vec());
    }

    #[test]
    fn value() {
        let mut tree = BTree::new(10);
        tree.add(21);
        tree.add(1);
        let v: Vec<i32> = vec![1, 10, 21];
        assert_eq!(v, tree.to_vec());
    }

    #[test]
    fn balance() {
        let mut tree: BTree<i32> = BTree::empty();
        tree.add(3);
        tree.add(1);
        tree.add(4);
        tree.add(6);
        tree.add(2);
        tree.add(5);
        tree.balance();
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(v, tree.to_vec());
    }
}