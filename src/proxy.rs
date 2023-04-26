multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use super::common;
use router::multi_pair_swap::ProxyTrait as _;

pub type SwapTokensFixedInputResultType<BigUint> = EsdtTokenPayment<BigUint>;
type SwapOperationType<M> =
    MultiValue4<ManagedAddress<M>, ManagedBuffer<M>, TokenIdentifier<M>, BigUint<M>>;
pub type PoolResultType<BigUint> = ManagedVec<BigUint, EsdtTokenPayment<BigUint>>;

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

#[multiversx_sc::module]
pub trait ProxyModule: common::CommonModule {
    fn ash_swap_call(
        &self,
        token_send: TokenIdentifier,
        amount_send: BigUint,
        token_out: TokenIdentifier,
        amount_out_min: BigUint,
    ) -> PoolResultType<Self::Api> {
        let ash_provider = self.provider_ash().get();

        let ash_call: PoolResultType<Self::Api> = self
            .ash_contract_proxy(ash_provider)
            .exchange(&token_out, amount_out_min)
            .with_esdt_transfer((token_send, 0u64, amount_send))
            .execute_on_dest_context();

        ash_call
    }

    fn router_dex_call(
        &self,
        token_send: TokenIdentifier,
        amount_send: BigUint,
        swap_operations: MultiValueEncoded<SwapOperationType<Self::Api>>,
    ) {
        let provider = self.provider_router_dex().get();

        let _last_payment: IgnoreValue = self
            .router_contract_proxy(provider.clone())
            .multi_pair_swap(swap_operations.clone())
            .with_esdt_transfer((token_send, 0u64, amount_send))
            .execute_on_dest_context();
    }

    fn pair_swap(
        &self,
        token_send: TokenIdentifier,
        amount_send: BigUint,
        token_out: TokenIdentifier,
        amount_out_min: BigUint,
    ) -> SwapTokensFixedInputResultType<Self::Api> {
        let provider = self.provider_lp().get();

        let result: SwapTokensFixedInputResultType<Self::Api> = self
            .pair_contract_proxy(provider)
            .swap_tokens_fixed_input(token_out, amount_out_min)
            .with_esdt_transfer((token_send, 0u64, amount_send))
            .execute_on_dest_context();

        result
    }

    #[proxy]
    fn pair_contract_proxy(&self, to: ManagedAddress) -> pair::Proxy<Self::Api>;

    #[proxy]
    fn router_contract_proxy(&self, to: ManagedAddress) -> router::Proxy<Self::Api>;

    #[proxy]
    fn ash_contract_proxy(&self, sc_address: ManagedAddress) -> ash_callee_proxy::Proxy<Self::Api>;
}
