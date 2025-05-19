use samambaias::entities::Samambaia;

use crate::common::traits::buscar_samambaia_do_datastore::BuscarSamambaiasDoDatastore;

// Recebemos não mais uma referência para o objeto concreto `Datastore`,
// sequer precisamos importá-lo agora.
//
// Podemos trabalhar com qualquer coisa que disponibilize o método `todas_as_samambaias`,
// definido na interface `BuscarSamambaiasDoDatastore`.
pub fn buscar_samambaias(serviço: &dyn BuscarSamambaiasDoDatastore) -> Vec<Samambaia> {
    serviço.todas_as_samambaias()
}

#[cfg(test)]
mod test {
    use datastore::{Datastore, FakeDb};
    use samambaias::entities::{EspécieSamambaia, EstágioDeVida, Samambaia};

    use super::buscar_samambaias;
    use crate::common::samambaia_datastore::SamambaiaDatastore;

    #[test]
    fn deveriamos_poder_chamar_a_função_com_o_datastore_normal() {
        let mut db = inicializar_db_falsa();
        let datastore_normal = Datastore::new(&mut db);

        let samambaias = buscar_samambaias(&datastore_normal);

        assert_eq!(3, samambaias.len());
    }

    #[test]
    fn deveriamos_poder_chamar_a_função_com_o_samambaia_datastore() {
        let mut db = inicializar_db_falsa();
        let samambaia_datastore = SamambaiaDatastore::new(&mut db);

        let samambaias = buscar_samambaias(&samambaia_datastore);

        assert_eq!(3, samambaias.len());
    }

    // apenas uma função utilitária para inicializar nosso banco de dados falso
    // previamente populado.
    fn inicializar_db_falsa() -> FakeDb<Samambaia> {
        let mut db = FakeDb::new();

        db.salvar(Samambaia::new(
            EspécieSamambaia::Tailandesa,
            EstágioDeVida::Adulta,
        ));

        db.salvar(Samambaia::new(
            EspécieSamambaia::Alface,
            EstágioDeVida::Muda,
        ));

        db.salvar(Samambaia::new(
            EspécieSamambaia::AmazonasAzul,
            EstágioDeVida::Esporo,
        ));

        db
    }
}
