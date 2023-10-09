pub trait TimerTrait 
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
}

pub struct Timer 
{
    pub nom: String,
    pub jeton: char,
}