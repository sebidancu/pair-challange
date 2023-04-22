// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           34
// Async Callback (empty):               1
// Total number of exported functions:  36

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    order_book_pair
    (
        createBuyOrder
        createSellOrder
        order
        matchOrders
        cancelOrders
        cancelAllOrders
        freeOrders
        setProvider
        changeFirstToken
        changeSecondToken
        setAdmin
        setProviderAsh
        setRouterDex
        setUsdt
        setMex
        getAdmin
        getFlagbiguint
        getUsdtid
        getMexid
        fillOrder
        resolve
        fund
        withdraw
        startGlobalOperation
        stopGlobalOperation
        getAddressOrderIds
        getOrderIdCounter
        getOrderById
        getAllOrders
        getFirstTokenId
        getSecondTokenId
        getProviderLp
        getProviderAsh
        getProviderRouter
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}