use core::Bitboard;

#[derive(Debug)]
pub struct MagicData {
    pub masks: [u64; 64],
    pub shifts: [u8; 64],
    pub magics: [u64; 64],
    pub offsets: [usize; 64],
    pub attacks: Vec<u64>,
}

fn random_magic() -> u64 {
    let mut rng = fastrand::Rng::new();

    rng.u64(..) & rng.u64(..) & rng.u64(..)
}

fn mask_occupancy(attack_mask: Bitboard, mut index: usize) -> Bitboard {
    let mut occupancy = Bitboard::empty();

    for square in attack_mask.squares() {
        if index & 1 != 0 {
            occupancy.set(square);
        }

        index >>= 1;
    }

    occupancy
}

fn generate_shifts(masks: &[Bitboard; 64]) -> [u8; 64] {
    let mut shifts = [0; 64];

    for square in Bitboard::all_squares() {
        shifts[square as usize] = 64 - masks[square as usize].count();
    }

    shifts
}

fn generate_magic_and_attacks<FA>(
    generate_attacks: FA,
    square: u8,
    mask: Bitboard,
    shift: u8,
) -> (u64, Vec<Bitboard>)
where
    FA: Fn(u8, Bitboard) -> Bitboard,
{
    let entries = 1 << mask.count();

    let mut occupancies = vec![Bitboard::empty(); entries];
    let mut attacks = vec![Bitboard::empty(); entries];

    for index in 0..entries {
        let occupancy = mask_occupancy(mask, index);

        occupancies[index] = occupancy;
        attacks[index] = generate_attacks(square, occupancy);
    }

    'search: loop {
        let magic = random_magic();

        if (mask.to_repr().wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            continue;
        }

        let mut used = vec![None; entries];

        for index in 0..entries {
            let occupancy = occupancies[index];
            let attack = attacks[index];

            let index = (occupancy.to_repr().wrapping_mul(magic) >> shift) as usize;

            if let Some(used_attack) = used[index] {
                if used_attack != attack {
                    continue 'search;
                }
            } else {
                used[index] = Some(attack);
            }
        }

        return (magic, attacks);
    }
}

pub(crate) fn generate_magic_data<FM, FA>(generate_masks: FM, generate_attacks: FA) -> MagicData
where
    FM: Fn() -> [Bitboard; 64],
    FA: Fn(u8, Bitboard) -> Bitboard,
{
    let masks = generate_masks();
    let shifts = generate_shifts(&masks);

    let mut magics = [0; 64];
    let mut offsets = [0; 64];
    let mut attacks = Vec::new();

    for square in Bitboard::all_squares() {
        let (m, a) = generate_magic_and_attacks(
            &generate_attacks,
            square,
            masks[square as usize],
            shifts[square as usize],
        );

        magics[square as usize] = m;
        offsets[square as usize] = attacks.len();
        attacks.extend(a);
    }

    MagicData {
        masks: masks.map(|mask| mask.to_repr()),
        shifts,
        magics,
        offsets,
        attacks: attacks.iter().map(|attack| attack.to_repr()).collect(),
    }
}
