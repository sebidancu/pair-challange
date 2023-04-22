#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod common;
pub mod events;
pub mod global;
pub mod orders;
pub mod validation;
pub mod callback;

// use ash_callee_proxy::ProxyTrait;
use common::{DealConfig, FeeConfig, FeeConfigEnum, OrderInputParams};
use router::multi_pair_swap::ProxyTrait as _;

pub type SwapTokensFixedInputResultType<BigUint> = EsdtTokenPayment<BigUint>;
type SwapOperationType<M> =
    MultiValue4<ManagedAddress<M>, ManagedBuffer<M>, TokenIdentifier<M>, BigUint<M>>;
pub type PoolResultType<BigUint> = ManagedVec<BigUint, EsdtTokenPayment<BigUint>>;

pub const SWAP_TOKENS_FIXED_INPUT_FUNC_NAME: &[u8] = b"swapTokensFixedInput";

mod ash_callee_proxy {
    use crate::PoolResultType;

    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait CalleeContract {
        #[payable("*")]
        #[endpoint(exchange)]
        fn exchange(
            &self,
            token_out: TokenIdentifier,
            amount_out_min: BigUint,
        ) -> PoolResultType<Self::Api>;
    }
}

#[multiversx_sc::contract]
pub trait OrderBookChallange:
    global::GlobalOperationModule
    + orders::OrdersModule
    + events::EventsModule
    + common::CommonModule
    + validation::ValidationModule
    + callback::CallbackModule
{
    #[init]
    fn init(&self, first_token_id: TokenIdentifier, second_token_id: TokenIdentifier) {
        self.first_token_id().set_if_empty(&first_token_id);
        self.second_token_id().set_if_empty(&second_token_id);
    }

    #[payable("*")]
    #[endpoint(createBuyOrder)]
    fn create_buy_order_endpoint(&self, amount: BigUint) {
        let admin = self.admin().get();

        let fee_config = FeeConfig {
            fee_type: FeeConfigEnum::Percent,
            fixed_fee: BigUint::from(0u64),
            percent_fee: 1_000,
        };

        let order_input = OrderInputParams {
            amount,
            match_provider: admin,
            fee_config,
            deal_config: DealConfig {
                match_provider_percent: 1_000,
            },
        };
        self.require_global_op_not_ongoing();
        self.require_valid_order_input_params(&order_input);
        let payment = self.require_valid_buy_payment();

        self.create_order(payment, order_input, common::OrderType::Buy);
    }

    #[payable("*")]
    #[endpoint(createSellOrder)]
    fn create_sell_order_endpoint(&self, amount: BigUint) {
        let admin = self.admin().get();

        let fee_config = FeeConfig {
            fee_type: FeeConfigEnum::Percent,
            fixed_fee: BigUint::from(0u64),
            percent_fee: 1_000,
        };

        let order_input = OrderInputParams {
            amount,
            match_provider: admin,
            fee_config,
            deal_config: DealConfig {
                match_provider_percent: 1_000,
            },
        };
        self.require_global_op_not_ongoing();
        self.require_valid_order_input_params(&order_input);
        let payment = self.require_valid_sell_payment();

        self.create_order(payment, order_input, common::OrderType::Sell);
    }

    #[payable("*")]
    #[endpoint(order)]
    fn order_endpoint(&self, tokenOut: TokenIdentifier, minOut: BigUint, maxFee: u64) {
        let provider = self.provider_lp().get();
        let fee_config = FeeConfig {
            fee_type: FeeConfigEnum::Percent,
            fixed_fee: BigUint::zero(),
            percent_fee: maxFee,
        };

        let order_input = OrderInputParams {
            amount: minOut.clone(),
            match_provider: provider,
            fee_config,
            deal_config: DealConfig {
                match_provider_percent: 1_000,
            },
        };
        self.require_global_op_not_ongoing();

        let payment = self.require_valid_sell_payment();

        if tokenOut == self.first_token_id().get() {
            self.create_order(payment, order_input, common::OrderType::Sell);
        } else if tokenOut == self.second_token_id().get() {
            self.create_order(payment, order_input, common::OrderType::Buy);
        }
    }

    #[endpoint(matchOrders)]
    fn match_orders_endpoint(&self, order_vec: MultiValueEncoded<u64>) {
        // order_vec:MultiValueEncoded<ManagedVec<u64>>
        let mut order_ids: ManagedVec<u64> = ManagedVec::new();

        for order in order_vec {
            order_ids.push(order);
        }
        self.require_global_op_not_ongoing();
        self.require_valid_match_input_order_ids(&order_ids);

        self.match_orders(order_ids);
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

    #[endpoint(setProvider)]
    fn set_provider(&self, address: ManagedAddress) {
        self.provider_lp().set(address);
    }

    #[endpoint(changeFirstToken)]
    fn change_first_token_id(&self, first_token_id: TokenIdentifier) {
        self.first_token_id().set(&first_token_id);
    }
    #[endpoint(changeSecondToken)]
    fn change_second_token_id(&self, second_token_id: TokenIdentifier) {
        self.second_token_id().set(&second_token_id);
    }

    // added
    #[endpoint(setAdmin)]
    fn set_admin(&self, address: ManagedAddress) {
        self.admin().set(address);
    }

    #[endpoint(setProviderAsh)]
    fn set_ash(&self, address: ManagedAddress) {
        self.provider_ash().set(address);
    }

    #[endpoint(setRouterDex)]
    fn set_router(&self, address: ManagedAddress) {
        self.provider_router_dex().set(address);
    }

    #[endpoint(setUsdt)]
    fn set_usdt(&self, tokenid: TokenIdentifier) {
        self.usdtid().set(&tokenid);
    }

    #[endpoint(setMex)]
    fn set_mex(&self, tokenid: TokenIdentifier) {
        self.mexid().set(&tokenid);
    }

    #[view(getAdmin)]
    #[storage_mapper("admin")]
    fn admin(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getFlagbiguint)]
    #[storage_mapper("flagbiguint")]
    fn flagbiguint(&self) -> SingleValueMapper<BigUint>;

    #[view(getUsdtid)]
    #[storage_mapper("usdtid")]
    fn usdtid(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getMexid)]
    #[storage_mapper("mexid")]
    fn mexid(&self) -> SingleValueMapper<TokenIdentifier>;

    #[payable("*")]
    #[endpoint(fillOrder)]
    fn fill_order_endpoint(&self, order_input: u64) {
        let provider = self.provider_lp().get();
        let order = self.orders(order_input).get();

        let result: SwapTokensFixedInputResultType<Self::Api> = self
            .pair_contract_proxy(provider)
            .swap_tokens_fixed_input(self.first_token_id().get(), order.output_amount.clone())
            .with_esdt_transfer((self.second_token_id().get(), 0u64, order.input_amount))
            .execute_on_dest_context();

        require!(result.amount > order.output_amount, "Wrong amount");

        self.send().direct_esdt(
            &order.creator,
            &self.first_token_id().get(),
            0u64,
            &result.amount,
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
        let provider = self.provider_router_dex().get();
        let ash_provider = self.provider_ash().get();
        let order = self.orders(order_input).get();

        let ash_call: PoolResultType<Self::Api> = self
            .ash_contract_proxy(ash_provider)
            .exchange(&token_out, amount_out_min)
            .with_esdt_transfer((
                self.second_token_id().get(),
                0u64,
                order.input_amount.clone(),
            ))
            .execute_on_dest_context();

        let nonce = 0u64;
        let sc = self.blockchain().get_sc_address();
        let first_token_id = self.first_token_id().get();
        let balance_first_before = self
            .blockchain()
            .get_esdt_balance(&sc, &first_token_id, nonce);

        for transfer in ash_call.iter() {
            if transfer.token_identifier == token_out {
                let _last_payment: IgnoreValue = self
                    .router_contract_proxy(provider.clone())
                    .multi_pair_swap(swap_operations.clone())
                    .with_esdt_transfer((
                        transfer.token_identifier.clone(),
                        0u64,
                        transfer.amount.clone(),
                    ))
                    .execute_on_dest_context();
            }
        }

        let balance_first_after = self
            .blockchain()
            .get_esdt_balance(&sc, &first_token_id, nonce);

        let transfer_to_user_amount = balance_first_after - balance_first_before;

        require!(
            &transfer_to_user_amount >= &order.output_amount,
            "Wrong amount"
        );

        self.send().direct_esdt(
            &order.creator,
            &first_token_id,
            0u64,
            &transfer_to_user_amount,
        );
    }

    // to delete
    #[payable("*")]
    #[endpoint(fund)]
    fn fund(&self) {}

    #[only_owner]
    #[payable("*")]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let nonce = 0u64;

        let sc = self.blockchain().get_sc_address();

        let first_token_id = self.first_token_id().get();
        let balance_first = self
            .blockchain()
            .get_esdt_balance(&sc, &first_token_id, nonce);

        let second_token_id = self.second_token_id().get();
        let balance_second = self
            .blockchain()
            .get_esdt_balance(&sc, &second_token_id, nonce);

        if balance_first > BigUint::from(0u64) {
            self.send()
                .direct_esdt(&caller, &first_token_id, nonce, &balance_first);
        }
        if balance_second > BigUint::from(0u64) {
            self.send()
                .direct_esdt(&caller, &second_token_id, nonce, &balance_second);
        }
    }

    #[proxy]
    fn pair_contract_proxy(&self, to: ManagedAddress) -> pair::Proxy<Self::Api>;

    #[proxy]
    fn router_contract_proxy(&self, to: ManagedAddress) -> router::Proxy<Self::Api>;

    #[proxy]
    fn ash_contract_proxy(&self, sc_address: ManagedAddress) -> ash_callee_proxy::Proxy<Self::Api>;
}
