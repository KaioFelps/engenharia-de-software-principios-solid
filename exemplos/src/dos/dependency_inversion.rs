use datastore::{Datastore, FakeDb};
use samambaias::entities::Samambaia;

pub fn buscar_samambaias(bd: &FakeDb) -> Vec<Samambaia> {
    // Ao receber a conexão como parâmetro, não precisamos mais instanciá-la dentro da função. Removemos
    // essa dependência da função.
    //
    // Note que `Datastore` ainda é uma dependência interna e, de fato, sua inicialização aqui dentro é
    // problemática, mas usaremos isso para explicar um próximo princípio.
    let datastore = Datastore::new(bd);
    datastore.todas_as_samambaias()
}
