use effects::{Effect, CtrlMsg};

pub struct Overdrive {
    pub bypassing: bool,
}

impl Effect for Overdrive {
    fn new() -> Self {
        Overdrive {
            bypassing: false
        }
    }

    fn name(&self) -> &'static str {
        "overdrive"
    }

    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        if self.bypassing { return; }
        for (i, x) in input.iter().enumerate() {
            let x = x.abs();
            let y = if 0. < x  && x < 0.333 {
                2. * x
            } else if 0.333 < x && x < 0.666 {
                let t = 2. - 3. * x;
                (3. - t * t) / 3.
            } else {
                x
            };
            output_l[i] = y;
            output_r[i] = y;
        }
    }

    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }

    fn ctrl(&mut self, msg: CtrlMsg) {
        use self::CtrlMsg::*;
        match msg {
            Bypass => self.bypass(),
        }
    }
}
