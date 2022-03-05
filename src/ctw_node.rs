use rand::Rng;

#[derive(Debug, Clone)]
pub struct CtwNode {
    pub zeros: u64,
    pub ones: u64,
    pub p_kt: f64,
    pub prob: f64,
    pub zero_child: Option<Box<CtwNode>>,
    pub one_child: Option<Box<CtwNode>>,
}

impl Default for CtwNode {
    fn default() -> Self {
        Self {
            zeros: 0,
            ones: 0,
            p_kt: 1.0,
            prob: 1.0,
            zero_child: None,
            one_child: None,
        }
    }
}

impl CtwNode {
    pub fn update_zero(&mut self, context: &[bool]) {
        match context {
            [] => (),
            [false, rest @ ..] => self.zero_child.get_or_insert_default().update_zero(rest),
            [true, rest @ ..] => self.one_child.get_or_insert_default().update_zero(rest),
        }
        self.p_kt = self.p_kt * (self.zeros as f64 + 0.5) / (self.ones + self.zeros + 1) as f64;
        self.zeros += 1;
        self.prob = self.calc_probablity();
    }

    pub fn update_one(&mut self, context: &[bool]) {
        match context {
            [] => (),
            [false, rest @ ..] => self.zero_child.get_or_insert_default().update_one(rest),
            [true, rest @ ..] => self.one_child.get_or_insert_default().update_one(rest),
        }
        self.p_kt = self.p_kt * (self.ones as f64 + 0.5) / (self.ones + self.zeros + 1) as f64;
        self.ones += 1;
        self.prob = self.calc_probablity();
    }

    pub fn update(&mut self, symbol: bool, context: &[bool]) {
        match symbol {
            true => self.update_one(context),
            false => self.update_zero(context),
        }
    }

    pub fn revert_zero(&mut self, context: &[bool]) {
        match context {
            [] => (),
            [false, rest @ ..] => self.zero_child.get_or_insert_default().revert_zero(rest),
            [true, rest @ ..] => self.one_child.get_or_insert_default().revert_zero(rest),
        }
        if self.zeros > 0 {
            self.zeros -= 1;
        }
        self.p_kt = self.p_kt * (self.zeros + self.ones + 1) as f64 / (self.zeros as f64 + 0.5);
        self.prob = self.calc_probablity();
    }

    pub fn revert_one(&mut self, context: &[bool]) {
        match context {
            [] => (),
            [false, rest @ ..] => self.zero_child.get_or_insert_default().revert_one(rest),
            [true, rest @ ..] => self.one_child.get_or_insert_default().revert_one(rest),
        }
        if self.ones > 0 {
            self.ones -= 1;
        }
        self.p_kt = self.p_kt * (self.zeros + self.ones + 1) as f64 / (self.ones as f64 + 0.5);
        self.prob = self.calc_probablity();
    }

    pub fn revert(&mut self, symbol: bool, context: &[bool]) {
        match symbol {
            true => self.revert_one(context),
            false => self.revert_zero(context),
        }
    }

    pub fn calc_probablity(&self) -> f64 {
        if self.is_end_of_context() {
            self.p_kt
        } else {
            let p_zero = self.zero_child.as_ref().map(|n| n.prob).unwrap_or(1.0);
            let p_one = self.one_child.as_ref().map(|n| n.prob).unwrap_or(1.0);
            (self.p_kt + p_zero * p_one) / 2.0
        }
    }

    pub fn is_end_of_context(&self) -> bool {
        let zero_child_count = self
            .zero_child
            .as_ref()
            .map(|n| n.zeros + n.ones)
            .unwrap_or_default();
        let one_child_count = self
            .one_child
            .as_ref()
            .map(|n| n.zeros + n.ones)
            .unwrap_or_default();

        zero_child_count == 0 && one_child_count == 0
    }

    pub fn sample(&mut self, context: &[bool], mut rng: impl Rng) -> bool {
        let p_before = self.prob;
        self.update_one(context);
        let p_after = self.prob;
        self.revert_one(context);
        let p = p_after / p_before;

        let r = rng.gen_range(0.0..1.0);
        r < p
    }
}
