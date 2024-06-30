#[derive(Debug, PartialOrd, PartialEq)]
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

    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn z(&self) -> i32 { self.z }

    pub fn set_x(&mut self, value: i32) { self.x = value; }
    pub fn set_y(&mut self, value: i32) { self.y = value; }
    pub fn set_z(&mut self, value: i32) { self.z = value; }
}