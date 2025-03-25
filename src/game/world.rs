pub struct World {
    pub walls: Vec<(u16, u16)>,
}

impl World {
    pub fn new(dimensions: (u16, u16)) -> std::io::Result<Self> {
        Ok(World {
            walls: vec![(10, 20); dimensions.1 as usize],
        })
    }
}
