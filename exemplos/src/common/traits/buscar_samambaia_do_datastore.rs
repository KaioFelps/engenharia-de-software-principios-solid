use samambaias::entities::Samambaia;

pub trait BuscarSamambaiasDoDatastore {
    // É o único método que nossa função precisa para funcionar.
    fn todas_as_samambaias(&self) -> Vec<Samambaia>;
}
