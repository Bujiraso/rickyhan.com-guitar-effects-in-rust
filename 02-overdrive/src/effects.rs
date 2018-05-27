pub trait Effect : Send {
    fn new() -> Self
        where Self: Sized;
    fn name(&self) -> &'static str;
    fn process_samples(&mut self, input: &[f32], output_l: &mut [f32], output_r: &mut [f32]) {
        output_l.clone_from_slice(input);
        output_r.clone_from_slice(input);
    }
    fn bypass(&mut self);
    fn ctrl(&mut self, msg: CtrlMsg);
}

pub enum CtrlMsg {
    Bypass
}
