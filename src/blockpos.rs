#[derive(Debug)]
pub struct BlockPos {
    x: i32,
    y: i32,
    z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    pub fn packed(&self) -> u64 {
        (self.x as u64&0x3FFFFFF) << 38 | (self.z as u64&0x3FFFFFF) << 12 | (self.y as u64&0xFFF)
    }
}