pub trait Control {
    fn set_value(&mut self, value: i16);

    fn get_id(&self) -> u8;
    fn get_alias(&self) -> &str;
    fn get_value(&self) -> i16;
}