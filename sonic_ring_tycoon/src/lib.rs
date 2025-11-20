pub struct GameState {
    pub rings: u64,
    pub multiplier: u64,
    pub multiplier_upgrade_cost: u64,
    pub knuckles_num_collectors: u64,
    pub knuckles_collection_rate: u64,
    pub knuckles_upgrade_cost: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rings: 0,
            multiplier: 1,
            multiplier_upgrade_cost: 50,
            knuckles_num_collectors: 0,
            knuckles_collection_rate: 1,
            knuckles_upgrade_cost: 30,
        }
    }
}

// TODO: Implement more complex passive ring collection
impl GameState {
    pub fn tick(&mut self, dt: f64) {
        // simulate passive ring collection (to be expanded)
        self.rings += (dt * 0.1) as u64;
    }
}
