pub struct ReorderAlbumsPhotosOptions {
    pub before: usize,
    pub after: usize,
}

impl Default for ReorderAlbumsPhotosOptions {
    fn default() -> Self {
        Self {
            before: 0,
            after: 0,
        }
    }
}
