// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {IERC20} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol";
import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";

contract SunriseSwapWithAxelarGMP is AxelarExecutable {
    IAxelarGasService public immutable gasReciever;
    string public chainName; // name of the chain this contract is deployed to

    constructor(
        address gateway_,
        address gasReceiver_,
        string memory chainName_
    ) AxelarExecutable(gateway_) {
        gasReciever = IAxelarGasService(gasReceiver_);
        chainName = chainName_;
    }

    function sunriseSwap(
        string memory destinationChain,
        string memory destinationAddress,
        string calldata sunriseAddress,
        string calldata channelId,
        string calldata memo,
        string memory symbol,
        uint256 amount
    ) external payable {
        address tokenAddress = gateway.tokenAddresses(symbol);
        require(
            IERC20(tokenAddress).transferFrom(
                msg.sender,
                address(this),
                amount
            ),
            "Token transfer failed"
        );
        require(
            IERC20(tokenAddress).approve(address(gateway), amount),
            "Token approval failed"
        );

        // Emit event for swap initiation
        emit SwapInitiated(
            msg.sender,
            destinationChain,
            destinationAddress,
            amount
        );

        // 1. Generate GMP payload
        bytes memory payloadToCW = _encodePayloadToCosmWasm(
            sunriseAddress,
            channelId,
            memo
        );
        // 2. Pay for gas
        if (msg.value > 0) {
            gasReciever.payNativeGasForContractCallWithToken{value: msg.value}(
                address(this),
                destinationChain,
                destinationAddress,
                payloadToCW,
                symbol,
                amount,
                msg.sender
            );
        }

        // 3. Make GMP call
        gateway.callContractWithToken(
            destinationChain,
            destinationAddress,
            payloadToCW,
            symbol,
            amount
        );
    }

    function _encodePayloadToCosmWasm(
        string calldata sunriseAddress,
        string calldata channelId,
        string calldata memo
    ) internal pure returns (bytes memory) {
        // Schema
        //   bytes4  version number (0x00000001)
        //   bytes   ABI-encoded payload, indicating function name and arguments:
        //     string                   CosmWasm contract method name
        //     dynamic array of string  CosmWasm contract argument name array
        //     dynamic array of string  argument abi type array
        //     bytes                    abi encoded argument values

        // contract call arguments for ExecuteMsg::receive_message_evm{ source_chain, source_address, payload }
        bytes memory argValues = abi.encode(sunriseAddress, channelId, memo);

        string[] memory argumentNameArray = new string[](3);
        argumentNameArray[0] = "sunrise_address";
        argumentNameArray[1] = "channel_id";
        argumentNameArray[2] = "memo";

        string[] memory abiTypeArray = new string[](3);
        abiTypeArray[0] = "string";
        abiTypeArray[1] = "string";
        abiTypeArray[2] = "string";

        bytes memory gmpPayload;
        gmpPayload = abi.encode(
            "sunrise_swap",
            argumentNameArray,
            abiTypeArray,
            argValues
        );

        return
            abi.encodePacked(
                bytes4(uint32(1)), // version number
                gmpPayload
            );
    }

    function _executeWithToken(
        string calldata /*sourceChain*/,
        string calldata /*sourceAddress*/,
        bytes calldata payload,
        string calldata tokenSymbol,
        uint256 amount
    ) internal override {
        address recipient = abi.decode(payload, (address));
        address tokenAddress = gateway.tokenAddresses(tokenSymbol);

        IERC20(tokenAddress).transfer(recipient, amount);

        // Emit event for successful swap
        emit SwapCompleted(recipient, amount);
    }

    event SwapInitiated(
        address indexed sender,
        string destinationChain,
        string destinationAddress,
        uint256 amount
    );

    event SwapCompleted(address indexed recipient, uint256 amount);
}
