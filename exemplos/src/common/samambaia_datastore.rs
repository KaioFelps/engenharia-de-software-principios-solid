use datastore::{Datastore, FakeDb};
use samambaias::entities::{EspécieSamambaia, EstágioDeVida, Samambaia};

pub struct SamambaiaDatastore<'a> {
    datastore: Datastore<'a>,
}

impl<'a> SamambaiaDatastore<'a> {
    pub fn new(db: &'a mut FakeDb) -> Self {
        Self {
            datastore: Datastore::new(db),
        }
    }

    pub fn todas_as_samambaias(&self) -> Vec<Samambaia> {
        self.datastore.todas_as_samambaias()
    }

    pub fn criar_esporo_de_samambaia(&self, espécie: EspécieSamambaia) -> Samambaia {}
}
