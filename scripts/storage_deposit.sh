#!/bin/bash
source neardev/dev-account.env

#OWNER_ID="muzikant.testnet"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$OWNER_ID\" }" --amount "0.1"

#NFT_CONTRACT="mfight-nft_v2.testnet"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$NFT_CONTRACT\" }" --amount "0.1"

#MT__CONTRACT="mfight-mt.testnet"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$MT__CONTRACT\" }" --amount "0.1"

#GAME_CONTRACT="mfight-market.testnet"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$GAME_CONTRACT\" }" --amount "0.1"

#ESCROW_CONTRACT="dev-1662470407789-30350352352939"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$ESCROW_CONTRACT\" }" --amount "0.1"

ESCROW_CONTRACT="dev-1662238661977-24614490913882"
near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$ESCROW_CONTRACT\" }" --amount "0.1"


#GAME_CONTRACT="mfight.testnet"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$GAME_CONTRACT\" }" --amount "0.1"
