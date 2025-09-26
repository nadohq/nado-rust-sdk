pub use contract_owner::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod contract_owner {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addEngine"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addEngine"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("engine"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("offchainExchange"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("engineType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IProductEngine.EngineType",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addOrUpdateProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addOrUpdateProducts",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spotIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("perpIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assignPubKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assignPubKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("i"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearPerpAddOrUpdateProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearPerpAddOrUpdateProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearSpotAddOrUpdateProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearSpotAddOrUpdateProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createDirectDepositV1"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createDirectDepositV1",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address payable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("creditDepositV1"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("creditDepositV1"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deletePubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deletePubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delistProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delistProduct"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pricesX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("directDepositV1Address"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("directDepositV1Address",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address payable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dumpFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dumpFees"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasPendingAddOrUpdateProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasPendingAddOrUpdateProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("multisig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_deployer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_spotEngine"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_perpEngine"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_endpoint"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_verifier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_wrappedNative"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address payable"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDirectDepositV1Ready"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDirectDepositV1Ready",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("recipient"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingPerpAddOrUpdateProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingPerpAddOrUpdateProductIds",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingSpotAddOrUpdateProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingSpotAddOrUpdateProductIds",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("perpUpdateRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpUpdateRisk"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeWithdrawPoolLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeWithdrawPoolLiquidity",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSpreads"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSpreads"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("spreads"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setWithdrawPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setWithdrawPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_withdrawPool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("spotUpdateRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("spotUpdateRisk"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitPerpAddOrUpdateProductCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitPerpAddOrUpdateProductCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sizeIncrement"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minSize"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitSpotAddOrUpdateProductCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitSpotAddOrUpdateProductCall",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quoteId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sizeIncrement"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minSize"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("config"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateTierFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateTierFeeRates"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerRateX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerRateX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawInsurance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONTRACTOWNER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\x1Cb\0\0\"V[b\0\0\xE4V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\0\xE2W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[aFE\x80b\0\0\xF4`\09`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x01\xF5W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11b\0\x01\x19W\x80c\xBB\xEF\x84\xB4\x11b\0\0\xAFW\x80c\xC9\xC5\xEF\xAA\x11b\0\0zW\x80c\xC9\xC5\xEF\xAA\x14b\0\x04:W\x80c\xD9\xFB\x99\xC1\x14b\0\x04DW\x80c\xEB\xD6\xC2\x94\x14b\0\x04[W\x80c\xF2\xFD\xE3\x8B\x14b\0\x04rW`\0\x80\xFD[\x80c\xBB\xEF\x84\xB4\x14b\0\x03\xDCW\x80c\xBE\x05\xA6\x9D\x14b\0\x03\xF3W\x80c\xBE\x13\xBA\xC4\x14b\0\x04\x0CW\x80c\xC2'\xDB\x96\x14b\0\x04#W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11b\0\0\xF0W\x80c\x8D\xA5\xCB[\x14b\0\x03\x85W\x80c\x90\x86\xA2\xA5\x14b\0\x03\x97W\x80c\x94\x91+\x80\x14b\0\x03\xAEW\x80c\x9Bj\xBA\x8F\x14b\0\x03\xC5W`\0\x80\xFD[\x80cqP\x18\xA6\x14b\0\x03MW\x80c\x8A)\xE2\xDE\x14b\0\x03WW\x80c\x8A\xB3\xDA\xAE\x14b\0\x03nW`\0\x80\xFD[\x80cG\xE2\xCA\x90\x11b\0\x01\x8FW\x80clU[\x1B\x11b\0\x01fW\x80clU[\x1B\x14b\0\x02\xFEW\x80clleN\x14b\0\x03\x15W\x80cn\x13\xCB\xF3\x14b\0\x03,W\x80cp|\x8BX\x14b\0\x03CW`\0\x80\xFD[\x80cG\xE2\xCA\x90\x14b\0\x02\xB9W\x80cS\x12\xB9\x1F\x14b\0\x02\xD0W\x80cV\xE4\x9E\xF3\x14b\0\x02\xE7W`\0\x80\xFD[\x80c\x19b\xC3\x84\x11b\0\x01\xD0W\x80c\x19b\xC3\x84\x14b\0\x02jW\x80c+\x12l_\x14b\0\x02tW\x80c3\x92\xC5\x85\x14b\0\x02\x8BW\x80c=\xAB\xE0\xD9\x14b\0\x02\xA2W`\0\x80\xFD[\x80c\x12\xEF\xA7\x1B\x14b\0\x01\xFAW\x80c\x14\\\xA3\x80\x14b\0\x02CW\x80c\x17K\x8D^\x14b\0\x02^W[`\0\x80\xFD[b\0\x02&b\0\x02\x0B6`\x04b\0$\xCAV[`\xA3` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\0\x02Mb\0\x04\x89V[`@Q\x90\x15\x15\x81R` \x01b\0\x02:V[b\0\x02hb\0\x04\xA3V[\0[b\0\x02hb\0\x05\rV[b\0\x02hb\0\x02\x856`\x04b\0&^V[b\0\x05qV[b\0\x02hb\0\x02\x9C6`\x04b\0'\x83V[b\0\tuV[b\0\x02hb\0\x02\xB36`\x04b\0(\rV[b\0\t\xEBV[b\0\x02Mb\0\x02\xCA6`\x04b\0(\xC7V[b\0\x0C\rV[b\0\x02hb\0\x02\xE16`\x04b\0)\x0FV[b\0\x0F\xB4V[b\0\x02hb\0\x02\xF86`\x04b\0)_V[b\0\x10\xBBV[b\0\x02hb\0\x03\x0F6`\x04b\0)\xBCV[b\0\x11+V[b\0\x02hb\0\x03&6`\x04b\0*!V[b\0\x12\xCAV[b\0\x02&b\0\x03=6`\x04b\0$\xCAV[b\0\x13|V[b\0\x02hb\0\x146V[b\0\x02hb\0\x14\xC6V[b\0\x02hb\0\x03h6`\x04b\0*]V[b\0\x14\xDCV[b\0\x02hb\0\x03\x7F6`\x04b\0'\x83V[b\0\x16\xEDV[`gT`\x01`\x01`\xA0\x1B\x03\x16b\0\x02&V[b\0\x02hb\0\x03\xA86`\x04b\0$\xCAV[b\0\x17+V[b\0\x02hb\0\x03\xBF6`\x04b\0$\xCAV[b\0\x17\x93V[b\0\x02hb\0\x03\xD66`\x04b\0+\x84V[b\0\x17\xCFV[b\0\x02hb\0\x03\xED6`\x04b\0$\xCAV[b\0\x1AJV[b\0\x03\xFDb\0\x1A\x86V[`@Qb\0\x02:\x91\x90b\0,>V[b\0\x02hb\0\x04\x1D6`\x04b\0,\x8AV[b\0\x1B\xFAV[b\0\x02hb\0\x0446`\x04b\0(\xC7V[b\0\x1CDV[b\0\x03\xFDb\0\x1C\x81V[b\0\x02hb\0\x04U6`\x04b\0,\xB7V[b\0\x1D\xEFV[b\0\x02hb\0\x04l6`\x04b\0)\x0FV[b\0\x1F\xB0V[b\0\x02hb\0\x04\x836`\x04b\0(\xC7V[b\0 \xCEV[`\xA5T`\0\x90\x15\x15\x80b\0\x04\x9EWP`\xA4T\x15\x15[\x90P\x90V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x04\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[b\0\x05\x0B`\xA5`\0b\0#\x96V[V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[b\0\x05\x0B`\xA4`\0b\0#\x96V[b\0\x05{b\0!dV[`\0[`\xA4T\x81\x10\x15b\0\x07mW`\0`\xA4\x82\x81T\x81\x10b\0\x05\xA1Wb\0\x05\xA1b\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x05\xB8\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x05\xE6\x90b\0-dV[\x80\x15b\0\x067W\x80`\x1F\x10b\0\x06\x0BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x067V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x06\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x06Q\x91\x90b\0.$V[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10b\0\x06sWb\0\x06sb\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x06\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fspot id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\xF4V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Qc\xDF\x14O\xD5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96c\xDF\x14O\xD5\x96b\0\x07\"\x96\x90\x95\x90\x94\x93\x92\x91`\x04\x01b\0/`V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x07=W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x07RW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x07d\x90b\x000SV[\x91PPb\0\x05~V[Pb\0\x07|`\xA4`\0b\0#\x96V[`\0[`\xA5T\x81\x10\x15b\0\tbW`\0`\xA5\x82\x81T\x81\x10b\0\x07\xA2Wb\0\x07\xA2b\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x07\xB9\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\xE7\x90b\0-dV[\x80\x15b\0\x088W\x80`\x1F\x10b\0\x08\x0CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x088V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x08\x1AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x08R\x91\x90b\x000oV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10b\0\x08tWb\0\x08tb\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x08\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fperp id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\xF4V[`\x9BT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xA4Y\x89\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x94c\xA4Y\x89\xAB\x94b\0\t\x17\x94\x90\x93\x90\x92\x91`\x04\x01b\x001\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t2W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\tGW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\tY\x90b\x000SV[\x91PPb\0\x07\x7FV[Pb\0\tq`\xA5`\0b\0#\x96V[PPV[b\0\t\x7Fb\0!dV[`\x9BT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\t\xB3\x90\x85\x90\x85\x90`\x04\x01b\x001qV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\t\xE3W=`\0\x80>=`\0\xFD[PPPPPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[\x84\x83\x14b\0\n\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0[\x85\x81\x10\x15b\0\x0C\x04W`\0`@Q\x80``\x01`@R\x80\x89\x89\x85\x81\x81\x10b\0\n\xB1Wb\0\n\xB1b\0-NV[\x90P` \x02\x01` \x81\x01\x90b\0\n\xC8\x91\x90b\x001\xC6V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x87\x85\x81\x81\x10b\0\n\xE8Wb\0\n\xE8b\0-NV[\x90P` \x02\x01` \x81\x01\x90b\0\n\xFF\x91\x90b\x001\xE6V[`\x0F\x0B\x81R` \x01\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91P`\x14\x90P\x82`@Q` \x01b\0\x0BW\x91\x90b\x002\x06V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x0Bw\x92\x91` \x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x0B\xB8\x90\x84\x90`\x04\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0B\xE8W=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x0B\xFB\x90b\x000SV[\x91PPb\0\n\x86V[PPPPPPPV[`\0\x80`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0CdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x0C\x8E\x91\x90\x81\x01\x90b\x003\x13V[\x90P`\0[\x81Q\x81\x10\x15b\0\x0F\xAAW`\0\x82\x82\x81Q\x81\x10b\0\x0C\xB4Wb\0\x0C\xB4b\0-NV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r:\x91\x90b\x003\xADV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\r\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\xF4V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x82\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\x06\x91\x90b\x003\xCDV[`\x9FT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x84\x16\x03b\0\x0E9Wb\0\x0E6`\x01`\x01`\xA0\x1B\x03\x89\x161\x82b\x003\xE7V[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0ExW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\x9E\x91\x90b\x004\x02V[b\0\x0E\xAB\x90`\x12b\x004'V[b\0\x0E\xB8\x90`\nb\x005JV[b\0\x0E\xC4\x90\x82b\x005[V[`\x9AT`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x0F\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0FA\x91\x90b\x005}V[`\x80\x01Q\x90Pb\0\x0F\\g\r\xE0\xB6\xB3\xA7d\0\0`\x05b\x006#V[`\x0F\x0Bb\0\x0Fw\x83\x83`\x0F\x0Bb\0!\xC0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x12b\0\x0F\x8FWP`\x01\x98\x97PPPPPPPPV[PPPPP\x80\x80b\0\x0F\xA1\x90b\x000SV[\x91PPb\0\x0C\x93V[P`\0\x93\x92PPPV[b\0\x0F\xBEb\0!dV[`\x9DT`@\x80Qc\x1Ft\xAC\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFB\xA5`\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x10\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10/\x91\x90b\x003\xADV[`@Qc\xFD\x8CR\xCD`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x92P\x90\x82\x16\x90c\xFD\x8CR\xCD\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x10\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x10\xB1W=`\0\x80>=`\0\xFD[PPPPPPPPV[b\0\x10\xC5b\0!dV[`\x9DT`@QcV\xE4\x9E\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cV\xE4\x9E\xF3\x90b\0\x10\xFB\x90\x86\x90\x86\x90\x86\x90`\x04\x01b\x006\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x11\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0C\x04W=`\0\x80>=`\0\xFD[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x11\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0b\0\x11\x8Db\0\x1A\x86V[\x90P`\0[\x81Q\x81\x10\x15b\0\x12@W\x81\x81\x81Q\x81\x10b\0\x11\xB1Wb\0\x11\xB1b\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x03b\0\x12+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a perp p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[\x80b\0\x127\x81b\x000SV[\x91PPb\0\x11\x92V[P`\xA5`@Q\x80`\x80\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85`\x0F\x0B\x81R` \x01\x84\x806\x03\x81\x01\x90b\0\x12\x7F\x91\x90b\x007\x1BV[\x90R`@Qb\0\x12\x93\x91\x90` \x01b\x007:V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\t\xE3\x94\x91\x90\x92\x01\x92\x01\x90b\0#\xB6V[b\0\x12\xD4b\0!dV[`@\x80Q\x80\x82\x01\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x81\x84\x01R\x82Q\x80\x82\x03\x84\x01\x81R``\x82\x01\x90\x93R\x90\x91`\0\x91b\0\x13;\x91`\x12\x91`\x80\x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x10\x81\x90\x84\x90`\x04\x01b\x002\xFEV[`\x9CT`\x9AT`\x9FT`@Q`\0\x93\x84\x93`\x01\x93`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93\x91\x83\x16\x92\x88\x92\x91\x16\x90b\0\x13\xB0\x90b\0$EV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R`@\x83\x01\x91\x90\x91R\x90\x91\x16``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x13\xF9W=`\0\x80>=`\0\xFD[P`\0\x93\x84R`\xA3` R`@\x90\x93 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UP\x90\x91\x90PV[b\0\x14@b\0!dV[`@\x80Q`\t`\xF8\x1B` \x82\x01R\x81Q`\x01\x81\x83\x03\x01\x81R`!\x82\x01\x92\x83\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x90\x93R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x14\x8F\x90\x84\x90`%\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x14\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x14\xBFW=`\0\x80>=`\0\xFD[PPPPPV[b\0\x14\xD0b\0!dV[b\0\x05\x0B`\0b\0\"NV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x14\xFDWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x15\x19WP0;\x15\x80\x15b\0\x15\x19WP`\0T`\xFF\x16`\x01\x14[b\0\x15\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x04\xF4V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x15\xB1W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x88\x163\x14b\0\x16\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fexpected deployed to initialize\0`D\x82\x01R`d\x01b\0\x04\xF4V[b\0\x16\x15b\0\"\xADV[b\0\x16 \x89b\0 \xCEV[`\x99\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x17\x90\x92U`\x9A\x80T\x82\x16\x8A\x84\x16\x17\x90U`\x9B\x80T\x82\x16\x89\x84\x16\x17\x90U`\x9C\x80T\x82\x16\x88\x84\x16\x17\x90U`\x9D\x80T\x82\x16\x87\x84\x16\x17\x90U`\x9E\x80T\x82\x16\x86\x84\x16\x17\x90U`\x9F\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15b\0\x16\xE2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[b\0\x16\xF7b\0!dV[`\x9AT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\t\xB3\x90\x85\x90\x85\x90`\x04\x01b\x001qV[`\0\x81\x81R`\xA3` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x17WWb\0\x17T\x82b\0\x13|V[\x90P[\x80`\x01`\x01`\xA0\x1B\x03\x16c&\x08o\x07`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t\xCEW`\0\x80\xFD[b\0\x17\x9Db\0!dV[`\x9DT`@Qc\x01)\"W`\xE7\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\x91+\x80\x90`$\x01b\0\x14\x8FV[b\0\x17\xD9b\0!dV[\x82Q\x84Q\x14b\0\x18\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[\x81Q\x84Q\x14b\0\x18aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[\x80Q\x84Q\x14b\0\x18\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0[\x84Q\x81\x10\x15b\0\x14\xBFW`\0`@Q\x80`\x80\x01`@R\x80\x87\x84\x81Q\x81\x10b\0\x18\xD4Wb\0\x18\xD4b\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x84\x81Q\x81\x10b\0\x18\xFCWb\0\x18\xFCb\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85\x84\x81Q\x81\x10b\0\x19$Wb\0\x19$b\0-NV[` \x02` \x01\x01Q`\x0F\x0B\x81R` \x01\x84\x84\x81Q\x81\x10b\0\x19IWb\0\x19Ib\0-NV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0F\x0B\x90R\x90P`\0`\x18`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x86\x01Q\x16\x81\x83\x01R\x90\x84\x01Q`\x0F\x90\x81\x0B``\x80\x84\x01\x91\x90\x91R\x85\x01Q\x90\x0B`\x80\x82\x01R`\xA0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x19\xBD\x92\x91` \x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x19\xFE\x90\x84\x90`\x04\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1A\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1A.W=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x1AA\x90b\x000SV[\x91PPb\0\x18\xA8V[b\0\x1ATb\0!dV[`\x9ET`@Qc.\xFB\xE1-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\xEF\x84\xB4\x90`$\x01b\0\x14\x8FV[`\xA5T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x1A\xAAWb\0\x1A\xAAb\0$\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x1A\xD4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA5T\x81\x10\x15b\0\x1B\xF4W`\0`\xA5\x82\x81T\x81\x10b\0\x1A\xFDWb\0\x1A\xFDb\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x1B\x14\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1BB\x90b\0-dV[\x80\x15b\0\x1B\x93W\x80`\x1F\x10b\0\x1BgWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1B\x93V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1BuW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x1B\xAD\x91\x90b\x000oV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x1B\xC9Wb\0\x1B\xC9b\0-NV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x1B\xEB\x81b\x000SV[\x91PPb\0\x1A\xDAV[P\x91\x90PV[b\0\x1C\x04b\0!dV[`\x9ET`@Qc/\x84\xEE\xB1`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\x13\xBA\xC4\x90`d\x01b\0\x10\xFBV[b\0\x1CNb\0!dV[`\x9DT`@Qca\x13\xED\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC2'\xDB\x96\x90`$\x01b\0\x14\x8FV[`\xA4T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x1C\xA5Wb\0\x1C\xA5b\0$\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x1C\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA4T\x81\x10\x15b\0\x1B\xF4W`\0`\xA4\x82\x81T\x81\x10b\0\x1C\xF8Wb\0\x1C\xF8b\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x1D\x0F\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1D=\x90b\0-dV[\x80\x15b\0\x1D\x8EW\x80`\x1F\x10b\0\x1DbWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1D\x8EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1DpW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x1D\xA8\x91\x90b\0.$V[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x1D\xC4Wb\0\x1D\xC4b\0-NV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x1D\xE6\x81b\x000SV[\x91PPb\0\x1C\xD5V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1EEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0b\0\x1EQb\0\x1C\x81V[\x90P`\0[\x81Q\x81\x10\x15b\0\x1F\x04W\x81\x81\x81Q\x81\x10b\0\x1EuWb\0\x1Eub\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x88c\xFF\xFF\xFF\xFF\x16\x03b\0\x1E\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a spot p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[\x80b\0\x1E\xFB\x81b\x000SV[\x91PPb\0\x1EVV[P`\xA4`@Q\x80`\xC0\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\x0F\x0B\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85\x806\x03\x81\x01\x90b\0\x1FO\x91\x90b\x007\xBBV[\x81R` \x01b\0\x1Fe6\x86\x90\x03\x86\x01\x86b\x007\x1BV[\x90R`@Qb\0\x1Fy\x91\x90` \x01b\08\x85V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x10\xB1\x94\x91\x90\x92\x01\x92\x01\x90b\0#\xB6V[b\0\x1F\xBAb\0!dV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x85\x16\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x10`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0 ]\x92\x91` \x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0 \x9E\x90\x84\x90`\x04\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0 \xB9W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x16\xE2W=`\0\x80>=`\0\xFD[b\0 \xD8b\0!dV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0!VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x04\xF4V[b\0!a\x81b\0\"NV[PV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x04\xF4V[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90b\0\"\x08WPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90b\0\"DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x04\xF4\x91\x90b\x002\xFEV[P\x90P[\x92\x91PPV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0#\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[b\0\x05\x0B`\0Ta\x01\0\x90\x04`\xFF\x16b\0#\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[b\0\x05\x0B3b\0\"NV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0!a\x91\x90b\0$SV[\x82\x80Tb\0#\xC4\x90b\0-dV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0#\xE8W`\0\x85Ub\0$3V[\x82`\x1F\x10b\0$\x03W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0$3V[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0$3W\x91\x82\x01[\x82\x81\x11\x15b\0$3W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0$\x16V[Pb\0$A\x92\x91Pb\0$tV[P\x90V[a\x0C\x95\x80b\09{\x839\x01\x90V[\x80\x82\x11\x15b\0$AW`\0b\0$j\x82\x82b\0$\x8BV[P`\x01\x01b\0$SV[[\x80\x82\x11\x15b\0$AW`\0\x81U`\x01\x01b\0$uV[P\x80Tb\0$\x99\x90b\0-dV[`\0\x82U\x80`\x1F\x10b\0$\xAAWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0!a\x91\x90b\0$tV[`\0` \x82\x84\x03\x12\x15b\0$\xDDW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0% Wb\0% b\0$\xE4V[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0% Wb\0% b\0$\xE4V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0% Wb\0% b\0$\xE4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0%\x9EWb\0%\x9Eb\0$\xE4V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0%\xC3Wb\0%\xC3b\0$\xE4V[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0!aW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12b\0%\xF2W`\0\x80\xFD[\x815` b\0&\x0Bb\0&\x05\x83b\0%\xA6V[b\0%rV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0&+W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0&SW\x805b\0&E\x81b\0%\xCDV[\x83R\x91\x83\x01\x91\x83\x01b\0&/V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0&rW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0&\x8BW`\0\x80\xFD[b\0&\x99\x86\x83\x87\x01b\0%\xE0V[\x93P` \x85\x015\x91P\x80\x82\x11\x15b\0&\xB0W`\0\x80\xFD[Pb\0&\xBF\x85\x82\x86\x01b\0%\xE0V[\x91PP\x92P\x92\x90PV[\x80`\x03\x0B\x81\x14b\0!aW`\0\x80\xFD[\x80`\x0F\x0B\x81\x14b\0!aW`\0\x80\xFD[\x805b\0&\xF6\x81b\0&\xD9V[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15b\0'\x0EW`\0\x80\xFD[b\0'\x18b\0$\xFAV[\x90P\x815b\0''\x81b\0&\xC9V[\x81R` \x82\x015b\0'9\x81b\0&\xC9V[` \x82\x01R`@\x82\x015b\0'N\x81b\0&\xC9V[`@\x82\x01R``\x82\x015b\0'c\x81b\0&\xC9V[``\x82\x01R`\x80\x82\x015b\0'x\x81b\0&\xD9V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15b\0'\x97W`\0\x80\xFD[\x825b\0'\xA4\x81b\0%\xCDV[\x91Pb\0'\xB5\x84` \x85\x01b\0&\xFBV[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0'\xD1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0'\xEAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0(\x06W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15b\0('W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0(@W`\0\x80\xFD[b\0(N\x8A\x83\x8B\x01b\0'\xBEV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15b\0(hW`\0\x80\xFD[b\0(v\x8A\x83\x8B\x01b\0'\xBEV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15b\0(\x90W`\0\x80\xFD[Pb\0(\x9F\x89\x82\x8A\x01b\0'\xBEV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0!aW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15b\0(\xDAW`\0\x80\xFD[\x815b\0(\xE7\x81b\0(\xB1V[\x93\x92PPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0&\xF6W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0)%W`\0\x80\xFD[\x835b\0)2\x81b\0%\xCDV[\x92Pb\0)B` \x85\x01b\0(\xEEV[\x91P`@\x84\x015b\0)T\x81b\0(\xB1V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0)uW`\0\x80\xFD[\x835b\0)\x82\x81b\0(\xB1V[\x92P` \x84\x015b\0)\x94\x81b\0(\xB1V[\x91P`@\x84\x015`\x02\x81\x10b\0)TW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15b\0\x1B\xF4W`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15b\0)\xD4W`\0\x80\xFD[\x845b\0)\xE1\x81b\0%\xCDV[\x93P` \x85\x015b\0)\xF3\x81b\0&\xD9V[\x92P`@\x85\x015b\0*\x05\x81b\0&\xD9V[\x91Pb\0*\x16\x86``\x87\x01b\0)\xA9V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15b\0*5W`\0\x80\xFD[b\0*@\x83b\0(\xEEV[\x91P` \x83\x015b\0*R\x81b\0(\xB1V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15b\0*{W`\0\x80\xFD[\x885b\0*\x88\x81b\0(\xB1V[\x97P` \x89\x015b\0*\x9A\x81b\0(\xB1V[\x96P`@\x89\x015b\0*\xAC\x81b\0(\xB1V[\x95P``\x89\x015b\0*\xBE\x81b\0(\xB1V[\x94P`\x80\x89\x015b\0*\xD0\x81b\0(\xB1V[\x93P`\xA0\x89\x015b\0*\xE2\x81b\0(\xB1V[\x92P`\xC0\x89\x015b\0*\xF4\x81b\0(\xB1V[\x91P`\xE0\x89\x015b\0+\x06\x81b\0(\xB1V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x82`\x1F\x83\x01\x12b\0+)W`\0\x80\xFD[\x815` b\0+<b\0&\x05\x83b\0%\xA6V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0+\\W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0&SW\x805b\0+v\x81b\0&\xD9V[\x83R\x91\x83\x01\x91\x83\x01b\0+`V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0+\x9BW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0+\xB4W`\0\x80\xFD[b\0+\xC2\x88\x83\x89\x01b\0%\xE0V[\x95P` \x87\x015\x91P\x80\x82\x11\x15b\0+\xD9W`\0\x80\xFD[b\0+\xE7\x88\x83\x89\x01b\0%\xE0V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\0+\xFEW`\0\x80\xFD[b\0,\x0C\x88\x83\x89\x01b\0+\x17V[\x93P``\x87\x015\x91P\x80\x82\x11\x15b\0,#W`\0\x80\xFD[Pb\0,2\x87\x82\x88\x01b\0+\x17V[\x91PP\x92\x95\x91\x94P\x92PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0,~W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0,ZV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0,\xA0W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15b\0,\xD3W`\0\x80\xFD[\x875b\0,\xE0\x81b\0%\xCDV[\x96P` \x88\x015b\0,\xF2\x81b\0%\xCDV[\x95P`@\x88\x015b\0-\x04\x81b\0&\xD9V[\x94P``\x88\x015b\0-\x16\x81b\0&\xD9V[\x93P`\xE0`\x7F\x19\x82\x01\x12\x15b\0-+W`\0\x80\xFD[P`\x80\x87\x01\x91Pb\0-B\x88a\x01`\x89\x01b\0)\xA9V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0-yW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x1B\xF4WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80Qb\0&\xF6\x81b\0&\xD9V[`\0`\xA0\x82\x84\x03\x12\x15b\0-\xBAW`\0\x80\xFD[b\0-\xC4b\0$\xFAV[\x90P\x81Qb\0-\xD3\x81b\0&\xC9V[\x81R` \x82\x01Qb\0-\xE5\x81b\0&\xC9V[` \x82\x01R`@\x82\x01Qb\0-\xFA\x81b\0&\xC9V[`@\x82\x01R``\x82\x01Qb\0.\x0F\x81b\0&\xC9V[``\x82\x01R`\x80\x82\x01Qb\0'x\x81b\0&\xD9V[`\0\x81\x83\x03a\x02\0\x81\x12\x15b\0.9W`\0\x80\xFD[b\0.Cb\0%&V[\x83Qb\0.P\x81b\0%\xCDV[\x81R` \x84\x01Qb\0.b\x81b\0%\xCDV[` \x82\x01R`@\x84\x01Qb\0.w\x81b\0&\xD9V[`@\x82\x01R``\x84\x01Qb\0.\x8C\x81b\0&\xD9V[``\x82\x01R`\xE0`\x7F\x19\x83\x01\x12\x15b\0.\xA4W`\0\x80\xFD[b\0.\xAEb\0%LV[\x91P`\x80\x84\x01Qb\0.\xC0\x81b\0(\xB1V[\x82R`\xA0\x84\x01Qb\0.\xD2\x81b\0&\xD9V[` \x83\x01R`\xC0\x84\x01Qb\0.\xE7\x81b\0&\xD9V[`@\x83\x01R`\xE0\x84\x01Qb\0.\xFC\x81b\0&\xD9V[``\x83\x01Rb\0/\x10a\x01\0\x85\x01b\0-\x9AV[`\x80\x83\x01Rb\0/$a\x01 \x85\x01b\0-\x9AV[`\xA0\x83\x01Rb\0/8a\x01@\x85\x01b\0-\x9AV[`\xC0\x83\x01R\x81`\x80\x82\x01Rb\0/S\x85a\x01`\x86\x01b\0-\xA7V[`\xA0\x82\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x0F\x85\x81\x0B`@\x83\x01R\x84\x90\x0B``\x82\x01Ra\x02\0\x81\x01b\0/\xEF`\x80\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[\x82Q`\x03\x90\x81\x0Ba\x01`\x84\x01R` \x84\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x84\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x84\x01Q\x90\x0Ba\x01\xC0\x83\x01R`\x80\x83\x01Q`\x0F\x0Ba\x01\xE0\x83\x01R[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\x000hWb\x000hb\x000=V[P`\x01\x01\x90V[`\0a\x01\0\x82\x84\x03\x12\x15b\x000\x83W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x000\xA9Wb\x000\xA9b\0$\xE4V[`@R\x82Qb\x000\xB9\x81b\0%\xCDV[\x81R` \x83\x01Qb\x000\xCB\x81b\0&\xD9V[` \x82\x01R`@\x83\x01Qb\x000\xE0\x81b\0&\xD9V[`@\x82\x01Rb\x000\xF4\x84``\x85\x01b\0-\xA7V[``\x82\x01R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x85\x16\x81R`\x0F\x84\x81\x0B` \x83\x01R\x83\x90\x0B`@\x82\x01Ra\x01\0\x81\x01b\x001h``\x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`\xC0\x81\x01b\0(\xE7` \x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15b\x001\xD9W`\0\x80\xFD[\x815b\0(\xE7\x81b\0%\xCDV[`\0` \x82\x84\x03\x12\x15b\x001\xF9W`\0\x80\xFD[\x815b\0(\xE7\x81b\0&\xD9V[`\0` \x80\x83R`\x80\x83\x01c\xFF\xFF\xFF\xFF\x85Q\x16\x82\x85\x01R\x81\x85\x01Q`\x0F\x0B`@\x85\x01R`@\x85\x01Q``\x80\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P\x84\x83\x01\x93P`\0\x92P[\x80\x83\x10\x15b\0&SW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90b\x002JV[`\0[\x83\x81\x10\x15b\x002\x89W\x81\x81\x01Q\x83\x82\x01R` \x01b\x002oV[\x83\x81\x11\x15b\x002\x99W`\0\x84\x84\x01R[PPPPV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qb\x002\xC2\x81`\x01\x85\x01` \x87\x01b\x002lV[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0\x81Q\x80\x84Rb\x002\xEA\x81` \x86\x01` \x86\x01b\x002lV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0(\xE7` \x83\x01\x84b\x002\xD0V[`\0` \x80\x83\x85\x03\x12\x15b\x003'W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\x003?W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13b\x003QW`\0\x80\xFD[\x80Qb\x003bb\0&\x05\x82b\0%\xA6V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15b\x003\x82W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15b\x0002W\x83Qb\x003\x9D\x81b\0%\xCDV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90b\x003\x87V[`\0` \x82\x84\x03\x12\x15b\x003\xC0W`\0\x80\xFD[\x81Qb\0(\xE7\x81b\0(\xB1V[`\0` \x82\x84\x03\x12\x15b\x003\xE0W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15b\x003\xFDWb\x003\xFDb\x000=V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\x004\x15W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0(\xE7W`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\x004DWb\x004Db\x000=V[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15b\x004\x8EW\x81`\0\x19\x04\x82\x11\x15b\x004rWb\x004rb\x000=V[\x80\x85\x16\x15b\x004\x80W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\x004RV[P\x92P\x92\x90PV[`\0\x82b\x004\xA7WP`\x01b\0\"HV[\x81b\x004\xB6WP`\0b\0\"HV[\x81`\x01\x81\x14b\x004\xCFW`\x02\x81\x14b\x004\xDAWb\x004\xFAV[`\x01\x91PPb\0\"HV[`\xFF\x84\x11\x15b\x004\xEEWb\x004\xEEb\x000=V[PP`\x01\x82\x1Bb\0\"HV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\x005\x1FWP\x81\x81\nb\0\"HV[b\x005+\x83\x83b\x004MV[\x80`\0\x19\x04\x82\x11\x15b\x005BWb\x005Bb\x000=V[\x02\x93\x92PPPV[`\0b\0(\xE7`\xFF\x84\x16\x83b\x004\x96V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\x005xWb\x005xb\x000=V[P\x02\x90V[`\0`\xA0\x82\x84\x03\x12\x15b\x005\x90W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x005\xB6Wb\x005\xB6b\0$\xE4V[`@R\x82Qb\x005\xC6\x81b\0&\xD9V[\x81R` \x83\x01Qb\x005\xD8\x81b\0&\xD9V[` \x82\x01R`@\x83\x01Qb\x005\xED\x81b\0&\xD9V[`@\x82\x01R``\x83\x01Qb\x006\x02\x81b\0&\xD9V[``\x82\x01R`\x80\x83\x01Qb\x006\x17\x81b\0&\xD9V[`\x80\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15b\x006_Wb\x006_b\x000=V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15b\x006\x8EWb\x006\x8Eb\x000=V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15b\x006\xADWb\x006\xADb\x000=V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15b\x006\xC6Wb\x006\xC6b\x000=V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R\x83\x16` \x82\x01R``\x81\x01`\x02\x83\x10b\x007\rWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82`@\x83\x01R\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15b\x007.W`\0\x80\xFD[b\0(\xE7\x83\x83b\0&\xFBV[`\0a\x01\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qb\x007\xB4``\x84\x01\x82\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15b\x007\xCEW`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x007\xF4Wb\x007\xF4b\0$\xE4V[`@R\x825b\08\x04\x81b\0(\xB1V[\x81R` \x83\x015b\08\x16\x81b\0&\xD9V[` \x82\x01R`@\x83\x015b\08+\x81b\0&\xD9V[`@\x82\x01R``\x83\x015b\08@\x81b\0&\xD9V[``\x82\x01Rb\08S`\x80\x84\x01b\0&\xE9V[`\x80\x82\x01Rb\08f`\xA0\x84\x01b\0&\xE9V[`\xA0\x82\x01Rb\08y`\xC0\x84\x01b\0&\xE9V[`\xC0\x82\x01R\x93\x92PPPV[`\0a\x02\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Qb\09,`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\xA0\x83\x01Q\x80Q`\x03\x90\x81\x0Ba\x01`\x85\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x85\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x85\x01R``\x82\x01Q\x90\x0Ba\x01\xC0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xE0\x84\x01Rb\x007\xB4V\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0C\x958\x03\x80a\x0C\x95\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01IV[a\083a\0\xE1V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x03\x84\x90U`\x04\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90UG\x80\x15a\0\xD7W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82\x90`\0\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\0\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xD3V[``\x91P[PPP[PPPPPa\x01\x9EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01FW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01_W`\0\x80\xFD[\x84Qa\x01j\x81a\x011V[` \x86\x01Q\x90\x94Pa\x01{\x81a\x011V[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa\x01\x93\x81a\x011V[\x93\x96\x92\x95P\x90\x93PPV[a\n\xE8\x80a\x01\xAD`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0NW\x80cT\xFDMP\x14a\x01\x03W\x80cqP\x18\xA6\x14a\x01$W\x80c\x8D\xA5\xCB[\x14a\x019W\x80c\xF2\xFD\xE3\x8B\x14a\x01aW`\0\x80\xFD[\x80c&\x08o\x07\x14a\0\xB9W\x80cPC\x1C\xE4\x14a\0\xCEW\x80cQ\xCF\xF8\xD9\x14a\0\xE3W`\0\x80\xFD[6a\0\xB4W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xB2W=`\0\x80>=`\0\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xC5W`\0\x80\xFD[Pa\0\xB2a\x01\x81V[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\0\xB2a\x04\x9CV[4\x80\x15a\0\xEFW`\0\x80\xFD[Pa\0\xB2a\0\xFE6`\x04a\x08\x86V[a\x05dV[4\x80\x15a\x01\x0FW`\0\x80\xFD[P`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x010W`\0\x80\xFD[Pa\0\xB2a\x05\xEDV[4\x80\x15a\x01EW`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1BV[4\x80\x15a\x01mW`\0\x80\xFD[Pa\0\xB2a\x01|6`\x04a\x08\x86V[a\x06\x01V[`\x02T`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xF2\x91\x90\x81\x01\x90a\x08\xD9V[\x90P`\0[\x81Q\x81\x10\x15a\x04\x98W`\0\x82\x82\x81Q\x81\x10a\x02\x14Wa\x02\x14a\t\x9EV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x97\x91\x90a\t\xB4V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03a\x91\x90a\t\xD1V[\x90P\x80\x15a\x04\x81W`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x90\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE0\x91\x90a\t\xEAV[P`\x01T`\x03T`@Qc\"\x1F\t9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`D\x82\x01R`\x80`d\x82\x01R`\x02`\x84\x82\x01Ra-1`\xF0\x1B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\"\x1F\t9\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04hW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04|W=`\0\x80>=`\0\xFD[PPPP[PPPP\x80\x80a\x04\x90\x90a\n\x0CV[\x91PPa\x01\xF7V[PPV[a\x04\xA4a\x06\x91V[`@QG\x90`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\xE8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xEDV[``\x91P[PP\x90P\x80a\x04\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FFailed to transfer native token `D\x82\x01R\x7Fto owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEBV[a\x05la\x06\x91V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD7\x91\x90a\t\xD1V[\x90Pa\x04\x98`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x06\xEBV[a\x05\xF5a\x06\x91V[a\x05\xFF`\0a\x08\tV[V[a\x06\ta\x06\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEBV[a\x06\x8E\x81a\x08\tV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xEBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x07\\\x91\x90a\ncV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\x99W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\x9EV[``\x91P[P\x91P\x91P\x81\x80\x15a\x07\xC8WP\x80Q\x15\x80a\x07\xC8WP\x80\x80` \x01\x90Q\x81\x01\x90a\x07\xC8\x91\x90a\t\xEAV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a*#`\xF1\x1B\x81RP\x90a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xEB\x91\x90a\n\x7FV[PPPPPPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x8EW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08\x98W`\0\x80\xFD[\x815a\x08\xA3\x81a\x08qV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xD4W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x08\xECW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x04W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t\x18W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\t*Wa\t*a\x08\xAAV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\tOWa\tOa\x08\xAAV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\tmW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\t\x92Wa\t\x83\x85a\x08\xC0V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\trV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\t\xC6W`\0\x80\xFD[\x81Qa\x08\xA3\x81a\x08qV[`\0` \x82\x84\x03\x12\x15a\t\xE3W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\t\xFCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xA3W`\0\x80\xFD[`\0`\x01\x82\x01a\n,WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\nNW\x81\x81\x01Q\x83\x82\x01R` \x01a\n6V[\x83\x81\x11\x15a\n]W`\0\x84\x84\x01R[PPPPV[`\0\x82Qa\nu\x81\x84` \x87\x01a\n3V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\n\x9E\x81`@\x85\x01` \x87\x01a\n3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 T\xF4\xF7]m-Pq\xFE\xFF\x99\x97\xDF\xC4\x1A\xF5\xFB\xD6\xED\xEE\xE4Mb\xF7\xBD\xA66(\x9Cl\xF6\xB9dsolcC\0\x08\r\x003\xA2dipfsX\"\x12 \x97\xBD\xDE\xC6\x90\xA0\x87\x9C\xCDz\xDC\xC2\xBD\xC9\xFBM\xEE\xD0\x86\x90\xE6Fl93\x9F\xE28cMT\x90dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static CONTRACTOWNER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x01\xF5W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11b\0\x01\x19W\x80c\xBB\xEF\x84\xB4\x11b\0\0\xAFW\x80c\xC9\xC5\xEF\xAA\x11b\0\0zW\x80c\xC9\xC5\xEF\xAA\x14b\0\x04:W\x80c\xD9\xFB\x99\xC1\x14b\0\x04DW\x80c\xEB\xD6\xC2\x94\x14b\0\x04[W\x80c\xF2\xFD\xE3\x8B\x14b\0\x04rW`\0\x80\xFD[\x80c\xBB\xEF\x84\xB4\x14b\0\x03\xDCW\x80c\xBE\x05\xA6\x9D\x14b\0\x03\xF3W\x80c\xBE\x13\xBA\xC4\x14b\0\x04\x0CW\x80c\xC2'\xDB\x96\x14b\0\x04#W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11b\0\0\xF0W\x80c\x8D\xA5\xCB[\x14b\0\x03\x85W\x80c\x90\x86\xA2\xA5\x14b\0\x03\x97W\x80c\x94\x91+\x80\x14b\0\x03\xAEW\x80c\x9Bj\xBA\x8F\x14b\0\x03\xC5W`\0\x80\xFD[\x80cqP\x18\xA6\x14b\0\x03MW\x80c\x8A)\xE2\xDE\x14b\0\x03WW\x80c\x8A\xB3\xDA\xAE\x14b\0\x03nW`\0\x80\xFD[\x80cG\xE2\xCA\x90\x11b\0\x01\x8FW\x80clU[\x1B\x11b\0\x01fW\x80clU[\x1B\x14b\0\x02\xFEW\x80clleN\x14b\0\x03\x15W\x80cn\x13\xCB\xF3\x14b\0\x03,W\x80cp|\x8BX\x14b\0\x03CW`\0\x80\xFD[\x80cG\xE2\xCA\x90\x14b\0\x02\xB9W\x80cS\x12\xB9\x1F\x14b\0\x02\xD0W\x80cV\xE4\x9E\xF3\x14b\0\x02\xE7W`\0\x80\xFD[\x80c\x19b\xC3\x84\x11b\0\x01\xD0W\x80c\x19b\xC3\x84\x14b\0\x02jW\x80c+\x12l_\x14b\0\x02tW\x80c3\x92\xC5\x85\x14b\0\x02\x8BW\x80c=\xAB\xE0\xD9\x14b\0\x02\xA2W`\0\x80\xFD[\x80c\x12\xEF\xA7\x1B\x14b\0\x01\xFAW\x80c\x14\\\xA3\x80\x14b\0\x02CW\x80c\x17K\x8D^\x14b\0\x02^W[`\0\x80\xFD[b\0\x02&b\0\x02\x0B6`\x04b\0$\xCAV[`\xA3` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\0\x02Mb\0\x04\x89V[`@Q\x90\x15\x15\x81R` \x01b\0\x02:V[b\0\x02hb\0\x04\xA3V[\0[b\0\x02hb\0\x05\rV[b\0\x02hb\0\x02\x856`\x04b\0&^V[b\0\x05qV[b\0\x02hb\0\x02\x9C6`\x04b\0'\x83V[b\0\tuV[b\0\x02hb\0\x02\xB36`\x04b\0(\rV[b\0\t\xEBV[b\0\x02Mb\0\x02\xCA6`\x04b\0(\xC7V[b\0\x0C\rV[b\0\x02hb\0\x02\xE16`\x04b\0)\x0FV[b\0\x0F\xB4V[b\0\x02hb\0\x02\xF86`\x04b\0)_V[b\0\x10\xBBV[b\0\x02hb\0\x03\x0F6`\x04b\0)\xBCV[b\0\x11+V[b\0\x02hb\0\x03&6`\x04b\0*!V[b\0\x12\xCAV[b\0\x02&b\0\x03=6`\x04b\0$\xCAV[b\0\x13|V[b\0\x02hb\0\x146V[b\0\x02hb\0\x14\xC6V[b\0\x02hb\0\x03h6`\x04b\0*]V[b\0\x14\xDCV[b\0\x02hb\0\x03\x7F6`\x04b\0'\x83V[b\0\x16\xEDV[`gT`\x01`\x01`\xA0\x1B\x03\x16b\0\x02&V[b\0\x02hb\0\x03\xA86`\x04b\0$\xCAV[b\0\x17+V[b\0\x02hb\0\x03\xBF6`\x04b\0$\xCAV[b\0\x17\x93V[b\0\x02hb\0\x03\xD66`\x04b\0+\x84V[b\0\x17\xCFV[b\0\x02hb\0\x03\xED6`\x04b\0$\xCAV[b\0\x1AJV[b\0\x03\xFDb\0\x1A\x86V[`@Qb\0\x02:\x91\x90b\0,>V[b\0\x02hb\0\x04\x1D6`\x04b\0,\x8AV[b\0\x1B\xFAV[b\0\x02hb\0\x0446`\x04b\0(\xC7V[b\0\x1CDV[b\0\x03\xFDb\0\x1C\x81V[b\0\x02hb\0\x04U6`\x04b\0,\xB7V[b\0\x1D\xEFV[b\0\x02hb\0\x04l6`\x04b\0)\x0FV[b\0\x1F\xB0V[b\0\x02hb\0\x04\x836`\x04b\0(\xC7V[b\0 \xCEV[`\xA5T`\0\x90\x15\x15\x80b\0\x04\x9EWP`\xA4T\x15\x15[\x90P\x90V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x04\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[b\0\x05\x0B`\xA5`\0b\0#\x96V[V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[b\0\x05\x0B`\xA4`\0b\0#\x96V[b\0\x05{b\0!dV[`\0[`\xA4T\x81\x10\x15b\0\x07mW`\0`\xA4\x82\x81T\x81\x10b\0\x05\xA1Wb\0\x05\xA1b\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x05\xB8\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x05\xE6\x90b\0-dV[\x80\x15b\0\x067W\x80`\x1F\x10b\0\x06\x0BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x067V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x06\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x06Q\x91\x90b\0.$V[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10b\0\x06sWb\0\x06sb\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x06\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fspot id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\xF4V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Qc\xDF\x14O\xD5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96c\xDF\x14O\xD5\x96b\0\x07\"\x96\x90\x95\x90\x94\x93\x92\x91`\x04\x01b\0/`V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x07=W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x07RW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x07d\x90b\x000SV[\x91PPb\0\x05~V[Pb\0\x07|`\xA4`\0b\0#\x96V[`\0[`\xA5T\x81\x10\x15b\0\tbW`\0`\xA5\x82\x81T\x81\x10b\0\x07\xA2Wb\0\x07\xA2b\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x07\xB9\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\xE7\x90b\0-dV[\x80\x15b\0\x088W\x80`\x1F\x10b\0\x08\x0CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x088V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x08\x1AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x08R\x91\x90b\x000oV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10b\0\x08tWb\0\x08tb\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x08\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fperp id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\xF4V[`\x9BT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xA4Y\x89\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x94c\xA4Y\x89\xAB\x94b\0\t\x17\x94\x90\x93\x90\x92\x91`\x04\x01b\x001\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t2W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\tGW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\tY\x90b\x000SV[\x91PPb\0\x07\x7FV[Pb\0\tq`\xA5`\0b\0#\x96V[PPV[b\0\t\x7Fb\0!dV[`\x9BT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\t\xB3\x90\x85\x90\x85\x90`\x04\x01b\x001qV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\t\xE3W=`\0\x80>=`\0\xFD[PPPPPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[\x84\x83\x14b\0\n\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0[\x85\x81\x10\x15b\0\x0C\x04W`\0`@Q\x80``\x01`@R\x80\x89\x89\x85\x81\x81\x10b\0\n\xB1Wb\0\n\xB1b\0-NV[\x90P` \x02\x01` \x81\x01\x90b\0\n\xC8\x91\x90b\x001\xC6V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x87\x85\x81\x81\x10b\0\n\xE8Wb\0\n\xE8b\0-NV[\x90P` \x02\x01` \x81\x01\x90b\0\n\xFF\x91\x90b\x001\xE6V[`\x0F\x0B\x81R` \x01\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91P`\x14\x90P\x82`@Q` \x01b\0\x0BW\x91\x90b\x002\x06V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x0Bw\x92\x91` \x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x0B\xB8\x90\x84\x90`\x04\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0B\xE8W=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x0B\xFB\x90b\x000SV[\x91PPb\0\n\x86V[PPPPPPPV[`\0\x80`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0CdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x0C\x8E\x91\x90\x81\x01\x90b\x003\x13V[\x90P`\0[\x81Q\x81\x10\x15b\0\x0F\xAAW`\0\x82\x82\x81Q\x81\x10b\0\x0C\xB4Wb\0\x0C\xB4b\0-NV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r:\x91\x90b\x003\xADV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\r\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x04\xF4V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x82\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\x06\x91\x90b\x003\xCDV[`\x9FT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x84\x16\x03b\0\x0E9Wb\0\x0E6`\x01`\x01`\xA0\x1B\x03\x89\x161\x82b\x003\xE7V[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0ExW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\x9E\x91\x90b\x004\x02V[b\0\x0E\xAB\x90`\x12b\x004'V[b\0\x0E\xB8\x90`\nb\x005JV[b\0\x0E\xC4\x90\x82b\x005[V[`\x9AT`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x0F\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0FA\x91\x90b\x005}V[`\x80\x01Q\x90Pb\0\x0F\\g\r\xE0\xB6\xB3\xA7d\0\0`\x05b\x006#V[`\x0F\x0Bb\0\x0Fw\x83\x83`\x0F\x0Bb\0!\xC0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x12b\0\x0F\x8FWP`\x01\x98\x97PPPPPPPPV[PPPPP\x80\x80b\0\x0F\xA1\x90b\x000SV[\x91PPb\0\x0C\x93V[P`\0\x93\x92PPPV[b\0\x0F\xBEb\0!dV[`\x9DT`@\x80Qc\x1Ft\xAC\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFB\xA5`\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x10\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10/\x91\x90b\x003\xADV[`@Qc\xFD\x8CR\xCD`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x92P\x90\x82\x16\x90c\xFD\x8CR\xCD\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x10\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x10\xB1W=`\0\x80>=`\0\xFD[PPPPPPPPV[b\0\x10\xC5b\0!dV[`\x9DT`@QcV\xE4\x9E\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cV\xE4\x9E\xF3\x90b\0\x10\xFB\x90\x86\x90\x86\x90\x86\x90`\x04\x01b\x006\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x11\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0C\x04W=`\0\x80>=`\0\xFD[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x11\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0b\0\x11\x8Db\0\x1A\x86V[\x90P`\0[\x81Q\x81\x10\x15b\0\x12@W\x81\x81\x81Q\x81\x10b\0\x11\xB1Wb\0\x11\xB1b\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x03b\0\x12+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a perp p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[\x80b\0\x127\x81b\x000SV[\x91PPb\0\x11\x92V[P`\xA5`@Q\x80`\x80\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85`\x0F\x0B\x81R` \x01\x84\x806\x03\x81\x01\x90b\0\x12\x7F\x91\x90b\x007\x1BV[\x90R`@Qb\0\x12\x93\x91\x90` \x01b\x007:V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\t\xE3\x94\x91\x90\x92\x01\x92\x01\x90b\0#\xB6V[b\0\x12\xD4b\0!dV[`@\x80Q\x80\x82\x01\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x81\x84\x01R\x82Q\x80\x82\x03\x84\x01\x81R``\x82\x01\x90\x93R\x90\x91`\0\x91b\0\x13;\x91`\x12\x91`\x80\x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x10\x81\x90\x84\x90`\x04\x01b\x002\xFEV[`\x9CT`\x9AT`\x9FT`@Q`\0\x93\x84\x93`\x01\x93`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93\x91\x83\x16\x92\x88\x92\x91\x16\x90b\0\x13\xB0\x90b\0$EV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R`@\x83\x01\x91\x90\x91R\x90\x91\x16``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x13\xF9W=`\0\x80>=`\0\xFD[P`\0\x93\x84R`\xA3` R`@\x90\x93 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UP\x90\x91\x90PV[b\0\x14@b\0!dV[`@\x80Q`\t`\xF8\x1B` \x82\x01R\x81Q`\x01\x81\x83\x03\x01\x81R`!\x82\x01\x92\x83\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x90\x93R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x14\x8F\x90\x84\x90`%\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x14\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x14\xBFW=`\0\x80>=`\0\xFD[PPPPPV[b\0\x14\xD0b\0!dV[b\0\x05\x0B`\0b\0\"NV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x14\xFDWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x15\x19WP0;\x15\x80\x15b\0\x15\x19WP`\0T`\xFF\x16`\x01\x14[b\0\x15\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x04\xF4V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x15\xB1W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x88\x163\x14b\0\x16\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fexpected deployed to initialize\0`D\x82\x01R`d\x01b\0\x04\xF4V[b\0\x16\x15b\0\"\xADV[b\0\x16 \x89b\0 \xCEV[`\x99\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x17\x90\x92U`\x9A\x80T\x82\x16\x8A\x84\x16\x17\x90U`\x9B\x80T\x82\x16\x89\x84\x16\x17\x90U`\x9C\x80T\x82\x16\x88\x84\x16\x17\x90U`\x9D\x80T\x82\x16\x87\x84\x16\x17\x90U`\x9E\x80T\x82\x16\x86\x84\x16\x17\x90U`\x9F\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15b\0\x16\xE2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[b\0\x16\xF7b\0!dV[`\x9AT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\t\xB3\x90\x85\x90\x85\x90`\x04\x01b\x001qV[`\0\x81\x81R`\xA3` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x17WWb\0\x17T\x82b\0\x13|V[\x90P[\x80`\x01`\x01`\xA0\x1B\x03\x16c&\x08o\x07`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t\xCEW`\0\x80\xFD[b\0\x17\x9Db\0!dV[`\x9DT`@Qc\x01)\"W`\xE7\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\x91+\x80\x90`$\x01b\0\x14\x8FV[b\0\x17\xD9b\0!dV[\x82Q\x84Q\x14b\0\x18\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[\x81Q\x84Q\x14b\0\x18aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[\x80Q\x84Q\x14b\0\x18\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0[\x84Q\x81\x10\x15b\0\x14\xBFW`\0`@Q\x80`\x80\x01`@R\x80\x87\x84\x81Q\x81\x10b\0\x18\xD4Wb\0\x18\xD4b\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x84\x81Q\x81\x10b\0\x18\xFCWb\0\x18\xFCb\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85\x84\x81Q\x81\x10b\0\x19$Wb\0\x19$b\0-NV[` \x02` \x01\x01Q`\x0F\x0B\x81R` \x01\x84\x84\x81Q\x81\x10b\0\x19IWb\0\x19Ib\0-NV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0F\x0B\x90R\x90P`\0`\x18`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x86\x01Q\x16\x81\x83\x01R\x90\x84\x01Q`\x0F\x90\x81\x0B``\x80\x84\x01\x91\x90\x91R\x85\x01Q\x90\x0B`\x80\x82\x01R`\xA0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x19\xBD\x92\x91` \x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x19\xFE\x90\x84\x90`\x04\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1A\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1A.W=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x1AA\x90b\x000SV[\x91PPb\0\x18\xA8V[b\0\x1ATb\0!dV[`\x9ET`@Qc.\xFB\xE1-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\xEF\x84\xB4\x90`$\x01b\0\x14\x8FV[`\xA5T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x1A\xAAWb\0\x1A\xAAb\0$\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x1A\xD4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA5T\x81\x10\x15b\0\x1B\xF4W`\0`\xA5\x82\x81T\x81\x10b\0\x1A\xFDWb\0\x1A\xFDb\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x1B\x14\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1BB\x90b\0-dV[\x80\x15b\0\x1B\x93W\x80`\x1F\x10b\0\x1BgWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1B\x93V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1BuW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x1B\xAD\x91\x90b\x000oV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x1B\xC9Wb\0\x1B\xC9b\0-NV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x1B\xEB\x81b\x000SV[\x91PPb\0\x1A\xDAV[P\x91\x90PV[b\0\x1C\x04b\0!dV[`\x9ET`@Qc/\x84\xEE\xB1`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\x13\xBA\xC4\x90`d\x01b\0\x10\xFBV[b\0\x1CNb\0!dV[`\x9DT`@Qca\x13\xED\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC2'\xDB\x96\x90`$\x01b\0\x14\x8FV[`\xA4T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x1C\xA5Wb\0\x1C\xA5b\0$\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x1C\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA4T\x81\x10\x15b\0\x1B\xF4W`\0`\xA4\x82\x81T\x81\x10b\0\x1C\xF8Wb\0\x1C\xF8b\0-NV[\x90`\0R` `\0 \x01\x80Tb\0\x1D\x0F\x90b\0-dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1D=\x90b\0-dV[\x80\x15b\0\x1D\x8EW\x80`\x1F\x10b\0\x1DbWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1D\x8EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1DpW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x1D\xA8\x91\x90b\0.$V[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x1D\xC4Wb\0\x1D\xC4b\0-NV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x1D\xE6\x81b\x000SV[\x91PPb\0\x1C\xD5V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1EEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x04\xF4V[`\0b\0\x1EQb\0\x1C\x81V[\x90P`\0[\x81Q\x81\x10\x15b\0\x1F\x04W\x81\x81\x81Q\x81\x10b\0\x1EuWb\0\x1Eub\0-NV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x88c\xFF\xFF\xFF\xFF\x16\x03b\0\x1E\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a spot p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[\x80b\0\x1E\xFB\x81b\x000SV[\x91PPb\0\x1EVV[P`\xA4`@Q\x80`\xC0\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\x0F\x0B\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85\x806\x03\x81\x01\x90b\0\x1FO\x91\x90b\x007\xBBV[\x81R` \x01b\0\x1Fe6\x86\x90\x03\x86\x01\x86b\x007\x1BV[\x90R`@Qb\0\x1Fy\x91\x90` \x01b\08\x85V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x10\xB1\x94\x91\x90\x92\x01\x92\x01\x90b\0#\xB6V[b\0\x1F\xBAb\0!dV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x85\x16\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x10`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0 ]\x92\x91` \x01b\x002\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0 \x9E\x90\x84\x90`\x04\x01b\x002\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0 \xB9W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x16\xE2W=`\0\x80>=`\0\xFD[b\0 \xD8b\0!dV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0!VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x04\xF4V[b\0!a\x81b\0\"NV[PV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x04\xF4V[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90b\0\"\x08WPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90b\0\"DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x04\xF4\x91\x90b\x002\xFEV[P\x90P[\x92\x91PPV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0#\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[b\0\x05\x0B`\0Ta\x01\0\x90\x04`\xFF\x16b\0#\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x04\xF4V[b\0\x05\x0B3b\0\"NV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0!a\x91\x90b\0$SV[\x82\x80Tb\0#\xC4\x90b\0-dV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0#\xE8W`\0\x85Ub\0$3V[\x82`\x1F\x10b\0$\x03W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0$3V[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0$3W\x91\x82\x01[\x82\x81\x11\x15b\0$3W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0$\x16V[Pb\0$A\x92\x91Pb\0$tV[P\x90V[a\x0C\x95\x80b\09{\x839\x01\x90V[\x80\x82\x11\x15b\0$AW`\0b\0$j\x82\x82b\0$\x8BV[P`\x01\x01b\0$SV[[\x80\x82\x11\x15b\0$AW`\0\x81U`\x01\x01b\0$uV[P\x80Tb\0$\x99\x90b\0-dV[`\0\x82U\x80`\x1F\x10b\0$\xAAWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0!a\x91\x90b\0$tV[`\0` \x82\x84\x03\x12\x15b\0$\xDDW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0% Wb\0% b\0$\xE4V[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0% Wb\0% b\0$\xE4V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0% Wb\0% b\0$\xE4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0%\x9EWb\0%\x9Eb\0$\xE4V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0%\xC3Wb\0%\xC3b\0$\xE4V[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0!aW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12b\0%\xF2W`\0\x80\xFD[\x815` b\0&\x0Bb\0&\x05\x83b\0%\xA6V[b\0%rV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0&+W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0&SW\x805b\0&E\x81b\0%\xCDV[\x83R\x91\x83\x01\x91\x83\x01b\0&/V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0&rW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0&\x8BW`\0\x80\xFD[b\0&\x99\x86\x83\x87\x01b\0%\xE0V[\x93P` \x85\x015\x91P\x80\x82\x11\x15b\0&\xB0W`\0\x80\xFD[Pb\0&\xBF\x85\x82\x86\x01b\0%\xE0V[\x91PP\x92P\x92\x90PV[\x80`\x03\x0B\x81\x14b\0!aW`\0\x80\xFD[\x80`\x0F\x0B\x81\x14b\0!aW`\0\x80\xFD[\x805b\0&\xF6\x81b\0&\xD9V[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15b\0'\x0EW`\0\x80\xFD[b\0'\x18b\0$\xFAV[\x90P\x815b\0''\x81b\0&\xC9V[\x81R` \x82\x015b\0'9\x81b\0&\xC9V[` \x82\x01R`@\x82\x015b\0'N\x81b\0&\xC9V[`@\x82\x01R``\x82\x015b\0'c\x81b\0&\xC9V[``\x82\x01R`\x80\x82\x015b\0'x\x81b\0&\xD9V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15b\0'\x97W`\0\x80\xFD[\x825b\0'\xA4\x81b\0%\xCDV[\x91Pb\0'\xB5\x84` \x85\x01b\0&\xFBV[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0'\xD1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0'\xEAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0(\x06W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15b\0('W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0(@W`\0\x80\xFD[b\0(N\x8A\x83\x8B\x01b\0'\xBEV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15b\0(hW`\0\x80\xFD[b\0(v\x8A\x83\x8B\x01b\0'\xBEV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15b\0(\x90W`\0\x80\xFD[Pb\0(\x9F\x89\x82\x8A\x01b\0'\xBEV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0!aW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15b\0(\xDAW`\0\x80\xFD[\x815b\0(\xE7\x81b\0(\xB1V[\x93\x92PPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0&\xF6W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0)%W`\0\x80\xFD[\x835b\0)2\x81b\0%\xCDV[\x92Pb\0)B` \x85\x01b\0(\xEEV[\x91P`@\x84\x015b\0)T\x81b\0(\xB1V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0)uW`\0\x80\xFD[\x835b\0)\x82\x81b\0(\xB1V[\x92P` \x84\x015b\0)\x94\x81b\0(\xB1V[\x91P`@\x84\x015`\x02\x81\x10b\0)TW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15b\0\x1B\xF4W`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15b\0)\xD4W`\0\x80\xFD[\x845b\0)\xE1\x81b\0%\xCDV[\x93P` \x85\x015b\0)\xF3\x81b\0&\xD9V[\x92P`@\x85\x015b\0*\x05\x81b\0&\xD9V[\x91Pb\0*\x16\x86``\x87\x01b\0)\xA9V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15b\0*5W`\0\x80\xFD[b\0*@\x83b\0(\xEEV[\x91P` \x83\x015b\0*R\x81b\0(\xB1V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15b\0*{W`\0\x80\xFD[\x885b\0*\x88\x81b\0(\xB1V[\x97P` \x89\x015b\0*\x9A\x81b\0(\xB1V[\x96P`@\x89\x015b\0*\xAC\x81b\0(\xB1V[\x95P``\x89\x015b\0*\xBE\x81b\0(\xB1V[\x94P`\x80\x89\x015b\0*\xD0\x81b\0(\xB1V[\x93P`\xA0\x89\x015b\0*\xE2\x81b\0(\xB1V[\x92P`\xC0\x89\x015b\0*\xF4\x81b\0(\xB1V[\x91P`\xE0\x89\x015b\0+\x06\x81b\0(\xB1V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x82`\x1F\x83\x01\x12b\0+)W`\0\x80\xFD[\x815` b\0+<b\0&\x05\x83b\0%\xA6V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0+\\W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0&SW\x805b\0+v\x81b\0&\xD9V[\x83R\x91\x83\x01\x91\x83\x01b\0+`V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0+\x9BW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0+\xB4W`\0\x80\xFD[b\0+\xC2\x88\x83\x89\x01b\0%\xE0V[\x95P` \x87\x015\x91P\x80\x82\x11\x15b\0+\xD9W`\0\x80\xFD[b\0+\xE7\x88\x83\x89\x01b\0%\xE0V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\0+\xFEW`\0\x80\xFD[b\0,\x0C\x88\x83\x89\x01b\0+\x17V[\x93P``\x87\x015\x91P\x80\x82\x11\x15b\0,#W`\0\x80\xFD[Pb\0,2\x87\x82\x88\x01b\0+\x17V[\x91PP\x92\x95\x91\x94P\x92PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0,~W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0,ZV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0,\xA0W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15b\0,\xD3W`\0\x80\xFD[\x875b\0,\xE0\x81b\0%\xCDV[\x96P` \x88\x015b\0,\xF2\x81b\0%\xCDV[\x95P`@\x88\x015b\0-\x04\x81b\0&\xD9V[\x94P``\x88\x015b\0-\x16\x81b\0&\xD9V[\x93P`\xE0`\x7F\x19\x82\x01\x12\x15b\0-+W`\0\x80\xFD[P`\x80\x87\x01\x91Pb\0-B\x88a\x01`\x89\x01b\0)\xA9V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0-yW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x1B\xF4WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80Qb\0&\xF6\x81b\0&\xD9V[`\0`\xA0\x82\x84\x03\x12\x15b\0-\xBAW`\0\x80\xFD[b\0-\xC4b\0$\xFAV[\x90P\x81Qb\0-\xD3\x81b\0&\xC9V[\x81R` \x82\x01Qb\0-\xE5\x81b\0&\xC9V[` \x82\x01R`@\x82\x01Qb\0-\xFA\x81b\0&\xC9V[`@\x82\x01R``\x82\x01Qb\0.\x0F\x81b\0&\xC9V[``\x82\x01R`\x80\x82\x01Qb\0'x\x81b\0&\xD9V[`\0\x81\x83\x03a\x02\0\x81\x12\x15b\0.9W`\0\x80\xFD[b\0.Cb\0%&V[\x83Qb\0.P\x81b\0%\xCDV[\x81R` \x84\x01Qb\0.b\x81b\0%\xCDV[` \x82\x01R`@\x84\x01Qb\0.w\x81b\0&\xD9V[`@\x82\x01R``\x84\x01Qb\0.\x8C\x81b\0&\xD9V[``\x82\x01R`\xE0`\x7F\x19\x83\x01\x12\x15b\0.\xA4W`\0\x80\xFD[b\0.\xAEb\0%LV[\x91P`\x80\x84\x01Qb\0.\xC0\x81b\0(\xB1V[\x82R`\xA0\x84\x01Qb\0.\xD2\x81b\0&\xD9V[` \x83\x01R`\xC0\x84\x01Qb\0.\xE7\x81b\0&\xD9V[`@\x83\x01R`\xE0\x84\x01Qb\0.\xFC\x81b\0&\xD9V[``\x83\x01Rb\0/\x10a\x01\0\x85\x01b\0-\x9AV[`\x80\x83\x01Rb\0/$a\x01 \x85\x01b\0-\x9AV[`\xA0\x83\x01Rb\0/8a\x01@\x85\x01b\0-\x9AV[`\xC0\x83\x01R\x81`\x80\x82\x01Rb\0/S\x85a\x01`\x86\x01b\0-\xA7V[`\xA0\x82\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x0F\x85\x81\x0B`@\x83\x01R\x84\x90\x0B``\x82\x01Ra\x02\0\x81\x01b\0/\xEF`\x80\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[\x82Q`\x03\x90\x81\x0Ba\x01`\x84\x01R` \x84\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x84\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x84\x01Q\x90\x0Ba\x01\xC0\x83\x01R`\x80\x83\x01Q`\x0F\x0Ba\x01\xE0\x83\x01R[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\x000hWb\x000hb\x000=V[P`\x01\x01\x90V[`\0a\x01\0\x82\x84\x03\x12\x15b\x000\x83W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x000\xA9Wb\x000\xA9b\0$\xE4V[`@R\x82Qb\x000\xB9\x81b\0%\xCDV[\x81R` \x83\x01Qb\x000\xCB\x81b\0&\xD9V[` \x82\x01R`@\x83\x01Qb\x000\xE0\x81b\0&\xD9V[`@\x82\x01Rb\x000\xF4\x84``\x85\x01b\0-\xA7V[``\x82\x01R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x85\x16\x81R`\x0F\x84\x81\x0B` \x83\x01R\x83\x90\x0B`@\x82\x01Ra\x01\0\x81\x01b\x001h``\x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`\xC0\x81\x01b\0(\xE7` \x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15b\x001\xD9W`\0\x80\xFD[\x815b\0(\xE7\x81b\0%\xCDV[`\0` \x82\x84\x03\x12\x15b\x001\xF9W`\0\x80\xFD[\x815b\0(\xE7\x81b\0&\xD9V[`\0` \x80\x83R`\x80\x83\x01c\xFF\xFF\xFF\xFF\x85Q\x16\x82\x85\x01R\x81\x85\x01Q`\x0F\x0B`@\x85\x01R`@\x85\x01Q``\x80\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P\x84\x83\x01\x93P`\0\x92P[\x80\x83\x10\x15b\0&SW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90b\x002JV[`\0[\x83\x81\x10\x15b\x002\x89W\x81\x81\x01Q\x83\x82\x01R` \x01b\x002oV[\x83\x81\x11\x15b\x002\x99W`\0\x84\x84\x01R[PPPPV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qb\x002\xC2\x81`\x01\x85\x01` \x87\x01b\x002lV[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0\x81Q\x80\x84Rb\x002\xEA\x81` \x86\x01` \x86\x01b\x002lV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0(\xE7` \x83\x01\x84b\x002\xD0V[`\0` \x80\x83\x85\x03\x12\x15b\x003'W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\x003?W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13b\x003QW`\0\x80\xFD[\x80Qb\x003bb\0&\x05\x82b\0%\xA6V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15b\x003\x82W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15b\x0002W\x83Qb\x003\x9D\x81b\0%\xCDV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90b\x003\x87V[`\0` \x82\x84\x03\x12\x15b\x003\xC0W`\0\x80\xFD[\x81Qb\0(\xE7\x81b\0(\xB1V[`\0` \x82\x84\x03\x12\x15b\x003\xE0W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15b\x003\xFDWb\x003\xFDb\x000=V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\x004\x15W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0(\xE7W`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\x004DWb\x004Db\x000=V[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15b\x004\x8EW\x81`\0\x19\x04\x82\x11\x15b\x004rWb\x004rb\x000=V[\x80\x85\x16\x15b\x004\x80W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\x004RV[P\x92P\x92\x90PV[`\0\x82b\x004\xA7WP`\x01b\0\"HV[\x81b\x004\xB6WP`\0b\0\"HV[\x81`\x01\x81\x14b\x004\xCFW`\x02\x81\x14b\x004\xDAWb\x004\xFAV[`\x01\x91PPb\0\"HV[`\xFF\x84\x11\x15b\x004\xEEWb\x004\xEEb\x000=V[PP`\x01\x82\x1Bb\0\"HV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\x005\x1FWP\x81\x81\nb\0\"HV[b\x005+\x83\x83b\x004MV[\x80`\0\x19\x04\x82\x11\x15b\x005BWb\x005Bb\x000=V[\x02\x93\x92PPPV[`\0b\0(\xE7`\xFF\x84\x16\x83b\x004\x96V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\x005xWb\x005xb\x000=V[P\x02\x90V[`\0`\xA0\x82\x84\x03\x12\x15b\x005\x90W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x005\xB6Wb\x005\xB6b\0$\xE4V[`@R\x82Qb\x005\xC6\x81b\0&\xD9V[\x81R` \x83\x01Qb\x005\xD8\x81b\0&\xD9V[` \x82\x01R`@\x83\x01Qb\x005\xED\x81b\0&\xD9V[`@\x82\x01R``\x83\x01Qb\x006\x02\x81b\0&\xD9V[``\x82\x01R`\x80\x83\x01Qb\x006\x17\x81b\0&\xD9V[`\x80\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15b\x006_Wb\x006_b\x000=V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15b\x006\x8EWb\x006\x8Eb\x000=V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15b\x006\xADWb\x006\xADb\x000=V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15b\x006\xC6Wb\x006\xC6b\x000=V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R\x83\x16` \x82\x01R``\x81\x01`\x02\x83\x10b\x007\rWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82`@\x83\x01R\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15b\x007.W`\0\x80\xFD[b\0(\xE7\x83\x83b\0&\xFBV[`\0a\x01\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qb\x007\xB4``\x84\x01\x82\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15b\x007\xCEW`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x007\xF4Wb\x007\xF4b\0$\xE4V[`@R\x825b\08\x04\x81b\0(\xB1V[\x81R` \x83\x015b\08\x16\x81b\0&\xD9V[` \x82\x01R`@\x83\x015b\08+\x81b\0&\xD9V[`@\x82\x01R``\x83\x015b\08@\x81b\0&\xD9V[``\x82\x01Rb\08S`\x80\x84\x01b\0&\xE9V[`\x80\x82\x01Rb\08f`\xA0\x84\x01b\0&\xE9V[`\xA0\x82\x01Rb\08y`\xC0\x84\x01b\0&\xE9V[`\xC0\x82\x01R\x93\x92PPPV[`\0a\x02\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Qb\09,`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\xA0\x83\x01Q\x80Q`\x03\x90\x81\x0Ba\x01`\x85\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x85\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x85\x01R``\x82\x01Q\x90\x0Ba\x01\xC0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xE0\x84\x01Rb\x007\xB4V\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0C\x958\x03\x80a\x0C\x95\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01IV[a\083a\0\xE1V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x03\x84\x90U`\x04\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90UG\x80\x15a\0\xD7W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82\x90`\0\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\0\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xD3V[``\x91P[PPP[PPPPPa\x01\x9EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01FW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01_W`\0\x80\xFD[\x84Qa\x01j\x81a\x011V[` \x86\x01Q\x90\x94Pa\x01{\x81a\x011V[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa\x01\x93\x81a\x011V[\x93\x96\x92\x95P\x90\x93PPV[a\n\xE8\x80a\x01\xAD`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0NW\x80cT\xFDMP\x14a\x01\x03W\x80cqP\x18\xA6\x14a\x01$W\x80c\x8D\xA5\xCB[\x14a\x019W\x80c\xF2\xFD\xE3\x8B\x14a\x01aW`\0\x80\xFD[\x80c&\x08o\x07\x14a\0\xB9W\x80cPC\x1C\xE4\x14a\0\xCEW\x80cQ\xCF\xF8\xD9\x14a\0\xE3W`\0\x80\xFD[6a\0\xB4W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xB2W=`\0\x80>=`\0\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xC5W`\0\x80\xFD[Pa\0\xB2a\x01\x81V[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\0\xB2a\x04\x9CV[4\x80\x15a\0\xEFW`\0\x80\xFD[Pa\0\xB2a\0\xFE6`\x04a\x08\x86V[a\x05dV[4\x80\x15a\x01\x0FW`\0\x80\xFD[P`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x010W`\0\x80\xFD[Pa\0\xB2a\x05\xEDV[4\x80\x15a\x01EW`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1BV[4\x80\x15a\x01mW`\0\x80\xFD[Pa\0\xB2a\x01|6`\x04a\x08\x86V[a\x06\x01V[`\x02T`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xF2\x91\x90\x81\x01\x90a\x08\xD9V[\x90P`\0[\x81Q\x81\x10\x15a\x04\x98W`\0\x82\x82\x81Q\x81\x10a\x02\x14Wa\x02\x14a\t\x9EV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x97\x91\x90a\t\xB4V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03a\x91\x90a\t\xD1V[\x90P\x80\x15a\x04\x81W`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x90\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE0\x91\x90a\t\xEAV[P`\x01T`\x03T`@Qc\"\x1F\t9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`D\x82\x01R`\x80`d\x82\x01R`\x02`\x84\x82\x01Ra-1`\xF0\x1B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\"\x1F\t9\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04hW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04|W=`\0\x80>=`\0\xFD[PPPP[PPPP\x80\x80a\x04\x90\x90a\n\x0CV[\x91PPa\x01\xF7V[PPV[a\x04\xA4a\x06\x91V[`@QG\x90`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\xE8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xEDV[``\x91P[PP\x90P\x80a\x04\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FFailed to transfer native token `D\x82\x01R\x7Fto owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEBV[a\x05la\x06\x91V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD7\x91\x90a\t\xD1V[\x90Pa\x04\x98`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x06\xEBV[a\x05\xF5a\x06\x91V[a\x05\xFF`\0a\x08\tV[V[a\x06\ta\x06\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEBV[a\x06\x8E\x81a\x08\tV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xEBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x07\\\x91\x90a\ncV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\x99W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\x9EV[``\x91P[P\x91P\x91P\x81\x80\x15a\x07\xC8WP\x80Q\x15\x80a\x07\xC8WP\x80\x80` \x01\x90Q\x81\x01\x90a\x07\xC8\x91\x90a\t\xEAV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a*#`\xF1\x1B\x81RP\x90a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xEB\x91\x90a\n\x7FV[PPPPPPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x8EW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08\x98W`\0\x80\xFD[\x815a\x08\xA3\x81a\x08qV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xD4W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x08\xECW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x04W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t\x18W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\t*Wa\t*a\x08\xAAV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\tOWa\tOa\x08\xAAV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\tmW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\t\x92Wa\t\x83\x85a\x08\xC0V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\trV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\t\xC6W`\0\x80\xFD[\x81Qa\x08\xA3\x81a\x08qV[`\0` \x82\x84\x03\x12\x15a\t\xE3W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\t\xFCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xA3W`\0\x80\xFD[`\0`\x01\x82\x01a\n,WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\nNW\x81\x81\x01Q\x83\x82\x01R` \x01a\n6V[\x83\x81\x11\x15a\n]W`\0\x84\x84\x01R[PPPPV[`\0\x82Qa\nu\x81\x84` \x87\x01a\n3V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\n\x9E\x81`@\x85\x01` \x87\x01a\n3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 T\xF4\xF7]m-Pq\xFE\xFF\x99\x97\xDF\xC4\x1A\xF5\xFB\xD6\xED\xEE\xE4Mb\xF7\xBD\xA66(\x9Cl\xF6\xB9dsolcC\0\x08\r\x003\xA2dipfsX\"\x12 \x97\xBD\xDE\xC6\x90\xA0\x87\x9C\xCDz\xDC\xC2\xBD\xC9\xFBM\xEE\xD0\x86\x90\xE6Fl93\x9F\xE28cMT\x90dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static CONTRACTOWNER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ContractOwner<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ContractOwner<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ContractOwner<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ContractOwner<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ContractOwner<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ContractOwner))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ContractOwner<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                CONTRACTOWNER_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                CONTRACTOWNER_ABI.clone(),
                CONTRACTOWNER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addEngine` (0x56e49ef3) function
        pub fn add_engine(
            &self,
            engine: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            engine_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [86, 228, 158, 243],
                    (engine, offchain_exchange, engine_type),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOrUpdateProducts` (0x2b126c5f) function
        pub fn add_or_update_products(
            &self,
            spot_ids: ::std::vec::Vec<u32>,
            perp_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 18, 108, 95], (spot_ids, perp_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assignPubKey` (0xbe13bac4) function
        pub fn assign_pub_key(
            &self,
            i: ::ethers::core::types::U256,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 19, 186, 196], (i, x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearPerpAddOrUpdateProductCalls` (0x174b8d5e) function
        pub fn clear_perp_add_or_update_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 75, 141, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearSpotAddOrUpdateProductCalls` (0x1962c384) function
        pub fn clear_spot_add_or_update_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 98, 195, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDirectDepositV1` (0x6e13cbf3) function
        pub fn create_direct_deposit_v1(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([110, 19, 203, 243], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `creditDepositV1` (0x9086a2a5) function
        pub fn credit_deposit_v1(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 134, 162, 165], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deletePubkey` (0xbbef84b4) function
        pub fn delete_pubkey(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 239, 132, 180], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delistProduct` (0x3dabe0d9) function
        pub fn delist_product(
            &self,
            product_ids: ::std::vec::Vec<u32>,
            prices_x18: ::std::vec::Vec<i128>,
            subaccounts: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 171, 224, 217], (product_ids, prices_x18, subaccounts))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `directDepositV1Address` (0x12efa71b) function
        pub fn direct_deposit_v1_address(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([18, 239, 167, 27], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dumpFees` (0x707c8b58) function
        pub fn dump_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasPendingAddOrUpdateProductCalls` (0x145ca380) function
        pub fn has_pending_add_or_update_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([20, 92, 163, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8a29e2de) function
        pub fn initialize(
            &self,
            multisig: ::ethers::core::types::Address,
            deployer: ::ethers::core::types::Address,
            spot_engine: ::ethers::core::types::Address,
            perp_engine: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            clearinghouse: ::ethers::core::types::Address,
            verifier: ::ethers::core::types::Address,
            wrapped_native: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [138, 41, 226, 222],
                    (
                        multisig,
                        deployer,
                        spot_engine,
                        perp_engine,
                        endpoint,
                        clearinghouse,
                        verifier,
                        wrapped_native,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDirectDepositV1Ready` (0x47e2ca90) function
        pub fn is_direct_deposit_v1_ready(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([71, 226, 202, 144], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingPerpAddOrUpdateProductIds` (0xbe05a69d) function
        pub fn pending_perp_add_or_update_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([190, 5, 166, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingSpotAddOrUpdateProductIds` (0xc9c5efaa) function
        pub fn pending_spot_add_or_update_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([201, 197, 239, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `perpUpdateRisk` (0x3392c585) function
        pub fn perp_update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 146, 197, 133], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceXWithdraw` (0xebd6c294) function
        pub fn rebalance_x_withdraw(
            &self,
            product_id: u32,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 214, 194, 148], (product_id, amount, send_to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeWithdrawPoolLiquidity` (0x5312b91f) function
        pub fn remove_withdraw_pool_liquidity(
            &self,
            product_id: u32,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 18, 185, 31], (product_id, amount, send_to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSpreads` (0x94912b80) function
        pub fn set_spreads(
            &self,
            spreads: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 145, 43, 128], spreads)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawPool` (0xc227db96) function
        pub fn set_withdraw_pool(
            &self,
            withdraw_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 39, 219, 150], withdraw_pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spotUpdateRisk` (0x8ab3daae) function
        pub fn spot_update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 179, 218, 174], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitPerpAddOrUpdateProductCall` (0x6c555b1b) function
        pub fn submit_perp_add_or_update_product_call(
            &self,
            product_id: u32,
            size_increment: i128,
            min_size: i128,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [108, 85, 91, 27],
                    (product_id, size_increment, min_size, risk_store),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitSpotAddOrUpdateProductCall` (0xd9fb99c1) function
        pub fn submit_spot_add_or_update_product_call(
            &self,
            product_id: u32,
            quote_id: u32,
            size_increment: i128,
            min_size: i128,
            config: Config,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [217, 251, 153, 193],
                    (
                        product_id,
                        quote_id,
                        size_increment,
                        min_size,
                        config,
                        risk_store,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateTierFeeRates` (0x9b6aba8f) function
        pub fn update_tier_fee_rates(
            &self,
            tier: ::std::vec::Vec<u32>,
            product_id: ::std::vec::Vec<u32>,
            maker_rate_x18: ::std::vec::Vec<i128>,
            taker_rate_x18: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 106, 186, 143],
                    (tier, product_id, maker_rate_x18, taker_rate_x18),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawInsurance` (0x6c6c654e) function
        pub fn withdraw_insurance(
            &self,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 108, 101, 78], (amount, send_to))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ContractOwnerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ContractOwner<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ContractOwnerEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for ContractOwnerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ContractOwnerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ContractOwnerEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ContractOwnerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for ContractOwnerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ContractOwnerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `addEngine` function with signature `addEngine(address,address,uint8)` and selector `0x56e49ef3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addEngine", abi = "addEngine(address,address,uint8)")]
    pub struct AddEngineCall {
        pub engine: ::ethers::core::types::Address,
        pub offchain_exchange: ::ethers::core::types::Address,
        pub engine_type: u8,
    }
    ///Container type for all input parameters for the `addOrUpdateProducts` function with signature `addOrUpdateProducts(uint32[],uint32[])` and selector `0x2b126c5f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addOrUpdateProducts",
        abi = "addOrUpdateProducts(uint32[],uint32[])"
    )]
    pub struct AddOrUpdateProductsCall {
        pub spot_ids: ::std::vec::Vec<u32>,
        pub perp_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `assignPubKey` function with signature `assignPubKey(uint256,uint256,uint256)` and selector `0xbe13bac4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "assignPubKey", abi = "assignPubKey(uint256,uint256,uint256)")]
    pub struct AssignPubKeyCall {
        pub i: ::ethers::core::types::U256,
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clearPerpAddOrUpdateProductCalls` function with signature `clearPerpAddOrUpdateProductCalls()` and selector `0x174b8d5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "clearPerpAddOrUpdateProductCalls",
        abi = "clearPerpAddOrUpdateProductCalls()"
    )]
    pub struct ClearPerpAddOrUpdateProductCallsCall;
    ///Container type for all input parameters for the `clearSpotAddOrUpdateProductCalls` function with signature `clearSpotAddOrUpdateProductCalls()` and selector `0x1962c384`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "clearSpotAddOrUpdateProductCalls",
        abi = "clearSpotAddOrUpdateProductCalls()"
    )]
    pub struct ClearSpotAddOrUpdateProductCallsCall;
    ///Container type for all input parameters for the `createDirectDepositV1` function with signature `createDirectDepositV1(bytes32)` and selector `0x6e13cbf3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "createDirectDepositV1", abi = "createDirectDepositV1(bytes32)")]
    pub struct CreateDirectDepositV1Call {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `creditDepositV1` function with signature `creditDepositV1(bytes32)` and selector `0x9086a2a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "creditDepositV1", abi = "creditDepositV1(bytes32)")]
    pub struct CreditDepositV1Call {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `deletePubkey` function with signature `deletePubkey(uint256)` and selector `0xbbef84b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deletePubkey", abi = "deletePubkey(uint256)")]
    pub struct DeletePubkeyCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `delistProduct` function with signature `delistProduct(uint32[],int128[],bytes32[])` and selector `0x3dabe0d9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "delistProduct",
        abi = "delistProduct(uint32[],int128[],bytes32[])"
    )]
    pub struct DelistProductCall {
        pub product_ids: ::std::vec::Vec<u32>,
        pub prices_x18: ::std::vec::Vec<i128>,
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `directDepositV1Address` function with signature `directDepositV1Address(bytes32)` and selector `0x12efa71b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "directDepositV1Address",
        abi = "directDepositV1Address(bytes32)"
    )]
    pub struct DirectDepositV1AddressCall(pub [u8; 32]);
    ///Container type for all input parameters for the `dumpFees` function with signature `dumpFees()` and selector `0x707c8b58`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dumpFees", abi = "dumpFees()")]
    pub struct DumpFeesCall;
    ///Container type for all input parameters for the `hasPendingAddOrUpdateProductCalls` function with signature `hasPendingAddOrUpdateProductCalls()` and selector `0x145ca380`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "hasPendingAddOrUpdateProductCalls",
        abi = "hasPendingAddOrUpdateProductCalls()"
    )]
    pub struct HasPendingAddOrUpdateProductCallsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address,address,address,address)` and selector `0x8a29e2de`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub multisig: ::ethers::core::types::Address,
        pub deployer: ::ethers::core::types::Address,
        pub spot_engine: ::ethers::core::types::Address,
        pub perp_engine: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub clearinghouse: ::ethers::core::types::Address,
        pub verifier: ::ethers::core::types::Address,
        pub wrapped_native: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isDirectDepositV1Ready` function with signature `isDirectDepositV1Ready(address)` and selector `0x47e2ca90`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "isDirectDepositV1Ready",
        abi = "isDirectDepositV1Ready(address)"
    )]
    pub struct IsDirectDepositV1ReadyCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pendingPerpAddOrUpdateProductIds` function with signature `pendingPerpAddOrUpdateProductIds()` and selector `0xbe05a69d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "pendingPerpAddOrUpdateProductIds",
        abi = "pendingPerpAddOrUpdateProductIds()"
    )]
    pub struct PendingPerpAddOrUpdateProductIdsCall;
    ///Container type for all input parameters for the `pendingSpotAddOrUpdateProductIds` function with signature `pendingSpotAddOrUpdateProductIds()` and selector `0xc9c5efaa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "pendingSpotAddOrUpdateProductIds",
        abi = "pendingSpotAddOrUpdateProductIds()"
    )]
    pub struct PendingSpotAddOrUpdateProductIdsCall;
    ///Container type for all input parameters for the `perpUpdateRisk` function with signature `perpUpdateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `0x3392c585`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "perpUpdateRisk",
        abi = "perpUpdateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct PerpUpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `rebalanceXWithdraw` function with signature `rebalanceXWithdraw(uint32,uint128,address)` and selector `0xebd6c294`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "rebalanceXWithdraw",
        abi = "rebalanceXWithdraw(uint32,uint128,address)"
    )]
    pub struct RebalanceXWithdrawCall {
        pub product_id: u32,
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeWithdrawPoolLiquidity` function with signature `removeWithdrawPoolLiquidity(uint32,uint128,address)` and selector `0x5312b91f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "removeWithdrawPoolLiquidity",
        abi = "removeWithdrawPoolLiquidity(uint32,uint128,address)"
    )]
    pub struct RemoveWithdrawPoolLiquidityCall {
        pub product_id: u32,
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setSpreads` function with signature `setSpreads(uint256)` and selector `0x94912b80`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setSpreads", abi = "setSpreads(uint256)")]
    pub struct SetSpreadsCall {
        pub spreads: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setWithdrawPool` function with signature `setWithdrawPool(address)` and selector `0xc227db96`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setWithdrawPool", abi = "setWithdrawPool(address)")]
    pub struct SetWithdrawPoolCall {
        pub withdraw_pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `spotUpdateRisk` function with signature `spotUpdateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `0x8ab3daae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "spotUpdateRisk",
        abi = "spotUpdateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct SpotUpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `submitPerpAddOrUpdateProductCall` function with signature `submitPerpAddOrUpdateProductCall(uint32,int128,int128,(int32,int32,int32,int32,int128))` and selector `0x6c555b1b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "submitPerpAddOrUpdateProductCall",
        abi = "submitPerpAddOrUpdateProductCall(uint32,int128,int128,(int32,int32,int32,int32,int128))"
    )]
    pub struct SubmitPerpAddOrUpdateProductCallCall {
        pub product_id: u32,
        pub size_increment: i128,
        pub min_size: i128,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `submitSpotAddOrUpdateProductCall` function with signature `submitSpotAddOrUpdateProductCall(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `0xd9fb99c1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "submitSpotAddOrUpdateProductCall",
        abi = "submitSpotAddOrUpdateProductCall(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct SubmitSpotAddOrUpdateProductCallCall {
        pub product_id: u32,
        pub quote_id: u32,
        pub size_increment: i128,
        pub min_size: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateTierFeeRates` function with signature `updateTierFeeRates(uint32[],uint32[],int128[],int128[])` and selector `0x9b6aba8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateTierFeeRates",
        abi = "updateTierFeeRates(uint32[],uint32[],int128[],int128[])"
    )]
    pub struct UpdateTierFeeRatesCall {
        pub tier: ::std::vec::Vec<u32>,
        pub product_id: ::std::vec::Vec<u32>,
        pub maker_rate_x18: ::std::vec::Vec<i128>,
        pub taker_rate_x18: ::std::vec::Vec<i128>,
    }
    ///Container type for all input parameters for the `withdrawInsurance` function with signature `withdrawInsurance(uint128,address)` and selector `0x6c6c654e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawInsurance", abi = "withdrawInsurance(uint128,address)")]
    pub struct WithdrawInsuranceCall {
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ContractOwnerCalls {
        AddEngine(AddEngineCall),
        AddOrUpdateProducts(AddOrUpdateProductsCall),
        AssignPubKey(AssignPubKeyCall),
        ClearPerpAddOrUpdateProductCalls(ClearPerpAddOrUpdateProductCallsCall),
        ClearSpotAddOrUpdateProductCalls(ClearSpotAddOrUpdateProductCallsCall),
        CreateDirectDepositV1(CreateDirectDepositV1Call),
        CreditDepositV1(CreditDepositV1Call),
        DeletePubkey(DeletePubkeyCall),
        DelistProduct(DelistProductCall),
        DirectDepositV1Address(DirectDepositV1AddressCall),
        DumpFees(DumpFeesCall),
        HasPendingAddOrUpdateProductCalls(HasPendingAddOrUpdateProductCallsCall),
        Initialize(InitializeCall),
        IsDirectDepositV1Ready(IsDirectDepositV1ReadyCall),
        Owner(OwnerCall),
        PendingPerpAddOrUpdateProductIds(PendingPerpAddOrUpdateProductIdsCall),
        PendingSpotAddOrUpdateProductIds(PendingSpotAddOrUpdateProductIdsCall),
        PerpUpdateRisk(PerpUpdateRiskCall),
        RebalanceXWithdraw(RebalanceXWithdrawCall),
        RemoveWithdrawPoolLiquidity(RemoveWithdrawPoolLiquidityCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetSpreads(SetSpreadsCall),
        SetWithdrawPool(SetWithdrawPoolCall),
        SpotUpdateRisk(SpotUpdateRiskCall),
        SubmitPerpAddOrUpdateProductCall(SubmitPerpAddOrUpdateProductCallCall),
        SubmitSpotAddOrUpdateProductCall(SubmitSpotAddOrUpdateProductCallCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateTierFeeRates(UpdateTierFeeRatesCall),
        WithdrawInsurance(WithdrawInsuranceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ContractOwnerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddEngineCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddEngine(decoded));
            }
            if let Ok(decoded) =
                <AddOrUpdateProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddOrUpdateProducts(decoded));
            }
            if let Ok(decoded) = <AssignPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignPubKey(decoded));
            }
            if let Ok(decoded) =
                <ClearPerpAddOrUpdateProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ClearPerpAddOrUpdateProductCalls(decoded));
            }
            if let Ok(decoded) =
                <ClearSpotAddOrUpdateProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ClearSpotAddOrUpdateProductCalls(decoded));
            }
            if let Ok(decoded) =
                <CreateDirectDepositV1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateDirectDepositV1(decoded));
            }
            if let Ok(decoded) =
                <CreditDepositV1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreditDepositV1(decoded));
            }
            if let Ok(decoded) = <DeletePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeletePubkey(decoded));
            }
            if let Ok(decoded) = <DelistProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelistProduct(decoded));
            }
            if let Ok(decoded) =
                <DirectDepositV1AddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DirectDepositV1Address(decoded));
            }
            if let Ok(decoded) = <DumpFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DumpFees(decoded));
            }
            if let Ok(decoded) =
                <HasPendingAddOrUpdateProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HasPendingAddOrUpdateProductCalls(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsDirectDepositV1ReadyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsDirectDepositV1Ready(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingPerpAddOrUpdateProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PendingPerpAddOrUpdateProductIds(decoded));
            }
            if let Ok(decoded) =
                <PendingSpotAddOrUpdateProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PendingSpotAddOrUpdateProductIds(decoded));
            }
            if let Ok(decoded) =
                <PerpUpdateRiskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PerpUpdateRisk(decoded));
            }
            if let Ok(decoded) =
                <RebalanceXWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceXWithdraw(decoded));
            }
            if let Ok(decoded) =
                <RemoveWithdrawPoolLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveWithdrawPoolLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetSpreadsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSpreads(decoded));
            }
            if let Ok(decoded) =
                <SetWithdrawPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetWithdrawPool(decoded));
            }
            if let Ok(decoded) =
                <SpotUpdateRiskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SpotUpdateRisk(decoded));
            }
            if let Ok(decoded) =
                <SubmitPerpAddOrUpdateProductCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SubmitPerpAddOrUpdateProductCall(decoded));
            }
            if let Ok(decoded) =
                <SubmitSpotAddOrUpdateProductCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SubmitSpotAddOrUpdateProductCall(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateTierFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateTierFeeRates(decoded));
            }
            if let Ok(decoded) =
                <WithdrawInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawInsurance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ContractOwnerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddEngine(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddOrUpdateProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssignPubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClearPerpAddOrUpdateProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClearSpotAddOrUpdateProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateDirectDepositV1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreditDepositV1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeletePubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelistProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DirectDepositV1Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DumpFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasPendingAddOrUpdateProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDirectDepositV1Ready(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingPerpAddOrUpdateProductIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingSpotAddOrUpdateProductIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PerpUpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceXWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveWithdrawPoolLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSpreads(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetWithdrawPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SpotUpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitPerpAddOrUpdateProductCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitSpotAddOrUpdateProductCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateTierFeeRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawInsurance(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ContractOwnerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddEngine(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOrUpdateProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearPerpAddOrUpdateProductCalls(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClearSpotAddOrUpdateProductCalls(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateDirectDepositV1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditDepositV1(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeletePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelistProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::DirectDepositV1Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasPendingAddOrUpdateProductCalls(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDirectDepositV1Ready(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingPerpAddOrUpdateProductIds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingSpotAddOrUpdateProductIds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PerpUpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceXWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveWithdrawPoolLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSpreads(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotUpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitPerpAddOrUpdateProductCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitSpotAddOrUpdateProductCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateTierFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawInsurance(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddEngineCall> for ContractOwnerCalls {
        fn from(value: AddEngineCall) -> Self {
            Self::AddEngine(value)
        }
    }
    impl ::core::convert::From<AddOrUpdateProductsCall> for ContractOwnerCalls {
        fn from(value: AddOrUpdateProductsCall) -> Self {
            Self::AddOrUpdateProducts(value)
        }
    }
    impl ::core::convert::From<AssignPubKeyCall> for ContractOwnerCalls {
        fn from(value: AssignPubKeyCall) -> Self {
            Self::AssignPubKey(value)
        }
    }
    impl ::core::convert::From<ClearPerpAddOrUpdateProductCallsCall> for ContractOwnerCalls {
        fn from(value: ClearPerpAddOrUpdateProductCallsCall) -> Self {
            Self::ClearPerpAddOrUpdateProductCalls(value)
        }
    }
    impl ::core::convert::From<ClearSpotAddOrUpdateProductCallsCall> for ContractOwnerCalls {
        fn from(value: ClearSpotAddOrUpdateProductCallsCall) -> Self {
            Self::ClearSpotAddOrUpdateProductCalls(value)
        }
    }
    impl ::core::convert::From<CreateDirectDepositV1Call> for ContractOwnerCalls {
        fn from(value: CreateDirectDepositV1Call) -> Self {
            Self::CreateDirectDepositV1(value)
        }
    }
    impl ::core::convert::From<CreditDepositV1Call> for ContractOwnerCalls {
        fn from(value: CreditDepositV1Call) -> Self {
            Self::CreditDepositV1(value)
        }
    }
    impl ::core::convert::From<DeletePubkeyCall> for ContractOwnerCalls {
        fn from(value: DeletePubkeyCall) -> Self {
            Self::DeletePubkey(value)
        }
    }
    impl ::core::convert::From<DelistProductCall> for ContractOwnerCalls {
        fn from(value: DelistProductCall) -> Self {
            Self::DelistProduct(value)
        }
    }
    impl ::core::convert::From<DirectDepositV1AddressCall> for ContractOwnerCalls {
        fn from(value: DirectDepositV1AddressCall) -> Self {
            Self::DirectDepositV1Address(value)
        }
    }
    impl ::core::convert::From<DumpFeesCall> for ContractOwnerCalls {
        fn from(value: DumpFeesCall) -> Self {
            Self::DumpFees(value)
        }
    }
    impl ::core::convert::From<HasPendingAddOrUpdateProductCallsCall> for ContractOwnerCalls {
        fn from(value: HasPendingAddOrUpdateProductCallsCall) -> Self {
            Self::HasPendingAddOrUpdateProductCalls(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ContractOwnerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsDirectDepositV1ReadyCall> for ContractOwnerCalls {
        fn from(value: IsDirectDepositV1ReadyCall) -> Self {
            Self::IsDirectDepositV1Ready(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ContractOwnerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingPerpAddOrUpdateProductIdsCall> for ContractOwnerCalls {
        fn from(value: PendingPerpAddOrUpdateProductIdsCall) -> Self {
            Self::PendingPerpAddOrUpdateProductIds(value)
        }
    }
    impl ::core::convert::From<PendingSpotAddOrUpdateProductIdsCall> for ContractOwnerCalls {
        fn from(value: PendingSpotAddOrUpdateProductIdsCall) -> Self {
            Self::PendingSpotAddOrUpdateProductIds(value)
        }
    }
    impl ::core::convert::From<PerpUpdateRiskCall> for ContractOwnerCalls {
        fn from(value: PerpUpdateRiskCall) -> Self {
            Self::PerpUpdateRisk(value)
        }
    }
    impl ::core::convert::From<RebalanceXWithdrawCall> for ContractOwnerCalls {
        fn from(value: RebalanceXWithdrawCall) -> Self {
            Self::RebalanceXWithdraw(value)
        }
    }
    impl ::core::convert::From<RemoveWithdrawPoolLiquidityCall> for ContractOwnerCalls {
        fn from(value: RemoveWithdrawPoolLiquidityCall) -> Self {
            Self::RemoveWithdrawPoolLiquidity(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ContractOwnerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetSpreadsCall> for ContractOwnerCalls {
        fn from(value: SetSpreadsCall) -> Self {
            Self::SetSpreads(value)
        }
    }
    impl ::core::convert::From<SetWithdrawPoolCall> for ContractOwnerCalls {
        fn from(value: SetWithdrawPoolCall) -> Self {
            Self::SetWithdrawPool(value)
        }
    }
    impl ::core::convert::From<SpotUpdateRiskCall> for ContractOwnerCalls {
        fn from(value: SpotUpdateRiskCall) -> Self {
            Self::SpotUpdateRisk(value)
        }
    }
    impl ::core::convert::From<SubmitPerpAddOrUpdateProductCallCall> for ContractOwnerCalls {
        fn from(value: SubmitPerpAddOrUpdateProductCallCall) -> Self {
            Self::SubmitPerpAddOrUpdateProductCall(value)
        }
    }
    impl ::core::convert::From<SubmitSpotAddOrUpdateProductCallCall> for ContractOwnerCalls {
        fn from(value: SubmitSpotAddOrUpdateProductCallCall) -> Self {
            Self::SubmitSpotAddOrUpdateProductCall(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ContractOwnerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateTierFeeRatesCall> for ContractOwnerCalls {
        fn from(value: UpdateTierFeeRatesCall) -> Self {
            Self::UpdateTierFeeRates(value)
        }
    }
    impl ::core::convert::From<WithdrawInsuranceCall> for ContractOwnerCalls {
        fn from(value: WithdrawInsuranceCall) -> Self {
            Self::WithdrawInsurance(value)
        }
    }
    ///Container type for all return fields from the `createDirectDepositV1` function with signature `createDirectDepositV1(bytes32)` and selector `0x6e13cbf3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreateDirectDepositV1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `directDepositV1Address` function with signature `directDepositV1Address(bytes32)` and selector `0x12efa71b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DirectDepositV1AddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasPendingAddOrUpdateProductCalls` function with signature `hasPendingAddOrUpdateProductCalls()` and selector `0x145ca380`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HasPendingAddOrUpdateProductCallsReturn(pub bool);
    ///Container type for all return fields from the `isDirectDepositV1Ready` function with signature `isDirectDepositV1Ready(address)` and selector `0x47e2ca90`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsDirectDepositV1ReadyReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingPerpAddOrUpdateProductIds` function with signature `pendingPerpAddOrUpdateProductIds()` and selector `0xbe05a69d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PendingPerpAddOrUpdateProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `pendingSpotAddOrUpdateProductIds` function with signature `pendingSpotAddOrUpdateProductIds()` and selector `0xc9c5efaa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PendingSpotAddOrUpdateProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///`Config(address,int128,int128,int128,int128,int128,int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Config {
        pub token: ::ethers::core::types::Address,
        pub interest_inflection_util_x18: i128,
        pub interest_floor_x18: i128,
        pub interest_small_cap_x18: i128,
        pub interest_large_cap_x18: i128,
        pub withdraw_fee_x18: i128,
        pub min_deposit_rate_x18: i128,
    }
    ///`RiskStore(int32,int32,int32,int32,int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RiskStore {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        pub price_x18: i128,
    }
}
