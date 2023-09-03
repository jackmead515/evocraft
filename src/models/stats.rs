
#[derive(Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
}

impl Energy {
    pub fn new(max: f32) -> Energy {
        Energy {
            current: max,
            max: max,
        }
    }

    pub fn percent(&self) -> f32 {
        self.current / self.max
    }

    pub fn expend(&mut self, amount: f32) {
        self.current -= amount;
        if self.current < 0.0 {
            self.current = 0.0;
        }
    }

    pub fn restore(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

impl Health {
    pub fn new(max: f32) -> Health {
        Health {
            current: max,
            max: max,
        }
    }

    pub fn percent(&self) -> f32 {
        self.current / self.max
    }

    pub fn damage(&mut self, amount: f32) {
        self.current -= amount;
        if self.current < 0.0 {
            self.current = 0.0;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}