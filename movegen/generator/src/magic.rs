pub struct MagicData {
    pub masks: [u64; 64],
    pub shifts: [u8; 64],
    pub magics: [u64; 64],
    pub offsets: [usize; 64],
    pub attacks: Vec<u64>,
}

pub(crate) fn random_magic() -> u64 {
    let mut rng = fastrand::Rng::new();

    rng.u64(..) & rng.u64(..) & rng.u64(..)
}
