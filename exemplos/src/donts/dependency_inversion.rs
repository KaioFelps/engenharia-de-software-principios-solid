use datastore::{Datastore, FakeDb};
use samambaias::entities::Samambaia;

pub fn buscar_samambaias() -> Vec<Samambaia> {
    // Estamos iniciando a conexão do banco de dados diretamente dentro do corpo da função.
    //
    // Além de ser uma má prática, é um baita problema de performance, já que conexões de banco de dados custam caro e,
    // geralmente, gostaríamos de ter uma só conexão (pool) sendo compartilhada para mais de uma operação do datastore.
    let bd = FakeDb::new();
    let datastore = Datastore::new(&bd);
    datastore.todas_as_samambaias()
}
