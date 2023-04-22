use multiversx_sc::{
    sc_error,
    types::{Address, SCResult},
};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, whitebox::*, DebugApi,
};
use order_book_pair::*;

const WASM_PATH: &'static str = "output/order-book-pair.wasm";
pub const FIRST_TOKEN_ID: &[u8] = b"RIDE-123456";
pub const SECOND_TOKEN_ID: &[u8] = b"USDC-123456";

pub struct OrderBookSetup<OrderBookObjBuilder>
where
    OrderBookObjBuilder: 'static + Copy + Fn() -> order_book_pair::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub cf_wrapper: ContractObjWrapper<order_book_pair::ContractObj<DebugApi>, OrderBookObjBuilder>,
}

pub fn setup_orderbook<OrderBookObjBuilder>(
    cf_builder: OrderBookObjBuilder,
) -> OrderBookSetup<OrderBookObjBuilder>
where
    OrderBookObjBuilder: 'static + Copy + Fn() -> order_book_pair::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let first_user_address = blockchain_wrapper.create_user_account(&rust_biguint!(5_000));
    let second_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper.set_esdt_balance(
        &first_user_address,
        FIRST_TOKEN_ID,
        &rust_biguint!(18_000000000000000000),
    );
    blockchain_wrapper.set_esdt_balance(
        &second_user_address,
        SECOND_TOKEN_ID,
        &rust_biguint!(30_500_000),
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            let first_token_id = managed_token_id!(FIRST_TOKEN_ID);
            let second_token_id = managed_token_id!(SECOND_TOKEN_ID);

            sc.init(first_token_id, second_token_id);
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    OrderBookSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    }
}

#[test]
pub fn init_test() {
    let cf_setup = setup_orderbook(order_book_pair::contract_obj);
    cf_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_init.scen.json");
}

#[test]
fn create_buy_order_test() {
    let mut cf_setup = setup_orderbook(order_book_pair::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;
    let user_addr = &cf_setup.second_user_address;

    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &cf_setup.cf_wrapper,
            SECOND_TOKEN_ID,
            0,
            &rust_biguint!(5_000),
            |sc| {
                sc.create_buy_order_endpoint(rust_biguint!(1_000).into());

                // let counter_order = sc.order_id_counter().get();
                // let expected_deposit = 1u64;
                // assert_eq!(counter_order, expected_deposit);
            },
        )
        .assert_ok();

    let mut sc_call = ScCallMandos::new(
        user_addr,
        cf_setup.cf_wrapper.address_ref(),
        "create_buy_order_endpoint",
    );
    sc_call.add_esdt_transfer(SECOND_TOKEN_ID, 0, &rust_biguint!(5_000));

    let expect = TxExpectMandos::new(0);
    b_wrapper.add_mandos_sc_call(sc_call, Some(expect));

    cf_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_createBuyOrder.scen.json");
}
