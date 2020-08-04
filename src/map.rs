pub struct Map {
    pub dimensions: (u16, u16),
    pub tiles: Vec<u8>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            dimensions: (16, 16),
            tiles: vec![0u8; 16 * 16],
        }
    }
}
