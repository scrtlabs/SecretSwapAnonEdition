use cosmwasm_std::{StdResult, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton};

use secretswap::PairInfoRaw;

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use sha2::{Digest, Sha256};

static KEY_PAIR_INFO: &[u8] = b"pair_info";
static KEY_ENTROPY_POOL: &[u8] = b"entropy_pool";

pub fn store_pair_info<S: Storage>(storage: &mut S, data: &PairInfoRaw) -> StdResult<()> {
    Singleton::new(storage, KEY_PAIR_INFO).save(data)
}

pub fn read_pair_info<S: Storage>(storage: &S) -> StdResult<PairInfoRaw> {
    ReadonlySingleton::new(storage, KEY_PAIR_INFO).load()
}

fn get_current_entropy_pool<S: Storage>(storage: &S) -> [u8; 32] {
    ReadonlySingleton::new(storage, KEY_ENTROPY_POOL)
        .load()
        .or::<[u8; 32]>(Ok([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]))
        .unwrap()
}

pub fn supply_more_entropy<S: Storage>(
    storage: &mut S,
    additional_entropy: &[u8],
) -> StdResult<()> {
    let current_entropy_pool = get_current_entropy_pool(storage);

    let mut new_entropy_source = Vec::from(current_entropy_pool);
    new_entropy_source.extend(additional_entropy);

    let new_entropy_pool: [u8; 32] = Sha256::digest(&new_entropy_source).into();

    Singleton::new(storage, KEY_ENTROPY_POOL).save(&new_entropy_pool)
}

pub fn get_random_number<S: Storage>(storage: &S) -> u64 {
    let entropy_pool = get_current_entropy_pool(storage);

    let mut rng = ChaChaRng::from_seed(entropy_pool);

    rng.next_u64()
}
