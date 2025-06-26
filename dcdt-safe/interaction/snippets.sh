ALICE="/home/numbat/numbat-sdk/drtpy/testnet/wallets/users/alice.pem"
BOB="/home/numbat/numbat-sdk/drtpy/testnet/wallets/users/bob.pem"
ADDRESS=$(drtpy data load --key=address-testnet-dcdt-safe)
DEPLOY_TRANSACTION=$(drtpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-gateway.numbat.com
CHAIN_ID=T

BOB_ADDRESS=0x8049d639e5a6980d1cd2392abcce41029cda74a1563523a202f09641cc2618f8 # 32 bytes
CAROL_ADDRESS=0xb2a11555ce521e4944e09ab17549d85b487dcd26c84b5017a39e31a3670889ba # 32 bytes
ALICE_ETH_ADDRESS=0x7d61a56899dd55e5D16C1Bab38f46f42b4d33887 # 20 bytes

TX_STATUS_EXECUTED=0x03
TX_STATUS_REJECTED=0x04

DCDT_SYSTEM_SC_ADDRESS=drt1yvesqqqqqqqqqqqqqqqqqqqqqqqqyvesqqqqqqqqqqqqqqqzlllsd5j0s2

########################################################################
################## Update after issuing the tokens #####################
########################################################################
WRAPPED_REWA_TOKEN_ID=0x
WRAPPED_ETH_TOKEN_ID=0x

deploy() {
    #######################################################################
    ################## Update with the contract's address #################
    #######################################################################
    local ETHEREUM_FEE_PREPAY_SC_ADDRESS=0x

    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 \
    --arguments ${ETHEREUM_FEE_PREPAY_SC_ADDRESS} ${WRAPPED_REWA_TOKEN_ID} ${WRAPPED_ETH_TOKEN_ID} \
    --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(drtpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(drtpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    drtpy data store --key=address-testnet-dcdt-safe --value=${ADDRESS}
    drtpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    drtpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

issueWrappedEth() {
    local TOKEN_DISPLAY_NAME=0x57726170706564457468  # "WrappedEth"
    local TOKEN_TICKER=0x57455448  # "WETH"
    local INITIAL_SUPPLY=0x01 # 1
    local NR_DECIMALS=0x12 # 18
    local CAN_ADD_SPECIAL_ROLES=0x63616e4164645370656369616c526f6c6573 # "canAddSpecialRoles"
    local TRUE=0x74727565 # "true"

    drtpy --verbose contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --value=5000000000000000000 --function="issue" \
    --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} ${INITIAL_SUPPLY} ${NR_DECIMALS} ${CAN_ADD_SPECIAL_ROLES} ${TRUE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRolesWrappedRewa() {
    local LOCAL_BURN_ROLE=0x44434454526f6c654c6f63616c4275726e # "DCDTRoleLocalBurn"
    local ADDRESS_HEX = $(drtpy wallet bech32 --decode ${ADDRESS})

    drtpy --verbose contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments ${WRAPPED_REWA_TOKEN_ID} ${ADDRESS_HEX} ${LOCAL_BURN_ROLE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRolesWrappedEth() {
    local LOCAL_BURN_ROLE=0x44434454526f6c654c6f63616c4275726e # "DCDTRoleLocalBurn"
    local ADDRESS_HEX = $(drtpy wallet bech32 --decode ${ADDRESS})

    drtpy --verbose contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments ${WRAPPED_ETH_TOKEN_ID} ${ADDRESS_HEX} ${LOCAL_BURN_ROLE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

getNextPendingTransaction() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=25000000 --function="getNextPendingTransaction" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setTransactionExecuted() {
    local RELAYER_REWARD_ADDRESS = ${CAROL_ADDRESS}
    local ORIGINAL_TX_SENDER = ${BOB_ADDRESS}
    local TX_NONCE = 0x01
    local TX_STATUS = ${TX_STATUS_EXECUTED}

    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=90000000 --function="setTransactionStatus" \
    --arguments ${RELAYER_REWARD_ADDRESS} ${ORIGINAL_TX_SENDER} ${TX_NONCE} ${TX_STATUS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setTransactionRejected() {
    local RELAYER_REWARD_ADDRESS = ${CAROL_ADDRESS}
    local ORIGINAL_TX_SENDER = ${BOB_ADDRESS}
    local TX_NONCE = 0x02
    local TX_STATUS = ${TX_STATUS_REJECTED}

    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=90000000 --function="setTransactionStatus" \
    --arguments ${RELAYER_REWARD_ADDRESS} ${ORIGINAL_TX_SENDER} ${TX_NONCE} ${TX_STATUS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}D}
}

createTransaction() {
    local CREATE_TRANSACTION_ENDPOINT = 0x6372656174655472616e73616374696f6e # "createTransaction"
    local DEST_ADDRESS = ${ALICE_ETH_ADDRESS}
    local TOKEN_USED_FOR_TX_FEES = 0x52455741 # "REWA"
    
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} \
    --gas-limit=50000000 --function="DCDTTransfer" \
    --arguments ${WRAPPED_REWA_TOKEN_IDENTIFIER} 0x64 ${CREATE_TRANSACTION_ENDPOINT} ${DEST_ADDRESS} ${TOKEN_USED_FOR_TX_FEES} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# views

getTransactionStatus() {
    drtpy --verbose contract query ${ADDRESS} --function="getTransactionStatus" \
    --arguments ${BOB_ADDRESS} 0x01 \
    --proxy=${PROXY}
}
