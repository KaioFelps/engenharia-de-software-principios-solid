use datastore::{Datastore, FakeDb};
use samambaias::entities::Samambaia;

pub fn buscar_samambaias(bd: &FakeDb) -> Vec<Samambaia> {
    // Observe que estamos recebendo a referência para a conexão do banco de dados
    // SOMENTE para inicializar o datastore.
    //
    // A lei de Deméter diz que não devemos utilizar intermediários para acessar um objeto-alvo.
    // Podemos, novamente, resolver isso com uma simples refatoração.
    let datastore = Datastore::new(bd);
    datastore.todas_as_samambaias()
}
