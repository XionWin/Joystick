pub trait TForceFeedback {
    type Item;
    
    fn register_force_feedback(&mut self, path: &str);
    fn set_rumble_effect(&self) -> Result<u16, &'static str>;
    fn run_effect(&self, id: u16) -> bool;
    fn remove_effect(&self, id: u16) -> bool;
}