use samambaias::entities::Samambaia;

#[derive(Default)]
pub struct FakeDb<T: Clone> {
    _db: Vec<T>,
}

impl<T: Clone> FakeDb<T> {
    pub fn new() -> Self {
        Self { _db: Vec::new() }
    }

    pub fn todos(&self) -> Vec<T> {
        self._db.to_vec()
    }

    pub fn salvar(&mut self, item: T) {
        self._db.push(item);
    }
}

pub struct Datastore<'a> {
    #[allow(unused)]
    samambaias_db: &'a mut FakeDb<Samambaia>,
}

impl<'a> Datastore<'a> {
    pub fn new(samambaias_db: &'a mut FakeDb<Samambaia>) -> Self {
        Self { samambaias_db }
    }

    pub fn todas_as_samambaias(&self) -> Vec<Samambaia> {
        self.samambaias_db.todos()
    }

    pub fn salvar_samambaia(&mut self, samambaia: Samambaia) -> Samambaia {
        self.samambaias_db.salvar(samambaia.clone());
        samambaia
    }
}
