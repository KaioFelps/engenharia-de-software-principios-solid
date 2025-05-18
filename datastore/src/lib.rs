use samambaias::entities::{EspécieSamambaia, EstágioDeVida, Samambaia};

#[derive(Default)]
pub struct FakeDb;

impl FakeDb {
    pub fn new() -> Self {
        Self
    }
}

pub struct Datastore<'a> {
    #[allow(unused)]
    db: &'a FakeDb,
}

impl<'a> Datastore<'a> {
    pub fn new(db: &'a FakeDb) -> Self {
        Self { db }
    }

    pub fn todas_as_samambaias(&self) -> Vec<Samambaia> {
        vec![Samambaia::new(
            EspécieSamambaia::Tailandesa,
            EstágioDeVida::Adulta,
        )]
    }
}
