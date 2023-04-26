# Replace the following with your own values (You need to run the script once to get the contract address)
OWNER="/Users/Sebi/MultiversX/Sc-CrowdFunding/walletBob.pem"
ADDRESS="erd1qqqqqqqqqqqqqpgqfk0r0kadzspsry58wdgffca0l7j9k28zhtfsm4eatr"

ALICE="/Users/Sebi/MultiversX/Sc-CrowdFunding/walletTest.pem"
# erd1qqqqqqqqqqqqqpgq493rr8r9863zugs3e27jmn5x8wh2zqhrj0wq2ma4rg
# erd1qqqqqqqqqqqqqpgqezgle0q7l05lkqsle6vdkmwymp4dntjsj0wqjt0wpp - ride wegld
#OWNER="erd1...xxx"
# Place your keystore file in the same directory as this script and replace the following with the name of the file
# Optionally, you can also put your password in the .passfile in the same directory as this script (if not, you will be prompted for the password)
WASM_PATH="output/order-book-pair.wasm"
PRIVATE_KEY="/Users/Sebi/MultiversX/Sc-CrowdFunding/walletBob.pem"
PROXY=https://devnet-api.multiversx.com
CHAIN_ID=D
DEPLOY_GAS="120000000"

RIDE=str:RIDE-6e4c49 
WEGLD=str:WEGLD-d7c6bb
USDC=str:USDC-8d4068
MEX=str:MEX-dc289c
USDT=str:USDT-188935
# erd1qqqqqqqqqqqqqpgq3sfgh89vurcdet6lwl4cysqddeyk0rqh2gesqpkk4e - usdc-usdt
# https://devnet-explorer.multiversx.com/transactions/15152dbad0d61fbcc400591098332e36e90fd2a72bad806d46403265166ad533#24ad2d9585609736a187dc5a840a76967a8b1590bbcb342565929e1136a34e60

# https://devnet-explorer.multiversx.com/transactions/50a8b91a6086b7b7df1d90dac39247853f41154d85718cd738c7c661859be2d3#smart

# source interactions/devnet.snippets.sh && deploy

# erd1qqqqqqqqqqqqqpgqq67uv84ma3cekpa55l4l68ajzhq8qm3u0n4s20ecvx - wegld/usdc lp
# erd1qqqqqqqqqqqqqpgqq67uv84ma3cekpa55l4l68ajzhq8qm3u0n4s20ecvx mex

# Standard deploy command. Provide any constructor arguments as needed (e.g deploy 12 TOKEN-123456). Numbers are automatically scaled to 18 decimals. (e.g. 12 -> 12000000000000000000)
deploy() {
# Arguments: 
    ARG_0=str:WEGLD-d7c6bb  # 0: first_token_id (TokenIdentifier)
    ARG_1=str:USDC-8d4068  # 1: second_token_id (TokenIdentifier)

    mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${OWNER} \
          --gas-limit=${DEPLOY_GAS} \
          --outfile="deploy.interaction.json" --send --proxy=${PROXY} --chain=D --arguments ${ARG_0} ${ARG_1} || return

    echo "Deployed contract at the address written above."
    echo "Pleade update the ADDRESS variable in this script with the address of the deployed contract, then run 'source interaction.sh' to update the environment variables."
}

# Standard upgrade command. Provide any constructor arguments as needed (e.g upgrade 12 TOKEN-123). Numbers are automatically scaled to 18 decimals. (e.g. 12 -> 12000000000000000000)
upgrade() {
# Arguments: 
    ARG_0=str:WEGLD-d7c6bb  # 0: first_token_id (TokenIdentifier)
    ARG_1=str:USDC-8d4068  # 1: second_token_id (TokenIdentifier)
    mxpy --verbose contract upgrade ${ADDRESS} --recall-nonce \
        --bytecode=${WASM_PATH} \
        --pem=${PRIVATE_KEY} \
        --gas-limit=100000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments ${ARG_0} ${ARG_1} \
        --metadata-payable-by-sc \
        --send || return

}

# All contract endpoints are available as functions. Provide any arguments as needed (e.g transfer 12 TOKEN-123)

setProvider() {
    # Arguments: 
    ARG_0=0x0000000000000000050006bdc61ebbec719b07b4a7ebfd1fb215c0706e3c7ceb
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="setProvider" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 \
        --send
}

setRouterDex(){
    # erd1qqqqqqqqqqqqqpgqg2esr6d6tfd250x4n3tkhfkw8cc4p2x50n4swatdz6
    ARG_0=0x0000000000000000050042b301e9ba5a5aaa3cd59c576ba6ce3e3150a8d47ceb
    ADDRESS_ARGUMENT="erd1qqqqqqqqqqqqqpgqg2esr6d6tfd250x4n3tkhfkw8cc4p2x50n4swatdz6"
    # mxpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgqg2esr6d6tfd250x4n3tkhfkw8cc4p2x50n4swatdz6
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="setRouterDex" \
        --proxy=${PROXY} --chain=D \
        --arguments $ADDRESS_ARGUMENT \
        --send
}

setAdmin() {
    # Arguments: 
    ARG_0=0xf8945e22b4b90bea3a5296495ef02ed88c4c20d4b759d96c7d21722ba95cbad3
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="setAdmin" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 \
        --send
}

setUsdt() {
    # Arguments: 
    ARG_0=str:USDT-188935
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="setUsdt" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 \
        --send
}

setMex() {
    # Arguments: 
    ARG_0=str:MEX-dc289c
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="setMex" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 \
        --send
}

setProviderAsh(){
    ARG_0=0x000000000000000005008c128b9cace0f0dcaf5f77eb82400d6e49678c175233
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="setProviderAsh" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 \
        --send
}

fillOrder(){
    # Arguments: 
    ARG_0=2
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=100000000 \
        --function="fillOrder" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 \
        --send
}

resolve(){
    # 
    ARG_0=8
    ARG_1 = 0x0000000000000000050006bdc61ebbec719b07b4a7ebfd1fb215c0706e3c7ceb@73776170546f6b656e734669786564496e707574@5745474c442d643763366262@0279e24f13a4b178@00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb@73776170546f6b656e734669786564496e707574@4d45582d646332383963@07adc1ea4f7305a7f118
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=100000000 \
        --function="resolve" \
        --proxy=${PROXY} --chain=D \
        --arguments $ARG_0 $USDC 9983939 0x0000000000000000050006bdc61ebbec719b07b4a7ebfd1fb215c0706e3c7ceb 0x73776170546f6b656e734669786564496e707574 0x5745474c442d643763366262 0x0233ac8fe9440630 0x00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb 0x73776170546f6b656e734669786564496e707574 0x4d45582d646332383963 0x07f006161256c1654c13 \
        --send
}

ARGUMENT_2=0xe9e5d24305ef5bded3d3dab5320ab9e48a5aa61bd8ed208542452289c7bf93dc


VALUE=0x4563918244f40000 #5wegld
VALUE_USDC=10000000 #10 $
ARGUMENT_1=370000000000000000
MEX_AMOUNT=34639245539079854288647
VALUE_USDT=10000000 #10 $
MAX_FEE=100

# tokenOut, minOut, maxFee
order()
{
    user_address="$(mxpy wallet pem-address $OWNER)"
    method_name=str:order
    destination_address=$ADDRESS
    mxpy --verbose contract call $ADDRESS --recall-nonce \
        --pem=${ALICE} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=D \
        --function="ESDTTransfer" \
        --arguments $USDT $VALUE_USDC \
                    $method_name \
                    $MEX \
                    $MEX_AMOUNT \
                    $MAX_FEE \
        --send || return
}


createBuyOrder() {
    user_address="$(mxpy wallet pem-address $OWNER)"
    method_name=str:createBuyOrder
    destination_address=$ADDRESS
    mxpy --verbose contract call $ADDRESS --recall-nonce \
        --pem=${ALICE} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=D \
        --function="ESDTTransfer" \
        --arguments $USDT $VALUE_USDC \
                    $method_name \
                    $MEX_AMOUNT \
        --send || return
}
# ESDTTransfer@WEGLD@4563918244F40000@createBuyOrder@ARGUMENT_1@ARGUMENT_2@ARGUMENT_3@ARGUMENT_4@ARGUMENT_5@ARGUMENT_6

createSellOrder() {
    VALUE_WANT=19240000 #USDC
    VALUE_SEND=24700000000000000000 #RIDE

    user_address="$(mxpy wallet pem-address $OWNER)"
    method_name=str:createSellOrder
    destination_address=$ADDRESS
    mxpy --verbose contract call $ADDRESS --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=D \
        --function="ESDTTransfer" \
        --arguments $RIDE $VALUE_SEND \
                    $method_name \
                    $VALUE_WANT \
        --send || return
}

matchOrders() {
# Arguments: 
    ARG_0=1
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="matchOrders" \
        --proxy=${PROXY} --chain=D \
        --arguments 0x03 0x02 11 \
        --send

}
cancel_all_orders(){
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="cancelAllOrders" \
        --proxy=${PROXY} --chain=D \
        --send
}

claim(){
        mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=10000000 \
        --function="claim" \
        --proxy=${PROXY} --chain=D \
        --send
}

fund(){
    VALUE_USDT_send=40000000 #40 $
    method_name=str:fund
    destination_address=$ADDRESS
    mxpy --verbose contract call $ADDRESS --recall-nonce \
        --pem=${ALICE} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=D \
        --function="ESDTTransfer" \
        --arguments $USDT $VALUE_USDT_send \
                    $method_name \
        --send || return

}


changeFirstToken(){
    user_address="$(mxpy wallet pem-address $OWNER)"
    method_name=str:createSellOrder
    destination_address=$ADDRESS
    mxpy --verbose contract call $ADDRESS --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=D \
        --function="changeFirstToken" \
        --arguments $MEX \
        --send || return

}

changeSecondToken(){
    user_address="$(mxpy wallet pem-address $OWNER)"
    method_name=str:createSellOrder
    destination_address=$ADDRESS
    mxpy --verbose contract call $ADDRESS --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=D \
        --function="changeSecondToken" \
        --arguments $USDT \
        --send || return

}

cancelOrders() {
# Arguments: 
ARG_0=${1}  # 0: order_ids (variadic<u64>)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "cancelOrders" \
        --arguments 8 9

}

cancelAllOrders() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "cancelAllOrders" 
}

freeOrders() {
# Arguments: 
ARG_0=${1}  # 0: order_ids (variadic<u64>)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "freeOrders" \
        --arguments ${ARG_0} 

}

startGlobalOperation() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "startGlobalOperation" 
}

stopGlobalOperation() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "stopGlobalOperation" 
}

# All contract views. Provide arguments as needed (e.g balanceOf 0x1234567890123456789012345678901234567890)

getAddressOrderIds() {
# Arguments: 
ARG_0=${0}  # 0: address (Address)
    mxpy contract query ${ADDRESS} \
        --function "getAddressOrderIds" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

getOrderIdCounter() {
    mxpy contract query ${ADDRESS} \
        --function "getOrderIdCounter" \
        --proxy=${PROXY} 
}

getOrderById() {
# Arguments: 
ARG_0=4  #$(echo "scale=0; (${1}*10^18)/1" | bc -l)  # 0: id (u64)
    mxpy contract query ${ADDRESS} \
        --function "getOrderById" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

getFirstTokenId() {
    mxpy contract query ${ADDRESS} \
        --function "getFirstTokenId" \
        --proxy=${PROXY} 
}

getSecondTokenId() {
    mxpy contract query ${ADDRESS} \
        --function "getSecondTokenId" \
        --proxy=${PROXY} 
}

getAllOrders(){
    mxpy contract query ${ADDRESS} \
    --function "getAllOrders" \
    --proxy=${PROXY} 

}

getProviderLp(){
    mxpy contract query ${ADDRESS} \
    --function "getProviderLp" \
    --proxy=${PROXY} 
}

getAdmin(){
    mxpy contract query ${ADDRESS} \
    --function "getAdmin" \
    --proxy=${PROXY} 
}

getFlagbiguint(){
    mxpy contract query ${ADDRESS} \
    --function "getFlagbiguint" \
    --proxy=${PROXY} 
}

FLAVIUS="erd1qqqqqqqqqqqqqpgqsn4q75yt0lf4822k0plldnxjet49vkpua0ws6un5tz"
getNFTIdentifier()
{
    mxpy contract query ${FLAVIUS} \
    --function "getNFTIdentifier" \
    --proxy=${PROXY} 

}