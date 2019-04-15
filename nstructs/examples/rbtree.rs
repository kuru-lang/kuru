use nstructs::RBTree;

fn main() {
    let mut tree = RBTree::new();

//    for i in 0..10 {
        let i = 0;
        tree.add(i);
//    }

    println!("{}", tree);
}
