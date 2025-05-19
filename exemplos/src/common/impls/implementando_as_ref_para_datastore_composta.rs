// Essa implementação foi separada em um arquivo separado apenas para fins didáticos.
// Normalmente, essa implementação estaria no mesmo arquivo em que a `SamambaiaDatastoreComComposição`
// foi declarada.

use crate::common::samambaia_datastore_composta::SamambaiaDatastoreComComposição;
use datastore::Datastore;

impl<'a> AsRef<Datastore<'a>> for SamambaiaDatastoreComComposição<'a> {
    fn as_ref(&self) -> &Datastore<'a> {
        &self.datastore
    }
}
