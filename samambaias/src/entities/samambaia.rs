#[derive(Clone)]
pub enum TipoDeRaiz {
    Tosseira,
    Risoma,
    Batatas,
}

#[derive(Clone)]
pub enum EspécieSamambaia {
    Marisa,
    Marissa,
    Bailarina,
    RaboDePeixe,
    RaboDeFaisão,
    Paulistinha,
    PaulistinhaDeMetro,
    Tailandesa,
    Chorona,
    EstrelaCadente,
    Sonata,
    AmazonasAzul,
    AmazonasGigante,
    EstrelaAzul,
    Esmeralda,
    Alface,
}

#[derive(Clone, PartialEq, Eq)]
pub enum EstágioDeVida {
    Esporo,
    Muda,
    Adulta,
}

#[derive(Clone)]
pub struct Samambaia {
    espécie: EspécieSamambaia,
    estágio: EstágioDeVida,
}

impl Samambaia {
    pub fn new(espécie: EspécieSamambaia, estágio: EstágioDeVida) -> Self {
        Self { espécie, estágio }
    }

    pub fn obter_muda(&self) -> Option<Self> {
        if self.estágio != EstágioDeVida::Adulta {
            return None;
        }

        Some(Self::new(self.espécie.clone(), EstágioDeVida::Muda))
    }

    pub fn disseminar_esporos(&self) -> Vec<Self> {
        if self.estágio != EstágioDeVida::Adulta {
            return Vec::new();
        }

        vec![
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
            Self::new(self.espécie.clone(), EstágioDeVida::Esporo),
        ]
    }
}
