use datastore::{Datastore, FakeDb};
use samambaias::entities::{EspécieSamambaia, EstágioDeVida, Samambaia};

pub struct SamambaiaDatastoreComComposição<'a> {
    pub datastore: Datastore<'a>,
}

impl<'a> SamambaiaDatastoreComComposição<'a> {
    pub fn new(db: &'a mut FakeDb<Samambaia>) -> Self {
        Self {
            datastore: Datastore::new(db),
        }
    }

    // Um método qualquer. Específico dessa estrutura
    pub fn criar_esporo_de_samambaia(&mut self, espécie: EspécieSamambaia) -> Samambaia {
        self.datastore
            .salvar_samambaia(Samambaia::new(espécie, EstágioDeVida::Esporo))
    }
}
