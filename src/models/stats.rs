
#[derive(Debug, Clone)]
pub struct ZeroMaxStat {
    pub value: f32,
    pub max: f32
}

impl ZeroMaxStat {

    pub fn new(value: f32, max: f32) -> ZeroMaxStat {

        if value > max {
            panic!("Value cannot be greater than max");
        }

        ZeroMaxStat {
            value: value,
            max: max
        }
    }

    pub fn invert_value(&self) -> f32 {
        self.max - self.value
    }

    pub fn invert_percent(&self) -> f32 {
        return 1.0 - self.percent();
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
