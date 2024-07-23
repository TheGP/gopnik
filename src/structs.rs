#[derive(Debug)]
pub enum PersonType {
    Dohlyak,
    Krutyak,
    Huilo
}

#[derive(Debug)]
pub struct Person {
    pub kind: PersonType,
    pub health: u8,
    pub attack: u8,
}
