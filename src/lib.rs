#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod common;
pub mod events;
pub mod global;
pub mod orders;
pub mod proxy;
pub mod validation;

// use ash_callee_proxy::ProxyTrait;
use common::{DealConfig, FeeConfig, FeeConfigEnum, OrderInputParams, Payment};

pub type SwapTokensFixedInputResultType<BigUint> = EsdtTokenPayment<BigUint>;
type SwapOperationType<M> =
    MultiValue4<ManagedAddress<M>, ManagedBuffer<M>, TokenIdentifier<M>, BigUint<M>>;
pub type PoolResultType<BigUint> = ManagedVec<BigUint, EsdtTokenPayment<BigUint>>;

pub const SWAP_TOKENS_FIXED_INPUT_FUNC_NAME: &[u8] = b"swapTokensFixedInput";

// mod ash_callee_proxy {
//     use crate::PoolResultType;

//     multiversx_sc::imports!();

//     #[multiversx_sc::proxy]
//     pub trait CalleeContract {
//         #[payable("*")]
//         #[endpoint(exchange)]
//         fn exchange(
//             &self,
//             token_out: TokenIdentifier,
//             amount_out_min: BigUint,
//         ) -> PoolResultType<Self::Api>;
//     }
// }

#[multiversx_sc::contract]
pub trait OrderBookChallange:
    global::GlobalOperationModule
    + orders::OrdersModule
    + events::EventsModule
    + common::CommonModule
    + validation::ValidationModule
    + proxy::ProxyModule
{
    #[init]
    fn init(&self, first_token_id: TokenIdentifier, second_token_id: TokenIdentifier) {
        self.first_token_id().set_if_empty(&first_token_id);
        self.second_token_id().set_if_empty(&second_token_id);
    }

    #[payable("*")]
    #[endpoint(order)]
    fn order_endpoint(&self, tokenout: TokenIdentifier, minout: BigUint, maxfee: u64) {
        require!(
            maxfee > self.min_fee().get(),
            "Maximum fee must be greater than minimum fee!"
        );
        let provider = self.provider_lp().get();
        let fee_config = FeeConfig {
            fee_type: FeeConfigEnum::Percent,
            fixed_fee: BigUint::zero(),
            percent_fee: maxfee,
        };

        let order_input = OrderInputParams {
            amount: minout.clone(),
            match_provider: provider,
            fee_config,
            deal_config: DealConfig {
                match_provider_percent: 1_000,
            },
        };
        self.require_global_op_not_ongoing();

        let payment = self.require_valid_payment();

        if tokenout == self.first_token_id().get() {
            self.create_order(payment, order_input, common::OrderType::Sell);
        } else if tokenout == self.second_token_id().get() {
            self.create_order(payment, order_input, common::OrderType::Buy);
        }
    }

    #[endpoint(cancelOrders)]
    fn cancel_orders_endpoint(&self, order_ids: MultiValueManagedVec<u64>) {
        self.require_global_op_not_ongoing();
        self.require_order_ids_not_empty(&order_ids);

        self.cancel_orders(order_ids);
    }

    #[endpoint(cancelAllOrders)]
    fn cancel_all_orders_endpoint(&self) {
        self.require_global_op_not_ongoing();
        self.cancel_all_orders();
    }

    #[endpoint(freeOrders)]
    fn free_orders_endpoint(&self, order_ids: MultiValueManagedVec<u64>) {
        self.require_global_op_not_ongoing();
        self.require_order_ids_not_empty(&order_ids);

        self.free_orders(order_ids);
    }

    #[only_owner]
    #[endpoint(setProvider)]
    fn set_provider(&self, address: ManagedAddress) {
        self.provider_lp().set(address);
    }

    #[only_owner]
    #[endpoint(setMinFee)]
    fn set_min_fee(&self, minfee: u64) {
        self.min_fee().set(minfee);
    }

    #[only_owner]
    #[endpoint(changeFirstToken)]
    fn change_first_token_id(&self, first_token_id: TokenIdentifier) {
        self.first_token_id().set(&first_token_id);
    }

    #[only_owner]
    #[endpoint(changeSecondToken)]
    fn change_second_token_id(&self, second_token_id: TokenIdentifier) {
        self.second_token_id().set(&second_token_id);
    }

    // added
    #[only_owner]
    #[endpoint(setAdmin)]
    fn set_admin(&self, address: ManagedAddress) {
        self.admin().set(address);
    }

    #[only_owner]
    #[endpoint(setProviderAsh)]
    fn set_ash(&self, address: ManagedAddress) {
        self.provider_ash().set(address);
    }

    #[only_owner]
    #[endpoint(setRouterDex)]
    fn set_router(&self, address: ManagedAddress) {
        self.provider_router_dex().set(address);
    }

    #[view(getAdmin)]
    #[storage_mapper("admin")]
    fn admin(&self) -> SingleValueMapper<ManagedAddress>;

    // created just for testing pair swap only
    #[payable("*")]
    #[endpoint(fillOrder)]
    fn fill_order_endpoint(&self, order_input: u64) {
        let order = self.orders(order_input).get();
        let first_token_id = self.first_token_id().get();
        let second_token_id = self.second_token_id().get();

        let swap = self.pair_swap(
            second_token_id,
            order.input_amount,
            first_token_id,
            order.output_amount.clone(),
        );

        require!(swap.amount > order.output_amount, "Wrong amount");

        self.send().direct_esdt(
            &order.creator,
            &self.first_token_id().get(),
            0u64,
            &swap.amount,
        );
    }

    #[payable("*")]
    #[endpoint(resolve)]
    fn fill_order_router_endpoint(
        &self,
        order_input: u64,
        token_out: TokenIdentifier,
        amount_out_min: BigUint,
        swap_operations: MultiValueEncoded<SwapOperationType<Self::Api>>,
    ) {
        let order = self.orders(order_input).get();
        let first_token_id = self.first_token_id().get();
        let second_toke_id = self.second_token_id().get();
        let balance_first_token_before = self.get_sc_balance(first_token_id.clone());
        let caller = self.blockchain().get_caller();

        let ash_swap_call: PoolResultType<Self::Api> = self.ash_swap_call(
            second_toke_id,
            order.input_amount.clone(),
            token_out.clone(),
            amount_out_min,
        );

        for transfer in ash_swap_call.iter() {
            if transfer.token_identifier == token_out {
                self.router_dex_call(
                    transfer.token_identifier.clone(),
                    transfer.amount.clone(),
                    swap_operations.clone(),
                );
            }
        }

        let balance_first_token_after = self.get_sc_balance(first_token_id.clone());

        let amount_after_swap = balance_first_token_after - balance_first_token_before;

        require!(&amount_after_swap >= &order.output_amount, "Wrong amount");

        let fee_amount_out = self.fee_amount(order.clone(), amount_after_swap.clone());

        let deposit = Payment {
            token_id: first_token_id.clone(),
            amount: fee_amount_out.clone(),
        };
        self.fee_resolver(caller).push(&deposit);

        let amount_to_transfer = amount_after_swap - fee_amount_out;

        self.send()
            .direct_esdt(&order.creator, &first_token_id, 0u64, &amount_to_transfer);
    }

    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {}

    #[endpoint(claim)]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        let deposits = self.fee_resolver(caller.clone());

        for deposit in deposits.iter(){
            self.send().direct_esdt(&caller, &deposit.token_id, 0u64, &deposit.amount);
        }

        self.fee_resolver(caller).clear();
    }
}
