# Contracts Repository

This repository contains the contracts for Sunrise.

## EVM Contracts

- [sunrise-swap-external-axelar](./evm/outpost-sunrise-swap-external-axelar) - Send tokens to the Cosmos chain via Axelar GMP. Give the channel, address and memo to operate `sunrise-swap-adaptor`.

## CosmWasm Contract

- [sunrise-swap-adaptor](./cosmwasm/contracts/sunrise-swap-adapter) - Receive tokens from Axelar GMP and send Swap Request via IBC Transfer to Sunrise.
