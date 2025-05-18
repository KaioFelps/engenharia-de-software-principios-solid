use datastore::Datastore;
use samambaias::entities::Samambaia;

// Recebemos uma referência do `Datastore`
pub fn buscar_samambaias(datastore: &Datastore) -> Vec<Samambaia> {
    datastore.todas_as_samambaias()
}

#[cfg(test)]
mod test {
    use datastore::FakeDb;
    use samambaias::entities::Samambaia;

    use super::buscar_samambaias;
    use crate::common::samambaia_datastore_composta::SamambaiaDatastoreComComposição;

    fn deveriamos_poder_chamar_a_função_com_a_estrutura_especialista() {
        let mut db = FakeDb::<Samambaia>::new();
        let samambaia_datastore = SamambaiaDatastoreComComposição::new(&mut db);

        // Observe que estamos passando a referência de `SamambaiaDatastoreComComposição`
        // como se fosse uma referência para `Datastore`.
        // Na verdade, de fato, é: estamos passando a referência para a `Datastore` que compõem
        // a struct especializada.
        let samambaias = buscar_samambaias(samambaia_datastore.as_ref());

        assert!(samambaias.is_empty());
    }
}
