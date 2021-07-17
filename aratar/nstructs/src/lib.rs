// TODO: Replace null node with Option::None.

const RED: bool = false;
const BLACK: bool = true;

/// Red/Black Tree
#[derive(Debug)]
pub struct RBTree {
    root: RBNode,
    null: RBNode,
}

impl std::fmt::Display for RBTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Tree {{")?;
        write!(f, "{:?}", self.root)?;
        write!(f, "}}")
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RBNode {
    // FALSE = RED, TRUE = BLACK
    isblack: bool,
    value: i32, // TODO: Generic.
    left: Option<Box<RBNode>>,
    right: Option<Box<RBNode>>,
    parent: Option<Box<RBNode>>,
}

const NULL: RBNode = RBNode {
    isblack: true,
    value: 0, // don't matter
    left: None,
    right: None,
    parent: None,
};

impl RBTree {
    /// Build a new empty Red/Black Tree.
    pub fn new() -> Self {
        RBTree {
            root: NULL.clone(),
            null: NULL.clone(),
        }
    }

    /// Add a value to the tree.
    pub fn add(&mut self, value: i32) {
        let mut x = self.root.clone();
        let mut y = self.null.clone();
        let mut z = self.null.clone();

        z.value = value; // set z.

        while x != self.null {
            y = x.clone();
            if z.value < x.value {
                x = *x.left.unwrap();
            } else {
                x = *x.right.unwrap();
            }
        }

        z.parent = Some(Box::new(y.clone()));
        if y == self.null {
            self.root = z.clone();
        } else if z.value < y.value {
            y.left = Some(Box::new(z.clone()));
        } else {
            y.right = Some(Box::new(z.clone()));
        }

        z.left = Some(Box::new(self.null.clone()));
        z.right = Some(Box::new(self.null.clone()));

        z.isblack = RED;
        
    }

    /// Fix-Up the tree.
    fn add_fixup(&mut self) {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
