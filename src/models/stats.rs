
#[derive(Debug, Clone)]
pub struct ZeroMaxStat {
    pub value: f32,
    pub max: f32
}

impl ZeroMaxStat {

    pub fn new(value: f32, max: f32) -> ZeroMaxStat {
        ZeroMaxStat {
            value: value,
            max: max
        }
    }

    pub fn percent(&self) -> f32 {
        self.value / self.max
    }

    pub fn restore(&mut self, amount: f32) {
        self.value += amount;
        if self.value > self.max {
            self.value = self.max;
        }
    }

    pub fn consume(&mut self, amount: f32) {
        self.value -= amount;
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

}
