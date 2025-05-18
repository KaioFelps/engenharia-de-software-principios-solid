use datastore::Datastore;
use samambaias::entities::Samambaia;

pub fn buscar_samambaias(datastore: Datastore) -> Vec<Samambaia> {
    // Note que agora sequer precisamos importar a estrutura `FakeDB`!
    // A função agora é completamente ignorante quanto a estrutura de `Datastore`.
    datastore.todas_as_samambaias()
}
