#![feature(trait_alias)]
pub mod setup;

use crate::setup::*;
use multiversx_sc::codec::multi_types::{MultiValue3, MultiValue4};
use multiversx_sc::sc_print;
use multiversx_sc::types::MultiValueEncoded;
use multiversx_sc::{
    sc_error,
    types::{Address, SCResult},
};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, whitebox::*, DebugApi,
};
use order_book_pair::common::{DealConfig, FeeConfig, FeeConfigEnum, Order};
use order_book_pair::orders::OrdersModule;
use order_book_pair::*;
pub type RustBigUint = num_bigint::BigUint;


#[test]
fn fillOrder() {
    let mut cf_setup = setup_orderbook(order_book_pair::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;
    let user_ana = &cf_setup.second_user_address;
    let user_bob = &cf_setup.first_user_address;
    let user_owner = &cf_setup.owner_address;
    let providerlp = managed_address!(&Address::zero());
    // let mut vec_orderds = MultiValueEncoded::new();
    // 0u64,1u64,2u64,3u64

    b_wrapper.set_provider(providerlp);
    b_wrapper.set_admin(user_owner);

    // limit order - buy 3 ride with 3.9 usdc
    b_wrapper
        .execute_esdt_transfer(
            user_ana,
            &cf_setup.cf_wrapper,
            SECOND_TOKEN_ID,
            0,
            &rust_biguint!(3_900_000),
            |sc| {
                sc.create_buy_order_endpoint(rust_biguint!(3_000000000000000000).into());
                let counter_id = sc.order_id_counter().get();
                assert_eq!(counter_id, 0u64);

                // vec_orderds.push(counter_id);
                // let order = sc.orders(counter_id).get();
                // let amount_input = order.input_amount;
                // sc_print!("{}", amount_input);
                // assert_eq!(amount_input,managed_biguint!(3_900_000));
            },
        )
        .assert_ok();

    // matchorders
    // b_wrapper
    //     .execute_tx(user_owner, &cf_setup.cf_wrapper, &rust_biguint!(0), |sc| {
    //         vec_orderds.push(0u64);
    //         vec_orderds.push(1u64);
    //         vec_orderds.push(2u64);
    //         vec_orderds.push(3u64);
    //         sc.match_orders_endpoint(order_type, vec_orderds);
    //     })
    //     .assert_ok();
}


