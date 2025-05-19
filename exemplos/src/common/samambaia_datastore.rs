use datastore::FakeDb;
use samambaias::entities::Samambaia;

pub struct SamambaiaDatastore<'a> {
    pub(super) db: &'a mut FakeDb<Samambaia>,
}

impl<'a> SamambaiaDatastore<'a> {
    pub fn new(db: &'a mut FakeDb<Samambaia>) -> Self {
        Self { db }
    }
}
