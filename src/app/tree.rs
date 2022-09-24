#[derive(Debug, Clone, Default)]
pub struct TreeItem {
    pub root: String,
    pub items: Vec<TreeItem>,
}

impl TreeItem {
    pub fn new(root: String) -> Self {
        TreeItem {
            root,
            items: Vec::new(),
        }
    }
}
