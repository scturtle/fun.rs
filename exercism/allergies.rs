#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Allergen {
    Eggs, Peanuts, Shellfish, Strawberries,
    Tomatoes, Chocolate, Pollen, Cats,
}

pub struct Allergies {
    status : usize,
}

impl Allergies {
    pub fn new(status: usize) -> Self {
        Allergies { status : status }
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.status & (1 << (*allergen as usize)) != 0
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8).map(|i| {unsafe {std::mem::transmute(i as u8)}})
              .filter(|a| self.is_allergic_to(a)).collect()
    }
}
