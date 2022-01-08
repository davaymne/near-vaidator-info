NEAR Staking Pool Details (validator info)
=================================

## Description

Add details about your whitelisted staking pool (validator info) on NEAR blockchain.

Currently deployed on Near Testnet with contract address: [`validator-info.testnet`](https://explorer.testnet.near.org/accounts/validator-info.testnet) 

## Available methods

- update_field '{"pool_id": "`<YOUR_POOL>`", "name": "`<INFO_FIELD_NAME>`", "value": "`<VALUE>`"}' --accountId=`<YOUR_POOL_OWNER_ACCOUNT_ID>` --gas=200000000000000

Please find list of suggested field names in [FIELDS.md](https://github.com/zavodil/near-pool-details/blob/master/FIELDS.md) 

- get_all_fields '{"from_index": 0, "limit": 100}'
- get_fields_by_pool '{"pool_id": "`<YOUR_POOL>`"}' 


## How to call

```
near call validator-info.testnet update_field '{"pool_id": "grassets.pool.f863973.m0", "name": "website", "value": "https://grassets.tech"}' --accountId=grassets.testnet  --gas=200000000000000

near call validator-info.testnet update_field '{"pool_id": "grassets.pool.f863973.m0", "name": "github", "value": "grassets-tech"}' --accountId=grassets.testnet  --gas=200000000000000

near view  validator-info.testnet get_all_fields '{"from_index": 0, "limit": 3}'

near view  validator-info.testnet get_fields_by_pool '{"pool_id": "grassets.pool.f863973.m0"}' 

```

## Before use, please install `near-cli`

Use Near Bootcamp for detailed instructions https://bootcamp.openshards.io/#nearup-node

```
npm install near-cli -g
```

## Login
You should have your private key stored in `.near-credentials/testnet/` to be able signing transactions

```
near login
```

## Join Us
 - Mainnet: [grassets.poolv1.near](https://explorer.near.org/accounts/grassets.poolv1.near)
 - Testnet: [grassets.pool.f863973.m0](https://explorer.testnet.near.org/accounts/grassets.pool.f863973.m0)
