#[derive(Debug, Clone, Default)]
pub struct TreeItem {
    pub root: String,
    pub items: Vec<TreeItem>,
}

impl TreeItem {
    pub fn new(root: &str) -> Self {
        TreeItem {
            root: String::from(root),
            items: Vec::new(),
        }
    }
}
