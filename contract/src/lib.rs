use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::wee_alloc;
use near_sdk::Gas;
use near_sdk::{env, ext_contract, near_bindgen};
use std::collections::HashMap;

const BASE: Gas = 25_000_000_000_000;
pub const CALLBACK: Gas = BASE * 2;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type PoolId = String;
type FieldName = String;
type FieldValue = String;
type FieldsStorageByPoolId = UnorderedMap<PoolId, HashMap<FieldName, FieldValue>>;

#[ext_contract(staking_pool)]
pub trait ExtStakingPool {
    fn get_owner_id(&self) -> String;
}

#[ext_contract(lockup_whitelist)]
pub trait ExtWhitelist {
    fn is_whitelisted(&self, staking_pool_account_id: AccountId) -> bool;
}

#[ext_contract(ext_self_owner)]
pub trait ExtPoolDetails {
    fn on_get_owner_id(
        &mut self,
        #[callback] get_owner_id: String,
        current_user_account_id: String,
        pool_id: String,
        name: String,
        value: String,
    ) -> bool;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PoolDetails {
    fields_by_pool: FieldsStorageByPoolId,
}

#[near_bindgen]
impl PoolDetails {
    pub fn update_field(&mut self, pool_id: String, name: String, value: String) -> bool {
        assert!(pool_id != "", "Abort. Pool ID is empty");

        assert!(name != "", "Abort. Name is empty");

        assert!(value != "", "Abort. Value is empty");

        assert!(name.len() <= 2000, "Abort. Name is longer then 2000 characters");

        assert!(value.len() <= 4000, "Abort. Value is longer then 4000 characters");

        //  lockup-whitelist.near for Mainnet, whitelist.f863973.m0 for Testnet
        lockup_whitelist::is_whitelisted(pool_id.clone(), &"whitelist.f863973.m0".to_string(), 0, BASE).and(staking_pool::get_owner_id(&pool_id, 0, BASE))
            .then(ext_self_owner::on_get_owner_id(
                env::predecessor_account_id(),
                pool_id,
                name,
                value,
                &env::current_account_id(),
                0,
                CALLBACK,
            ));


        true
    }

    pub fn get_all_fields(&self, from_index: u64, limit: u64) -> HashMap<PoolId, HashMap<FieldName, FieldValue>> {
        assert!(limit <= 100, "Abort. Limit > 100");

        let keys = self.fields_by_pool.keys_as_vector();
        let values = self.fields_by_pool.values_as_vector();

        (from_index..std::cmp::min(from_index + limit, self.fields_by_pool.len()))
            .map(|index| {
                let key = keys.get(index).unwrap();
                let value = values.get(index).unwrap();
                (key, value)
            })
            .collect()
    }

    pub fn get_num_pools(&self) -> u64 {
        self.fields_by_pool.len()
    }

    pub fn get_fields_by_pool(&self, pool_id: String) -> Option<HashMap<FieldName, FieldValue>> {
        self.fields_by_pool.get(&pool_id)
    }

    pub fn on_get_owner_id(
        &mut self,
        #[callback] is_whitelisted: bool,
        #[callback] owner_id: String,
        current_user_account_id: String,
        pool_id: String,
        name: String,
        value: String,
    ) -> bool {
        assert_self();

        assert!(
            is_whitelisted,
            "Abort. Pool {} was not whitelisted.",
            pool_id
        );

        assert!(
            owner_id == current_user_account_id,
            "You are not the owner of pool. Login as {} in order to update {}. Your current account is {}",
            owner_id,
            pool_id,
            current_user_account_id
        );

        env::log(format!("Field {} added for pool {}", name, pool_id).as_bytes());

        let mut fields = self.fields_by_pool.get(&pool_id).unwrap_or_default();
        fields.insert(name, value);

        self.fields_by_pool.insert(&pool_id, &fields);

        true
    }
}

fn assert_self() {
    assert_eq!(env::predecessor_account_id(), env::current_account_id());
}
