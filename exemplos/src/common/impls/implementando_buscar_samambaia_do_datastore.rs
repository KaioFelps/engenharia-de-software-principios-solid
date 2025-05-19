use datastore::Datastore;

use crate::common::{
    samambaia_datastore::SamambaiaDatastore,
    traits::buscar_samambaia_do_datastore::BuscarSamambaiasDoDatastore,
};

impl BuscarSamambaiasDoDatastore for SamambaiaDatastore<'_> {
    fn todas_as_samambaias(&self) -> Vec<samambaias::entities::Samambaia> {
        self.db.todos()
    }
}

impl BuscarSamambaiasDoDatastore for Datastore<'_> {
    fn todas_as_samambaias(&self) -> Vec<samambaias::entities::Samambaia> {
        // para manter compatibilidade com os demais exemplos, não removerei o método já existente na classe.
        self.todas_as_samambaias()
    }
}
