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
                    ::std::borrow::ToOwned::to_owned("addProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addProducts"),
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
                    ::std::borrow::ToOwned::to_owned("batchSubmitUpdateProductTxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("batchSubmitUpdateProductTxs",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slowModeTxs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearPerpAddProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearPerpAddProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearSpotAddProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearSpotAddProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearUpdateProductTxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearUpdateProductTxs",),
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
                    ::std::borrow::ToOwned::to_owned("hasPendingAddProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasPendingAddProductCalls",),
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
                    ::std::borrow::ToOwned::to_owned("hasPendingUpdateProductTxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasPendingUpdateProductTxs",),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("pendingPerpAddProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingPerpAddProductIds",),
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
                    ::std::borrow::ToOwned::to_owned("pendingSpotAddProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingSpotAddProductIds",),
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
                    ::std::borrow::ToOwned::to_owned("submitPerpAddProductCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitPerpAddProductCall",),
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
                    ::std::borrow::ToOwned::to_owned("submitSpotAddProductCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitSpotAddProductCall",),
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
                    ::std::borrow::ToOwned::to_owned("submitUpdateProductTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitUpdateProductTx",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slowModeTx"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("updateProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateProducts"),
                        inputs: ::std::vec![],
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\x1Cb\0\0\"V[b\0\0\xE4V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\0\xE2W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[aJK\x80b\0\0\xF4`\09`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x026W`\x005`\xE0\x1C\x80c\x8A)\xE2\xDE\x11b\0\x01BW\x80c\xB4\x96\xFB\xB4\x11b\0\0\xCCW\x80c\xD4\xA5\xEA\x85\x11b\0\0\x97W\x80c\xEB\xD6\xC2\x94\x11b\0\0zW\x80c\xEB\xD6\xC2\x94\x14b\0\x04\xC7W\x80c\xF2\xFD\xE3\x8B\x14b\0\x04\xDEW\x80c\xFB\x1C\xB0I\x14b\0\x04\xF5W`\0\x80\xFD[\x80c\xD4\xA5\xEA\x85\x14b\0\x04\xA6W\x80c\xE7{\xDCn\x14b\0\x04\xBDW`\0\x80\xFD[\x80c\xB4\x96\xFB\xB4\x14b\0\x04WW\x80c\xBB\xEF\x84\xB4\x14b\0\x04aW\x80c\xBE\x13\xBA\xC4\x14b\0\x04xW\x80c\xC2'\xDB\x96\x14b\0\x04\x8FW`\0\x80\xFD[\x80c\x8D\xB4\x8E\xCD\x11b\0\x01\rW\x80c\x8D\xB4\x8E\xCD\x14b\0\x04\x08W\x80c\x90]\x8AE\x14b\0\x04\x1FW\x80c\x90\x86\xA2\xA5\x14b\0\x04)W\x80c\x9Bj\xBA\x8F\x14b\0\x04@W`\0\x80\xFD[\x80c\x8A)\xE2\xDE\x14b\0\x03\xB1W\x80c\x8A\xB3\xDA\xAE\x14b\0\x03\xC8W\x80c\x8C\xA4g%\x14b\0\x03\xDFW\x80c\x8D\xA5\xCB[\x14b\0\x03\xF6W`\0\x80\xFD[\x80cS\x12\xB9\x1F\x11b\0\x01\xC4W\x80cp|\x8BX\x11b\0\x01\x8FW\x80cp|\x8BX\x14b\0\x03|W\x80cqP\x18\xA6\x14b\0\x03\x86W\x80cx\x8DBH\x14b\0\x03\x90W\x80c{gLz\x14b\0\x03\x9AW`\0\x80\xFD[\x80cS\x12\xB9\x1F\x14b\0\x03 W\x80cV\xE4\x9E\xF3\x14b\0\x037W\x80clleN\x14b\0\x03NW\x80cn\x13\xCB\xF3\x14b\0\x03eW`\0\x80\xFD[\x80c=\x14\x1D8\x11b\0\x02\x05W\x80c=\x14\x1D8\x14b\0\x02\xCFW\x80c=\xAB\xE0\xD9\x14b\0\x02\xD9W\x80cG\xE2\xCA\x90\x14b\0\x02\xF0W\x80cP\xE8'\x90\x14b\0\x03\x07W`\0\x80\xFD[\x80c\x05wU\x9F\x14b\0\x02;W\x80c\x05\x97Iv\x14b\0\x02ZW\x80c\x12\xEF\xA7\x1B\x14b\0\x02sW\x80c3\x92\xC5\x85\x14b\0\x02\xB8W[`\0\x80\xFD[b\0\x02Eb\0\x05\0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\0\x02qb\0\x02k6`\x04b\0))V[b\0\x05\x1AV[\0[b\0\x02\x9Fb\0\x02\x846`\x04b\0)\x8EV[`\xA3` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01b\0\x02QV[b\0\x02qb\0\x02\xC96`\x04b\0+\x02V[b\0\x06\xBBV[b\0\x02qb\0\x07)V[b\0\x02qb\0\x02\xEA6`\x04b\0+\x8CV[b\0\x08\x83V[b\0\x02Eb\0\x03\x016`\x04b\0,FV[b\0\n\xA5V[b\0\x03\x11b\0\x0EJV[`@Qb\0\x02Q\x91\x90b\0,mV[b\0\x02qb\0\x0316`\x04b\0,\xDAV[b\0\x0F\xBEV[b\0\x02qb\0\x03H6`\x04b\0-*V[b\0\x10\xC5V[b\0\x02qb\0\x03_6`\x04b\0-tV[b\0\x115V[b\0\x02\x9Fb\0\x03v6`\x04b\0)\x8EV[b\0\x11\xE7V[b\0\x02qb\0\x12\xA1V[b\0\x02qb\0\x131V[b\0\x03\x11b\0\x13GV[b\0\x02qb\0\x03\xAB6`\x04b\0.UV[b\0\x14\xB5V[b\0\x02qb\0\x03\xC26`\x04b\0.\xC0V[b\0\x18\xB9V[b\0\x02qb\0\x03\xD96`\x04b\0+\x02V[b\0\x1A\xCAV[b\0\x02qb\0\x03\xF06`\x04b\0/zV[b\0\x1B\x08V[`gT`\x01`\x01`\xA0\x1B\x03\x16b\0\x02\x9FV[b\0\x02qb\0\x04\x196`\x04b\0/\xC0V[b\0\x1C,V[b\0\x02qb\0\x1D\xE3V[b\0\x02qb\0\x04:6`\x04b\0)\x8EV[b\0\x1EGV[b\0\x02qb\0\x04Q6`\x04b\x000\xC4V[b\0\x1E\xAFV[b\0\x02qb\0!*V[b\0\x02qb\0\x04r6`\x04b\0)\x8EV[b\0!\x8EV[b\0\x02qb\0\x04\x896`\x04b\x001~V[b\0!\xCAV[b\0\x02qb\0\x04\xA06`\x04b\0,FV[b\0\"\x14V[b\0\x02qb\0\x04\xB76`\x04b\x001\xABV[b\0\"QV[b\0\x02qb\0\"\xE6V[b\0\x02qb\0\x04\xD86`\x04b\0,\xDAV[b\0#JV[b\0\x02qb\0\x04\xEF6`\x04b\0,FV[b\0$hV[`\xA0T\x15\x15b\0\x02EV[`\xA2T`\0\x90\x15\x15\x80b\0\x05\x15WP`\xA1T\x15\x15[\x90P\x90V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0b\0\x05\x80b\0\x13GV[\x90P`\0[\x81Q\x81\x10\x15b\0\x06)W\x81\x81\x81Q\x81\x10b\0\x05\xA4Wb\0\x05\xA4b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x03b\0\x06\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Ftrying to add a perp product twi`D\x82\x01Rb1\xB2\x97`\xE9\x1B`d\x82\x01R`\x84\x01b\0\x05kV[\x80b\0\x06 \x81b\x002NV[\x91PPb\0\x05\x85V[P`\xA2`@Q\x80`\x80\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85`\x0F\x0B\x81R` \x01\x84\x806\x03\x81\x01\x90b\0\x06h\x91\x90b\x002jV[\x90R`@Qb\0\x06|\x91\x90` \x01b\x002\x89V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x06\xB3\x94\x91\x90\x92\x01\x92\x01\x90b\0'0V[PPPPPPV[b\0\x06\xC5b\0$\xFEV[`\x9BT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x06\xF9\x90\x85\x90\x85\x90`\x04\x01b\x003\nV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x07\x14W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\xB3W=`\0\x80>=`\0\xFD[b\0\x073b\0$\xFEV[`\0[`\xA0T\x81\x10\x15b\0\x08rW`\0`\xA0\x82\x81T\x81\x10b\0\x07YWb\0\x07Yb\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x07p\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\x9E\x90b\x003_V[\x80\x15b\0\x07\xEFW\x80`\x1F\x10b\0\x07\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x07\xEFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x07\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PP`\x9CT`@Qcs\x02v\xCF`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x16\x93c\xE6\x04\xED\x9E\x93Pb\0\x08'\x92P\x85\x91P`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x08BW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x08WW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x08i\x90b\x002NV[\x91PPb\0\x076V[Pb\0\x08\x81`\xA0`\0b\0'\xBFV[V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x08\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[\x84\x83\x14b\0\t\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0[\x85\x81\x10\x15b\0\n\x9CW`\0`@Q\x80``\x01`@R\x80\x89\x89\x85\x81\x81\x10b\0\tIWb\0\tIb\x002\"V[\x90P` \x02\x01` \x81\x01\x90b\0\t`\x91\x90b\x004\x0BV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x87\x85\x81\x81\x10b\0\t\x80Wb\0\t\x80b\x002\"V[\x90P` \x02\x01` \x81\x01\x90b\0\t\x97\x91\x90b\x004+V[`\x0F\x0B\x81R` \x01\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91P`\x14\x90P\x82`@Q` \x01b\0\t\xEF\x91\x90b\x004KV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\n\x0F\x92\x91` \x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\nP\x90\x84\x90`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\nkW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\n\x80W=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\n\x93\x90b\x002NV[\x91PPb\0\t\x1EV[PPPPPPPV[`\0\x80`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\n\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x0B&\x91\x90\x81\x01\x90b\x004\xE2V[\x90P`\0[\x81Q\x81\x10\x15b\0\x0E@W`\0\x82\x82\x81Q\x81\x10b\0\x0BLWb\0\x0BLb\x002\"V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0B\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0B\xD2\x91\x90b\x005\x87V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x0C,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x05kV[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x82\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0CxW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0C\x9E\x91\x90b\x005\xA7V[`\x9FT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x84\x16\x03b\0\x0C\xD1Wb\0\x0C\xCE`\x01`\x01`\xA0\x1B\x03\x89\x161\x82b\x005\xC1V[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r6\x91\x90b\x005\xDCV[b\0\rC\x90`\x12b\x006\x01V[b\0\rP\x90`\nb\x007$V[b\0\r\\\x90\x82b\x0075V[`\x9AT`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r\xD7\x91\x90b\x007dV[`\x80\x01Q\x90Pb\0\r\xF2g\r\xE0\xB6\xB3\xA7d\0\0`\x05b\08\nV[`\x0F\x0Bb\0\x0E\r\x83\x83`\x0F\x0Bb\0%Z\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x12b\0\x0E%WP`\x01\x98\x97PPPPPPPPV[PPPPP\x80\x80b\0\x0E7\x90b\x002NV[\x91PPb\0\x0B+V[P`\0\x93\x92PPPV[`\xA1T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0EnWb\0\x0Enb\0)\xA8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x0E\x98W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA1T\x81\x10\x15b\0\x0F\xB8W`\0`\xA1\x82\x81T\x81\x10b\0\x0E\xC1Wb\0\x0E\xC1b\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x0E\xD8\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x0F\x06\x90b\x003_V[\x80\x15b\0\x0FWW\x80`\x1F\x10b\0\x0F+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x0FWV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x0F9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x0Fq\x91\x90b\09:V[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x0F\x8DWb\0\x0F\x8Db\x002\"V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x0F\xAF\x81b\x002NV[\x91PPb\0\x0E\x9EV[P\x91\x90PV[b\0\x0F\xC8b\0$\xFEV[`\x9DT`@\x80Qc\x1Ft\xAC\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFB\xA5`\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x10\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x109\x91\x90b\x005\x87V[`@Qc\xFD\x8CR\xCD`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x92P\x90\x82\x16\x90c\xFD\x8CR\xCD\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x10\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x10\xBBW=`\0\x80>=`\0\xFD[PPPPPPPPV[b\0\x10\xCFb\0$\xFEV[`\x9DT`@QcV\xE4\x9E\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cV\xE4\x9E\xF3\x90b\0\x11\x05\x90\x86\x90\x86\x90\x86\x90`\x04\x01b\0:vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x11 W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\n\x9CW=`\0\x80>=`\0\xFD[b\0\x11?b\0$\xFEV[`@\x80Q\x80\x82\x01\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x81\x84\x01R\x82Q\x80\x82\x03\x84\x01\x81R``\x82\x01\x90\x93R\x90\x91`\0\x91b\0\x11\xA6\x91`\x12\x91`\x80\x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x10\x8B\x90\x84\x90`\x04\x01b\x003\xF6V[`\x9CT`\x9AT`\x9FT`@Q`\0\x93\x84\x93`\x01\x93`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93\x91\x83\x16\x92\x88\x92\x91\x16\x90b\0\x12\x1B\x90b\0'\xDFV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R`@\x83\x01\x91\x90\x91R\x90\x91\x16``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x12dW=`\0\x80>=`\0\xFD[P`\0\x93\x84R`\xA3` R`@\x90\x93 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UP\x90\x91\x90PV[b\0\x12\xABb\0$\xFEV[`@\x80Q`\t`\xF8\x1B` \x82\x01R\x81Q`\x01\x81\x83\x03\x01\x81R`!\x82\x01\x92\x83\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x90\x93R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x12\xFA\x90\x84\x90`%\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x13\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x13*W=`\0\x80>=`\0\xFD[PPPPPV[b\0\x13;b\0$\xFEV[b\0\x08\x81`\0b\0%\xE8V[`\xA2T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x13kWb\0\x13kb\0)\xA8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x13\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA2T\x81\x10\x15b\0\x0F\xB8W`\0`\xA2\x82\x81T\x81\x10b\0\x13\xBEWb\0\x13\xBEb\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x13\xD5\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x14\x03\x90b\x003_V[\x80\x15b\0\x14TW\x80`\x1F\x10b\0\x14(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x14TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x146W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x14n\x91\x90b\0:\xBBV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x14\x8AWb\0\x14\x8Ab\x002\"V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x14\xAC\x81b\x002NV[\x91PPb\0\x13\x9BV[b\0\x14\xBFb\0$\xFEV[`\0[`\xA1T\x81\x10\x15b\0\x16\xB1W`\0`\xA1\x82\x81T\x81\x10b\0\x14\xE5Wb\0\x14\xE5b\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x14\xFC\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x15*\x90b\x003_V[\x80\x15b\0\x15{W\x80`\x1F\x10b\0\x15OWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x15{V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x15]W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x15\x95\x91\x90b\09:V[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10b\0\x15\xB7Wb\0\x15\xB7b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x16\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fspot id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x05kV[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Qc%\xBC\xE8\xC9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96c%\xBC\xE8\xC9\x96b\0\x16f\x96\x90\x95\x90\x94\x93\x92\x91`\x04\x01b\0;LV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x16\x81W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x16\x96W=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x16\xA8\x90b\x002NV[\x91PPb\0\x14\xC2V[Pb\0\x16\xC0`\xA1`\0b\0'\xBFV[`\0[`\xA2T\x81\x10\x15b\0\x18\xA6W`\0`\xA2\x82\x81T\x81\x10b\0\x16\xE6Wb\0\x16\xE6b\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x16\xFD\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x17+\x90b\x003_V[\x80\x15b\0\x17|W\x80`\x1F\x10b\0\x17PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x17|V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x17^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x17\x96\x91\x90b\0:\xBBV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10b\0\x17\xB8Wb\0\x17\xB8b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x18\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fperp id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x05kV[`\x9BT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xD7\xAA\xAC\xFF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x94c\xD7\xAA\xAC\xFF\x94b\0\x18[\x94\x90\x93\x90\x92\x91`\x04\x01b\0<#V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x18vW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x18\x8BW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x18\x9D\x90b\x002NV[\x91PPb\0\x16\xC3V[Pb\0\x18\xB5`\xA2`\0b\0'\xBFV[PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x18\xDAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x18\xF6WP0;\x15\x80\x15b\0\x18\xF6WP`\0T`\xFF\x16`\x01\x14[b\0\x19jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x05kV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x19\x8EW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x88\x163\x14b\0\x19\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fexpected deployed to initialize\0`D\x82\x01R`d\x01b\0\x05kV[b\0\x19\xF2b\0&GV[b\0\x19\xFD\x89b\0$hV[`\x99\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x17\x90\x92U`\x9A\x80T\x82\x16\x8A\x84\x16\x17\x90U`\x9B\x80T\x82\x16\x89\x84\x16\x17\x90U`\x9C\x80T\x82\x16\x88\x84\x16\x17\x90U`\x9D\x80T\x82\x16\x87\x84\x16\x17\x90U`\x9E\x80T\x82\x16\x86\x84\x16\x17\x90U`\x9F\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15b\0\x1A\xBFW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[b\0\x1A\xD4b\0$\xFEV[`\x9AT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x06\xF9\x90\x85\x90\x85\x90`\x04\x01b\x003\nV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1B^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0[\x81\x81\x10\x15b\0\x1C'W`\0\x83\x83\x83\x81\x81\x10b\0\x1B\x81Wb\0\x1B\x81b\x002\"V[\x90P` \x02\x81\x01\x90b\0\x1B\x95\x91\x90b\0<\x94V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP`\xA0\x80T`\x01\x81\x01\x82U\x91R\x84Q\x94\x95Pb\0\x1C\x0F\x94\x7Fx\xFD\xC8\xD4\"\xC4\x9C\xED\x03Z\x9E\xDF\x18\xD0\r<j\x8D\x81\xDF!\x0F>^D\x8E\x04^w\xB4\x1E\x88\x90\x91\x01\x93P` \x86\x01\x92P\x90Pb\0'0V[PP\x80\x80b\0\x1C\x1E\x90b\x002NV[\x91PPb\0\x1BaV[PPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1C\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0b\0\x1C\x8Eb\0\x0EJV[\x90P`\0[\x81Q\x81\x10\x15b\0\x1D7W\x81\x81\x81Q\x81\x10b\0\x1C\xB2Wb\0\x1C\xB2b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x88c\xFF\xFF\xFF\xFF\x16\x03b\0\x1D\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Ftrying to add a spot product twi`D\x82\x01Rb1\xB2\x97`\xE9\x1B`d\x82\x01R`\x84\x01b\0\x05kV[\x80b\0\x1D.\x81b\x002NV[\x91PPb\0\x1C\x93V[P`\xA1`@Q\x80`\xC0\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\x0F\x0B\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85\x806\x03\x81\x01\x90b\0\x1D\x82\x91\x90b\0<\xDEV[\x81R` \x01b\0\x1D\x986\x86\x90\x03\x86\x01\x86b\x002jV[\x90R`@Qb\0\x1D\xAC\x91\x90` \x01b\0=\xA8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x10\xBB\x94\x91\x90\x92\x01\x92\x01\x90b\0'0V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1E9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[b\0\x08\x81`\xA2`\0b\0'\xBFV[`\0\x81\x81R`\xA3` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x1EsWb\0\x1Ep\x82b\0\x11\xE7V[\x90P[\x80`\x01`\x01`\xA0\x1B\x03\x16c&\x08o\x07`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x07\x14W`\0\x80\xFD[b\0\x1E\xB9b\0$\xFEV[\x82Q\x84Q\x14b\0\x1E\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[\x81Q\x84Q\x14b\0\x1FAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[\x80Q\x84Q\x14b\0\x1F\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0[\x84Q\x81\x10\x15b\0\x13*W`\0`@Q\x80`\x80\x01`@R\x80\x87\x84\x81Q\x81\x10b\0\x1F\xB4Wb\0\x1F\xB4b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x84\x81Q\x81\x10b\0\x1F\xDCWb\0\x1F\xDCb\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85\x84\x81Q\x81\x10b\0 \x04Wb\0 \x04b\x002\"V[` \x02` \x01\x01Q`\x0F\x0B\x81R` \x01\x84\x84\x81Q\x81\x10b\0 )Wb\0 )b\x002\"V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0F\x0B\x90R\x90P`\0`\x18`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x86\x01Q\x16\x81\x83\x01R\x90\x84\x01Q`\x0F\x90\x81\x0B``\x80\x84\x01\x91\x90\x91R\x85\x01Q\x90\x0B`\x80\x82\x01R`\xA0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0 \x9D\x92\x91` \x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0 \xDE\x90\x84\x90`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0 \xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0!\x0EW=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0!!\x90b\x002NV[\x91PPb\0\x1F\x88V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0!\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[b\0\x08\x81`\xA0`\0b\0'\xBFV[b\0!\x98b\0$\xFEV[`\x9ET`@Qc.\xFB\xE1-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\xEF\x84\xB4\x90`$\x01b\0\x12\xFAV[b\0!\xD4b\0$\xFEV[`\x9ET`@Qc/\x84\xEE\xB1`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\x13\xBA\xC4\x90`d\x01b\0\x11\x05V[b\0\"\x1Eb\0$\xFEV[`\x9DT`@Qca\x13\xED\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC2'\xDB\x96\x90`$\x01b\0\x12\xFAV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\"\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[`\xA0\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91Rb\0\x1C'\x90\x7Fx\xFD\xC8\xD4\"\xC4\x9C\xED\x03Z\x9E\xDF\x18\xD0\r<j\x8D\x81\xDF!\x0F>^D\x8E\x04^w\xB4\x1E\x88\x01\x83\x83b\0'\xEDV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0#<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[b\0\x08\x81`\xA1`\0b\0'\xBFV[b\0#Tb\0$\xFEV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x85\x16\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x10`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0#\xF7\x92\x91` \x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0$8\x90\x84\x90`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0$SW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1A\xBFW=`\0\x80>=`\0\xFD[b\0$rb\0$\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0$\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x05kV[b\0$\xFB\x81b\0%\xE8V[PV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x08\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x05kV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90b\0%\xA2WPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90b\0%\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05k\x91\x90b\x003\xF6V[P\x90P[\x92\x91PPV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0&\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x05kV[b\0\x08\x81`\0Ta\x01\0\x90\x04`\xFF\x16b\0'%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x05kV[b\0\x08\x813b\0%\xE8V[\x82\x80Tb\0'>\x90b\x003_V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0'bW`\0\x85Ub\0'\xADV[\x82`\x1F\x10b\0'}W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0'\xADV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0'\xADW\x91\x82\x01[\x82\x81\x11\x15b\0'\xADW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0'\x90V[Pb\0'\xBB\x92\x91Pb\0(jV[P\x90V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0$\xFB\x91\x90b\0(\x81V[a\x0Bx\x80b\0>\x9E\x839\x01\x90V[\x82\x80Tb\0'\xFB\x90b\x003_V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0(\x1FW`\0\x85Ub\0'\xADV[\x82`\x1F\x10b\0(:W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ub\0'\xADV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0'\xADW\x91\x82\x01[\x82\x81\x11\x15b\0'\xADW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0(MV[[\x80\x82\x11\x15b\0'\xBBW`\0\x81U`\x01\x01b\0(kV[\x80\x82\x11\x15b\0'\xBBW`\0b\0(\x98\x82\x82b\0(\xA2V[P`\x01\x01b\0(\x81V[P\x80Tb\0(\xB0\x90b\x003_V[`\0\x82U\x80`\x1F\x10b\0(\xC1WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0$\xFB\x91\x90b\0(jV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0$\xFBW`\0\x80\xFD[\x80`\x0F\x0B\x81\x14b\0$\xFBW`\0\x80\xFD[\x805b\0)\x11\x81b\0(\xF4V[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15b\0\x0F\xB8W`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15b\0)AW`\0\x80\xFD[\x845b\0)N\x81b\0(\xE1V[\x93P` \x85\x015b\0)`\x81b\0(\xF4V[\x92P`@\x85\x015b\0)r\x81b\0(\xF4V[\x91Pb\0)\x83\x86``\x87\x01b\0)\x16V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15b\0)\xA1W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0)\xE4Wb\0)\xE4b\0)\xA8V[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0)\xE4Wb\0)\xE4b\0)\xA8V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0)\xE4Wb\0)\xE4b\0)\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*bWb\0*bb\0)\xA8V[`@R\x91\x90PV[\x80`\x03\x0B\x81\x14b\0$\xFBW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15b\0*\x8DW`\0\x80\xFD[b\0*\x97b\0)\xBEV[\x90P\x815b\0*\xA6\x81b\0*jV[\x81R` \x82\x015b\0*\xB8\x81b\0*jV[` \x82\x01R`@\x82\x015b\0*\xCD\x81b\0*jV[`@\x82\x01R``\x82\x015b\0*\xE2\x81b\0*jV[``\x82\x01R`\x80\x82\x015b\0*\xF7\x81b\0(\xF4V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15b\0+\x16W`\0\x80\xFD[\x825b\0+#\x81b\0(\xE1V[\x91Pb\0+4\x84` \x85\x01b\0*zV[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0+PW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0+iW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0+\x85W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15b\0+\xA6W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0+\xBFW`\0\x80\xFD[b\0+\xCD\x8A\x83\x8B\x01b\0+=V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15b\0+\xE7W`\0\x80\xFD[b\0+\xF5\x8A\x83\x8B\x01b\0+=V[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15b\0,\x0FW`\0\x80\xFD[Pb\0,\x1E\x89\x82\x8A\x01b\0+=V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0$\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15b\0,YW`\0\x80\xFD[\x815b\0,f\x81b\0,0V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0,\xADW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0,\x89V[P\x90\x96\x95PPPPPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0)\x11W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0,\xF0W`\0\x80\xFD[\x835b\0,\xFD\x81b\0(\xE1V[\x92Pb\0-\r` \x85\x01b\0,\xB9V[\x91P`@\x84\x015b\0-\x1F\x81b\0,0V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0-@W`\0\x80\xFD[\x835b\0-M\x81b\0,0V[\x92P` \x84\x015b\0-_\x81b\0,0V[\x91P`@\x84\x015`\x02\x81\x10b\0-\x1FW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0-\x88W`\0\x80\xFD[b\0-\x93\x83b\0,\xB9V[\x91P` \x83\x015b\0-\xA5\x81b\0,0V[\x80\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0-\xCDWb\0-\xCDb\0)\xA8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0-\xE9W`\0\x80\xFD[\x815` b\0.\x02b\0-\xFC\x83b\0-\xB0V[b\0*6V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0.\"W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0.JW\x805b\0.<\x81b\0(\xE1V[\x83R\x91\x83\x01\x91\x83\x01b\0.&V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0.iW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0.\x82W`\0\x80\xFD[b\0.\x90\x86\x83\x87\x01b\0-\xD7V[\x93P` \x85\x015\x91P\x80\x82\x11\x15b\0.\xA7W`\0\x80\xFD[Pb\0.\xB6\x85\x82\x86\x01b\0-\xD7V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15b\0.\xDEW`\0\x80\xFD[\x885b\0.\xEB\x81b\0,0V[\x97P` \x89\x015b\0.\xFD\x81b\0,0V[\x96P`@\x89\x015b\0/\x0F\x81b\0,0V[\x95P``\x89\x015b\0/!\x81b\0,0V[\x94P`\x80\x89\x015b\0/3\x81b\0,0V[\x93P`\xA0\x89\x015b\0/E\x81b\0,0V[\x92P`\xC0\x89\x015b\0/W\x81b\0,0V[\x91P`\xE0\x89\x015b\0/i\x81b\0,0V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80` \x83\x85\x03\x12\x15b\0/\x8EW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0/\xA6W`\0\x80\xFD[b\0/\xB4\x85\x82\x86\x01b\0+=V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15b\0/\xDCW`\0\x80\xFD[\x875b\0/\xE9\x81b\0(\xE1V[\x96P` \x88\x015b\0/\xFB\x81b\0(\xE1V[\x95P`@\x88\x015b\x000\r\x81b\0(\xF4V[\x94P``\x88\x015b\x000\x1F\x81b\0(\xF4V[\x93P`\xE0`\x7F\x19\x82\x01\x12\x15b\x0004W`\0\x80\xFD[P`\x80\x87\x01\x91Pb\x000K\x88a\x01`\x89\x01b\0)\x16V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x82`\x1F\x83\x01\x12b\x000iW`\0\x80\xFD[\x815` b\x000|b\0-\xFC\x83b\0-\xB0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\x000\x9CW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0.JW\x805b\x000\xB6\x81b\0(\xF4V[\x83R\x91\x83\x01\x91\x83\x01b\x000\xA0V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\x000\xDBW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\x000\xF4W`\0\x80\xFD[b\x001\x02\x88\x83\x89\x01b\0-\xD7V[\x95P` \x87\x015\x91P\x80\x82\x11\x15b\x001\x19W`\0\x80\xFD[b\x001'\x88\x83\x89\x01b\0-\xD7V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\x001>W`\0\x80\xFD[b\x001L\x88\x83\x89\x01b\x000WV[\x93P``\x87\x015\x91P\x80\x82\x11\x15b\x001cW`\0\x80\xFD[Pb\x001r\x87\x82\x88\x01b\x000WV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\x001\x94W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15b\x001\xBFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\x001\xD8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\x001\xEDW`\0\x80\xFD[\x815\x81\x81\x11\x15b\x001\xFDW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15b\x002\x10W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\x002cWb\x002cb\x0028V[P`\x01\x01\x90V[`\0`\xA0\x82\x84\x03\x12\x15b\x002}W`\0\x80\xFD[b\0,f\x83\x83b\0*zV[`\0a\x01\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qb\x003\x03``\x84\x01\x82\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P\x92\x91PPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`\xC0\x81\x01b\0,f` \x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\x003tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x0F\xB8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\x003\xB2W\x81\x81\x01Q\x83\x82\x01R` \x01b\x003\x98V[\x83\x81\x11\x15b\x003\xC2W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\x003\xE2\x81` \x86\x01` \x86\x01b\x003\x95V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0,f` \x83\x01\x84b\x003\xC8V[`\0` \x82\x84\x03\x12\x15b\x004\x1EW`\0\x80\xFD[\x815b\0,f\x81b\0(\xE1V[`\0` \x82\x84\x03\x12\x15b\x004>W`\0\x80\xFD[\x815b\0,f\x81b\0(\xF4V[`\0` \x80\x83R`\x80\x83\x01c\xFF\xFF\xFF\xFF\x85Q\x16\x82\x85\x01R\x81\x85\x01Q`\x0F\x0B`@\x85\x01R`@\x85\x01Q``\x80\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P\x84\x83\x01\x93P`\0\x92P[\x80\x83\x10\x15b\0.JW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90b\x004\x8FV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qb\x004\xD4\x81`\x01\x85\x01` \x87\x01b\x003\x95V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15b\x004\xF6W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\x005\x0EW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13b\x005 W`\0\x80\xFD[\x80Qb\x0051b\0-\xFC\x82b\0-\xB0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15b\x005QW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15b\x005|W\x83Qb\x005l\x81b\0(\xE1V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90b\x005VV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\x005\x9AW`\0\x80\xFD[\x81Qb\0,f\x81b\0,0V[`\0` \x82\x84\x03\x12\x15b\x005\xBAW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15b\x005\xD7Wb\x005\xD7b\x0028V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\x005\xEFW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0,fW`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\x006\x1EWb\x006\x1Eb\x0028V[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15b\x006hW\x81`\0\x19\x04\x82\x11\x15b\x006LWb\x006Lb\x0028V[\x80\x85\x16\x15b\x006ZW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\x006,V[P\x92P\x92\x90PV[`\0\x82b\x006\x81WP`\x01b\0%\xE2V[\x81b\x006\x90WP`\0b\0%\xE2V[\x81`\x01\x81\x14b\x006\xA9W`\x02\x81\x14b\x006\xB4Wb\x006\xD4V[`\x01\x91PPb\0%\xE2V[`\xFF\x84\x11\x15b\x006\xC8Wb\x006\xC8b\x0028V[PP`\x01\x82\x1Bb\0%\xE2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\x006\xF9WP\x81\x81\nb\0%\xE2V[b\x007\x05\x83\x83b\x006'V[\x80`\0\x19\x04\x82\x11\x15b\x007\x1CWb\x007\x1Cb\x0028V[\x02\x93\x92PPPV[`\0b\0,f`\xFF\x84\x16\x83b\x006pV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\x007RWb\x007Rb\x0028V[P\x02\x90V[\x80Qb\0)\x11\x81b\0(\xF4V[`\0`\xA0\x82\x84\x03\x12\x15b\x007wW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x007\x9DWb\x007\x9Db\0)\xA8V[`@R\x82Qb\x007\xAD\x81b\0(\xF4V[\x81R` \x83\x01Qb\x007\xBF\x81b\0(\xF4V[` \x82\x01R`@\x83\x01Qb\x007\xD4\x81b\0(\xF4V[`@\x82\x01R``\x83\x01Qb\x007\xE9\x81b\0(\xF4V[``\x82\x01R`\x80\x83\x01Qb\x007\xFE\x81b\0(\xF4V[`\x80\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15b\08FWb\08Fb\x0028V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15b\08uWb\08ub\x0028V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15b\08\x94Wb\08\x94b\x0028V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15b\08\xADWb\08\xADb\x0028V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15b\08\xD0W`\0\x80\xFD[b\08\xDAb\0)\xBEV[\x90P\x81Qb\08\xE9\x81b\0*jV[\x81R` \x82\x01Qb\08\xFB\x81b\0*jV[` \x82\x01R`@\x82\x01Qb\09\x10\x81b\0*jV[`@\x82\x01R``\x82\x01Qb\09%\x81b\0*jV[``\x82\x01R`\x80\x82\x01Qb\0*\xF7\x81b\0(\xF4V[`\0\x81\x83\x03a\x02\0\x81\x12\x15b\09OW`\0\x80\xFD[b\09Yb\0)\xEAV[\x83Qb\09f\x81b\0(\xE1V[\x81R` \x84\x01Qb\09x\x81b\0(\xE1V[` \x82\x01R`@\x84\x01Qb\09\x8D\x81b\0(\xF4V[`@\x82\x01R``\x84\x01Qb\09\xA2\x81b\0(\xF4V[``\x82\x01R`\xE0`\x7F\x19\x83\x01\x12\x15b\09\xBAW`\0\x80\xFD[b\09\xC4b\0*\x10V[\x91P`\x80\x84\x01Qb\09\xD6\x81b\0,0V[\x82R`\xA0\x84\x01Qb\09\xE8\x81b\0(\xF4V[` \x83\x01R`\xC0\x84\x01Qb\09\xFD\x81b\0(\xF4V[`@\x83\x01R`\xE0\x84\x01Qb\0:\x12\x81b\0(\xF4V[``\x83\x01Rb\0:&a\x01\0\x85\x01b\x007WV[`\x80\x83\x01Rb\0::a\x01 \x85\x01b\x007WV[`\xA0\x83\x01Rb\0:Na\x01@\x85\x01b\x007WV[`\xC0\x83\x01R\x81`\x80\x82\x01Rb\0:i\x85a\x01`\x86\x01b\08\xBDV[`\xA0\x82\x01R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R\x83\x16` \x82\x01R``\x81\x01`\x02\x83\x10b\0:\xADWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82`@\x83\x01R\x94\x93PPPPV[`\0a\x01\0\x82\x84\x03\x12\x15b\0:\xCFW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0:\xF5Wb\0:\xF5b\0)\xA8V[`@R\x82Qb\0;\x05\x81b\0(\xE1V[\x81R` \x83\x01Qb\0;\x17\x81b\0(\xF4V[` \x82\x01R`@\x83\x01Qb\0;,\x81b\0(\xF4V[`@\x82\x01Rb\0;@\x84``\x85\x01b\08\xBDV[``\x82\x01R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x0F\x85\x81\x0B`@\x83\x01R\x84\x90\x0B``\x82\x01Ra\x02\0\x81\x01b\0;\xDB`\x80\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[\x82Q`\x03\x90\x81\x0Ba\x01`\x84\x01R` \x84\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x84\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x84\x01Q\x90\x0Ba\x01\xC0\x83\x01R`\x80\x83\x01Q`\x0F\x0Ba\x01\xE0\x83\x01Rb\x005|V[c\xFF\xFF\xFF\xFF\x85\x16\x81R`\x0F\x84\x81\x0B` \x83\x01R\x83\x90\x0B`@\x82\x01Ra\x01\0\x81\x01b\0<\x8B``\x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x95\x94PPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12b\0<\xACW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0<\xC8W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15b\0+\x85W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15b\0<\xF1W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0=\x17Wb\0=\x17b\0)\xA8V[`@R\x825b\0='\x81b\0,0V[\x81R` \x83\x015b\0=9\x81b\0(\xF4V[` \x82\x01R`@\x83\x015b\0=N\x81b\0(\xF4V[`@\x82\x01R``\x83\x015b\0=c\x81b\0(\xF4V[``\x82\x01Rb\0=v`\x80\x84\x01b\0)\x04V[`\x80\x82\x01Rb\0=\x89`\xA0\x84\x01b\0)\x04V[`\xA0\x82\x01Rb\0=\x9C`\xC0\x84\x01b\0)\x04V[`\xC0\x82\x01R\x93\x92PPPV[`\0a\x02\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Qb\0>O`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\xA0\x83\x01Q\x80Q`\x03\x90\x81\x0Ba\x01`\x85\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x85\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x85\x01R``\x82\x01Q\x90\x0Ba\x01\xC0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xE0\x84\x01Rb\x003\x03V\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0Bx8\x03\x80a\x0Bx\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x14V[a\083a\0\xACV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x03\x84\x90U`\x04\x80T\x92\x84\x16\x92\x90\x91\x16\x82\x17\x90U`@QG\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xA2W=`\0\x80>=`\0\xFD[PPPPPa\x01iV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x11W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01*W`\0\x80\xFD[\x84Qa\x015\x81a\0\xFCV[` \x86\x01Q\x90\x94Pa\x01F\x81a\0\xFCV[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa\x01^\x81a\0\xFCV[\x93\x96\x92\x95P\x90\x93PPV[a\n\0\x80a\x01x`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0CW\x80cqP\x18\xA6\x14a\x01\x04W\x80c\x8D\xA5\xCB[\x14a\x01\x19W\x80c\xF2\xFD\xE3\x8B\x14a\x01AW`\0\x80\xFD[\x80c&\x08o\x07\x14a\0\xAEW\x80cQ\xCF\xF8\xD9\x14a\0\xC3W\x80cT\xFDMP\x14a\0\xE3W`\0\x80\xFD[6a\0\xA9W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xA7W=`\0\x80>=`\0\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xBAW`\0\x80\xFD[Pa\0\xA7a\x01aV[4\x80\x15a\0\xCFW`\0\x80\xFD[Pa\0\xA7a\0\xDE6`\x04a\x07\x9EV[a\x04|V[4\x80\x15a\0\xEFW`\0\x80\xFD[P`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x10W`\0\x80\xFD[Pa\0\xA7a\x05\x05V[4\x80\x15a\x01%W`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xFBV[4\x80\x15a\x01MW`\0\x80\xFD[Pa\0\xA7a\x01\\6`\x04a\x07\x9EV[a\x05\x19V[`\x02T`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xD2\x91\x90\x81\x01\x90a\x07\xF1V[\x90P`\0[\x81Q\x81\x10\x15a\x04xW`\0\x82\x82\x81Q\x81\x10a\x01\xF4Wa\x01\xF4a\x08\xB6V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02w\x91\x90a\x08\xCCV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03A\x91\x90a\x08\xE9V[\x90P\x80\x15a\x04aW`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x90\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC0\x91\x90a\t\x02V[P`\x01T`\x03T`@Qc\"\x1F\t9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`D\x82\x01R`\x80`d\x82\x01R`\x02`\x84\x82\x01Ra-1`\xF0\x1B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\"\x1F\t9\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04HW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\\W=`\0\x80>=`\0\xFD[PPPP[PPPP\x80\x80a\x04p\x90a\t$V[\x91PPa\x01\xD7V[PPV[a\x04\x84a\x05\xA9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xEF\x91\x90a\x08\xE9V[\x90Pa\x04x`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x06\x03V[a\x05\ra\x05\xA9V[a\x05\x17`\0a\x07!V[V[a\x05!a\x05\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xCBV[a\x05\xA6\x81a\x07!V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xCBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x06t\x91\x90a\t{V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x06\xB1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xB6V[``\x91P[P\x91P\x91P\x81\x80\x15a\x06\xE0WP\x80Q\x15\x80a\x06\xE0WP\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xE0\x91\x90a\t\x02V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a*#`\xF1\x1B\x81RP\x90a\x07\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xCB\x91\x90a\t\x97V[PPPPPPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xB0W`\0\x80\xFD[\x815a\x07\xBB\x81a\x07\x89V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xECW`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x08\x04W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x1CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x080W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x08BWa\x08Ba\x07\xC2V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x08gWa\x08ga\x07\xC2V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x08\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x08\xAAWa\x08\x9B\x85a\x07\xD8V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x08\x8AV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xDEW`\0\x80\xFD[\x81Qa\x07\xBB\x81a\x07\x89V[`\0` \x82\x84\x03\x12\x15a\x08\xFBW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\t\x14W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xBBW`\0\x80\xFD[`\0`\x01\x82\x01a\tDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\tfW\x81\x81\x01Q\x83\x82\x01R` \x01a\tNV[\x83\x81\x11\x15a\tuW`\0\x84\x84\x01R[PPPPV[`\0\x82Qa\t\x8D\x81\x84` \x87\x01a\tKV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t\xB6\x81`@\x85\x01` \x87\x01a\tKV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 x8\xA2\x94\x0CY\x0C\xBE\\o\x1A\x06F\xF3\xAC\xEE!\xF1\x87\xFF,S\x82\xD2t\xBFmw\x10\0&\xDFdsolcC\0\x08\r\x003\xA2dipfsX\"\x12 \x05W\xC3\x90\x9F~q\xCF\x1A\x9B\xB6\x96w\xF9\xC7\x844\xA7n\r\xCA$O\x04\xABY\x84|\x13\xD3B\xAFdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static CONTRACTOWNER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x026W`\x005`\xE0\x1C\x80c\x8A)\xE2\xDE\x11b\0\x01BW\x80c\xB4\x96\xFB\xB4\x11b\0\0\xCCW\x80c\xD4\xA5\xEA\x85\x11b\0\0\x97W\x80c\xEB\xD6\xC2\x94\x11b\0\0zW\x80c\xEB\xD6\xC2\x94\x14b\0\x04\xC7W\x80c\xF2\xFD\xE3\x8B\x14b\0\x04\xDEW\x80c\xFB\x1C\xB0I\x14b\0\x04\xF5W`\0\x80\xFD[\x80c\xD4\xA5\xEA\x85\x14b\0\x04\xA6W\x80c\xE7{\xDCn\x14b\0\x04\xBDW`\0\x80\xFD[\x80c\xB4\x96\xFB\xB4\x14b\0\x04WW\x80c\xBB\xEF\x84\xB4\x14b\0\x04aW\x80c\xBE\x13\xBA\xC4\x14b\0\x04xW\x80c\xC2'\xDB\x96\x14b\0\x04\x8FW`\0\x80\xFD[\x80c\x8D\xB4\x8E\xCD\x11b\0\x01\rW\x80c\x8D\xB4\x8E\xCD\x14b\0\x04\x08W\x80c\x90]\x8AE\x14b\0\x04\x1FW\x80c\x90\x86\xA2\xA5\x14b\0\x04)W\x80c\x9Bj\xBA\x8F\x14b\0\x04@W`\0\x80\xFD[\x80c\x8A)\xE2\xDE\x14b\0\x03\xB1W\x80c\x8A\xB3\xDA\xAE\x14b\0\x03\xC8W\x80c\x8C\xA4g%\x14b\0\x03\xDFW\x80c\x8D\xA5\xCB[\x14b\0\x03\xF6W`\0\x80\xFD[\x80cS\x12\xB9\x1F\x11b\0\x01\xC4W\x80cp|\x8BX\x11b\0\x01\x8FW\x80cp|\x8BX\x14b\0\x03|W\x80cqP\x18\xA6\x14b\0\x03\x86W\x80cx\x8DBH\x14b\0\x03\x90W\x80c{gLz\x14b\0\x03\x9AW`\0\x80\xFD[\x80cS\x12\xB9\x1F\x14b\0\x03 W\x80cV\xE4\x9E\xF3\x14b\0\x037W\x80clleN\x14b\0\x03NW\x80cn\x13\xCB\xF3\x14b\0\x03eW`\0\x80\xFD[\x80c=\x14\x1D8\x11b\0\x02\x05W\x80c=\x14\x1D8\x14b\0\x02\xCFW\x80c=\xAB\xE0\xD9\x14b\0\x02\xD9W\x80cG\xE2\xCA\x90\x14b\0\x02\xF0W\x80cP\xE8'\x90\x14b\0\x03\x07W`\0\x80\xFD[\x80c\x05wU\x9F\x14b\0\x02;W\x80c\x05\x97Iv\x14b\0\x02ZW\x80c\x12\xEF\xA7\x1B\x14b\0\x02sW\x80c3\x92\xC5\x85\x14b\0\x02\xB8W[`\0\x80\xFD[b\0\x02Eb\0\x05\0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\0\x02qb\0\x02k6`\x04b\0))V[b\0\x05\x1AV[\0[b\0\x02\x9Fb\0\x02\x846`\x04b\0)\x8EV[`\xA3` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01b\0\x02QV[b\0\x02qb\0\x02\xC96`\x04b\0+\x02V[b\0\x06\xBBV[b\0\x02qb\0\x07)V[b\0\x02qb\0\x02\xEA6`\x04b\0+\x8CV[b\0\x08\x83V[b\0\x02Eb\0\x03\x016`\x04b\0,FV[b\0\n\xA5V[b\0\x03\x11b\0\x0EJV[`@Qb\0\x02Q\x91\x90b\0,mV[b\0\x02qb\0\x0316`\x04b\0,\xDAV[b\0\x0F\xBEV[b\0\x02qb\0\x03H6`\x04b\0-*V[b\0\x10\xC5V[b\0\x02qb\0\x03_6`\x04b\0-tV[b\0\x115V[b\0\x02\x9Fb\0\x03v6`\x04b\0)\x8EV[b\0\x11\xE7V[b\0\x02qb\0\x12\xA1V[b\0\x02qb\0\x131V[b\0\x03\x11b\0\x13GV[b\0\x02qb\0\x03\xAB6`\x04b\0.UV[b\0\x14\xB5V[b\0\x02qb\0\x03\xC26`\x04b\0.\xC0V[b\0\x18\xB9V[b\0\x02qb\0\x03\xD96`\x04b\0+\x02V[b\0\x1A\xCAV[b\0\x02qb\0\x03\xF06`\x04b\0/zV[b\0\x1B\x08V[`gT`\x01`\x01`\xA0\x1B\x03\x16b\0\x02\x9FV[b\0\x02qb\0\x04\x196`\x04b\0/\xC0V[b\0\x1C,V[b\0\x02qb\0\x1D\xE3V[b\0\x02qb\0\x04:6`\x04b\0)\x8EV[b\0\x1EGV[b\0\x02qb\0\x04Q6`\x04b\x000\xC4V[b\0\x1E\xAFV[b\0\x02qb\0!*V[b\0\x02qb\0\x04r6`\x04b\0)\x8EV[b\0!\x8EV[b\0\x02qb\0\x04\x896`\x04b\x001~V[b\0!\xCAV[b\0\x02qb\0\x04\xA06`\x04b\0,FV[b\0\"\x14V[b\0\x02qb\0\x04\xB76`\x04b\x001\xABV[b\0\"QV[b\0\x02qb\0\"\xE6V[b\0\x02qb\0\x04\xD86`\x04b\0,\xDAV[b\0#JV[b\0\x02qb\0\x04\xEF6`\x04b\0,FV[b\0$hV[`\xA0T\x15\x15b\0\x02EV[`\xA2T`\0\x90\x15\x15\x80b\0\x05\x15WP`\xA1T\x15\x15[\x90P\x90V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0b\0\x05\x80b\0\x13GV[\x90P`\0[\x81Q\x81\x10\x15b\0\x06)W\x81\x81\x81Q\x81\x10b\0\x05\xA4Wb\0\x05\xA4b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x03b\0\x06\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Ftrying to add a perp product twi`D\x82\x01Rb1\xB2\x97`\xE9\x1B`d\x82\x01R`\x84\x01b\0\x05kV[\x80b\0\x06 \x81b\x002NV[\x91PPb\0\x05\x85V[P`\xA2`@Q\x80`\x80\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85`\x0F\x0B\x81R` \x01\x84\x806\x03\x81\x01\x90b\0\x06h\x91\x90b\x002jV[\x90R`@Qb\0\x06|\x91\x90` \x01b\x002\x89V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x06\xB3\x94\x91\x90\x92\x01\x92\x01\x90b\0'0V[PPPPPPV[b\0\x06\xC5b\0$\xFEV[`\x9BT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x06\xF9\x90\x85\x90\x85\x90`\x04\x01b\x003\nV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x07\x14W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\xB3W=`\0\x80>=`\0\xFD[b\0\x073b\0$\xFEV[`\0[`\xA0T\x81\x10\x15b\0\x08rW`\0`\xA0\x82\x81T\x81\x10b\0\x07YWb\0\x07Yb\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x07p\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\x9E\x90b\x003_V[\x80\x15b\0\x07\xEFW\x80`\x1F\x10b\0\x07\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x07\xEFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x07\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PP`\x9CT`@Qcs\x02v\xCF`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x16\x93c\xE6\x04\xED\x9E\x93Pb\0\x08'\x92P\x85\x91P`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x08BW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x08WW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x08i\x90b\x002NV[\x91PPb\0\x076V[Pb\0\x08\x81`\xA0`\0b\0'\xBFV[V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x08\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[\x84\x83\x14b\0\t\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0[\x85\x81\x10\x15b\0\n\x9CW`\0`@Q\x80``\x01`@R\x80\x89\x89\x85\x81\x81\x10b\0\tIWb\0\tIb\x002\"V[\x90P` \x02\x01` \x81\x01\x90b\0\t`\x91\x90b\x004\x0BV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x87\x85\x81\x81\x10b\0\t\x80Wb\0\t\x80b\x002\"V[\x90P` \x02\x01` \x81\x01\x90b\0\t\x97\x91\x90b\x004+V[`\x0F\x0B\x81R` \x01\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91P`\x14\x90P\x82`@Q` \x01b\0\t\xEF\x91\x90b\x004KV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\n\x0F\x92\x91` \x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\nP\x90\x84\x90`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\nkW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\n\x80W=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\n\x93\x90b\x002NV[\x91PPb\0\t\x1EV[PPPPPPPV[`\0\x80`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\n\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x0B&\x91\x90\x81\x01\x90b\x004\xE2V[\x90P`\0[\x81Q\x81\x10\x15b\0\x0E@W`\0\x82\x82\x81Q\x81\x10b\0\x0BLWb\0\x0BLb\x002\"V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0B\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0B\xD2\x91\x90b\x005\x87V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x0C,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x05kV[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x82\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0CxW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0C\x9E\x91\x90b\x005\xA7V[`\x9FT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x84\x16\x03b\0\x0C\xD1Wb\0\x0C\xCE`\x01`\x01`\xA0\x1B\x03\x89\x161\x82b\x005\xC1V[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r6\x91\x90b\x005\xDCV[b\0\rC\x90`\x12b\x006\x01V[b\0\rP\x90`\nb\x007$V[b\0\r\\\x90\x82b\x0075V[`\x9AT`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r\xD7\x91\x90b\x007dV[`\x80\x01Q\x90Pb\0\r\xF2g\r\xE0\xB6\xB3\xA7d\0\0`\x05b\08\nV[`\x0F\x0Bb\0\x0E\r\x83\x83`\x0F\x0Bb\0%Z\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x12b\0\x0E%WP`\x01\x98\x97PPPPPPPPV[PPPPP\x80\x80b\0\x0E7\x90b\x002NV[\x91PPb\0\x0B+V[P`\0\x93\x92PPPV[`\xA1T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x0EnWb\0\x0Enb\0)\xA8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x0E\x98W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA1T\x81\x10\x15b\0\x0F\xB8W`\0`\xA1\x82\x81T\x81\x10b\0\x0E\xC1Wb\0\x0E\xC1b\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x0E\xD8\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x0F\x06\x90b\x003_V[\x80\x15b\0\x0FWW\x80`\x1F\x10b\0\x0F+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x0FWV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x0F9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x0Fq\x91\x90b\09:V[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x0F\x8DWb\0\x0F\x8Db\x002\"V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x0F\xAF\x81b\x002NV[\x91PPb\0\x0E\x9EV[P\x91\x90PV[b\0\x0F\xC8b\0$\xFEV[`\x9DT`@\x80Qc\x1Ft\xAC\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFB\xA5`\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x10\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x109\x91\x90b\x005\x87V[`@Qc\xFD\x8CR\xCD`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x92P\x90\x82\x16\x90c\xFD\x8CR\xCD\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x10\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x10\xBBW=`\0\x80>=`\0\xFD[PPPPPPPPV[b\0\x10\xCFb\0$\xFEV[`\x9DT`@QcV\xE4\x9E\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cV\xE4\x9E\xF3\x90b\0\x11\x05\x90\x86\x90\x86\x90\x86\x90`\x04\x01b\0:vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x11 W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\n\x9CW=`\0\x80>=`\0\xFD[b\0\x11?b\0$\xFEV[`@\x80Q\x80\x82\x01\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x81\x84\x01R\x82Q\x80\x82\x03\x84\x01\x81R``\x82\x01\x90\x93R\x90\x91`\0\x91b\0\x11\xA6\x91`\x12\x91`\x80\x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x10\x8B\x90\x84\x90`\x04\x01b\x003\xF6V[`\x9CT`\x9AT`\x9FT`@Q`\0\x93\x84\x93`\x01\x93`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93\x91\x83\x16\x92\x88\x92\x91\x16\x90b\0\x12\x1B\x90b\0'\xDFV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R`@\x83\x01\x91\x90\x91R\x90\x91\x16``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x12dW=`\0\x80>=`\0\xFD[P`\0\x93\x84R`\xA3` R`@\x90\x93 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UP\x90\x91\x90PV[b\0\x12\xABb\0$\xFEV[`@\x80Q`\t`\xF8\x1B` \x82\x01R\x81Q`\x01\x81\x83\x03\x01\x81R`!\x82\x01\x92\x83\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x90\x93R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x12\xFA\x90\x84\x90`%\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x13\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x13*W=`\0\x80>=`\0\xFD[PPPPPV[b\0\x13;b\0$\xFEV[b\0\x08\x81`\0b\0%\xE8V[`\xA2T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x13kWb\0\x13kb\0)\xA8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x13\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA2T\x81\x10\x15b\0\x0F\xB8W`\0`\xA2\x82\x81T\x81\x10b\0\x13\xBEWb\0\x13\xBEb\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x13\xD5\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x14\x03\x90b\x003_V[\x80\x15b\0\x14TW\x80`\x1F\x10b\0\x14(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x14TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x146W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x14n\x91\x90b\0:\xBBV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x14\x8AWb\0\x14\x8Ab\x002\"V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0\x14\xAC\x81b\x002NV[\x91PPb\0\x13\x9BV[b\0\x14\xBFb\0$\xFEV[`\0[`\xA1T\x81\x10\x15b\0\x16\xB1W`\0`\xA1\x82\x81T\x81\x10b\0\x14\xE5Wb\0\x14\xE5b\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x14\xFC\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x15*\x90b\x003_V[\x80\x15b\0\x15{W\x80`\x1F\x10b\0\x15OWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x15{V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x15]W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x15\x95\x91\x90b\09:V[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10b\0\x15\xB7Wb\0\x15\xB7b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x16\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fspot id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x05kV[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Qc%\xBC\xE8\xC9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96c%\xBC\xE8\xC9\x96b\0\x16f\x96\x90\x95\x90\x94\x93\x92\x91`\x04\x01b\0;LV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x16\x81W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x16\x96W=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x16\xA8\x90b\x002NV[\x91PPb\0\x14\xC2V[Pb\0\x16\xC0`\xA1`\0b\0'\xBFV[`\0[`\xA2T\x81\x10\x15b\0\x18\xA6W`\0`\xA2\x82\x81T\x81\x10b\0\x16\xE6Wb\0\x16\xE6b\x002\"V[\x90`\0R` `\0 \x01\x80Tb\0\x16\xFD\x90b\x003_V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x17+\x90b\x003_V[\x80\x15b\0\x17|W\x80`\x1F\x10b\0\x17PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x17|V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x17^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x17\x96\x91\x90b\0:\xBBV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10b\0\x17\xB8Wb\0\x17\xB8b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x18\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fperp id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x05kV[`\x9BT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xD7\xAA\xAC\xFF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x94c\xD7\xAA\xAC\xFF\x94b\0\x18[\x94\x90\x93\x90\x92\x91`\x04\x01b\0<#V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x18vW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x18\x8BW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x18\x9D\x90b\x002NV[\x91PPb\0\x16\xC3V[Pb\0\x18\xB5`\xA2`\0b\0'\xBFV[PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x18\xDAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x18\xF6WP0;\x15\x80\x15b\0\x18\xF6WP`\0T`\xFF\x16`\x01\x14[b\0\x19jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x05kV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x19\x8EW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x88\x163\x14b\0\x19\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fexpected deployed to initialize\0`D\x82\x01R`d\x01b\0\x05kV[b\0\x19\xF2b\0&GV[b\0\x19\xFD\x89b\0$hV[`\x99\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x17\x90\x92U`\x9A\x80T\x82\x16\x8A\x84\x16\x17\x90U`\x9B\x80T\x82\x16\x89\x84\x16\x17\x90U`\x9C\x80T\x82\x16\x88\x84\x16\x17\x90U`\x9D\x80T\x82\x16\x87\x84\x16\x17\x90U`\x9E\x80T\x82\x16\x86\x84\x16\x17\x90U`\x9F\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15b\0\x1A\xBFW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[b\0\x1A\xD4b\0$\xFEV[`\x9AT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x06\xF9\x90\x85\x90\x85\x90`\x04\x01b\x003\nV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1B^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0[\x81\x81\x10\x15b\0\x1C'W`\0\x83\x83\x83\x81\x81\x10b\0\x1B\x81Wb\0\x1B\x81b\x002\"V[\x90P` \x02\x81\x01\x90b\0\x1B\x95\x91\x90b\0<\x94V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP`\xA0\x80T`\x01\x81\x01\x82U\x91R\x84Q\x94\x95Pb\0\x1C\x0F\x94\x7Fx\xFD\xC8\xD4\"\xC4\x9C\xED\x03Z\x9E\xDF\x18\xD0\r<j\x8D\x81\xDF!\x0F>^D\x8E\x04^w\xB4\x1E\x88\x90\x91\x01\x93P` \x86\x01\x92P\x90Pb\0'0V[PP\x80\x80b\0\x1C\x1E\x90b\x002NV[\x91PPb\0\x1BaV[PPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1C\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0b\0\x1C\x8Eb\0\x0EJV[\x90P`\0[\x81Q\x81\x10\x15b\0\x1D7W\x81\x81\x81Q\x81\x10b\0\x1C\xB2Wb\0\x1C\xB2b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x88c\xFF\xFF\xFF\xFF\x16\x03b\0\x1D\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Ftrying to add a spot product twi`D\x82\x01Rb1\xB2\x97`\xE9\x1B`d\x82\x01R`\x84\x01b\0\x05kV[\x80b\0\x1D.\x81b\x002NV[\x91PPb\0\x1C\x93V[P`\xA1`@Q\x80`\xC0\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\x0F\x0B\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85\x806\x03\x81\x01\x90b\0\x1D\x82\x91\x90b\0<\xDEV[\x81R` \x01b\0\x1D\x986\x86\x90\x03\x86\x01\x86b\x002jV[\x90R`@Qb\0\x1D\xAC\x91\x90` \x01b\0=\xA8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x10\xBB\x94\x91\x90\x92\x01\x92\x01\x90b\0'0V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x1E9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[b\0\x08\x81`\xA2`\0b\0'\xBFV[`\0\x81\x81R`\xA3` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x1EsWb\0\x1Ep\x82b\0\x11\xE7V[\x90P[\x80`\x01`\x01`\xA0\x1B\x03\x16c&\x08o\x07`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x07\x14W`\0\x80\xFD[b\0\x1E\xB9b\0$\xFEV[\x82Q\x84Q\x14b\0\x1E\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[\x81Q\x84Q\x14b\0\x1FAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[\x80Q\x84Q\x14b\0\x1F\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x05kV[`\0[\x84Q\x81\x10\x15b\0\x13*W`\0`@Q\x80`\x80\x01`@R\x80\x87\x84\x81Q\x81\x10b\0\x1F\xB4Wb\0\x1F\xB4b\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x84\x81Q\x81\x10b\0\x1F\xDCWb\0\x1F\xDCb\x002\"V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85\x84\x81Q\x81\x10b\0 \x04Wb\0 \x04b\x002\"V[` \x02` \x01\x01Q`\x0F\x0B\x81R` \x01\x84\x84\x81Q\x81\x10b\0 )Wb\0 )b\x002\"V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0F\x0B\x90R\x90P`\0`\x18`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x86\x01Q\x16\x81\x83\x01R\x90\x84\x01Q`\x0F\x90\x81\x0B``\x80\x84\x01\x91\x90\x91R\x85\x01Q\x90\x0B`\x80\x82\x01R`\xA0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0 \x9D\x92\x91` \x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0 \xDE\x90\x84\x90`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0 \xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0!\x0EW=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0!!\x90b\x002NV[\x91PPb\0\x1F\x88V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0!\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[b\0\x08\x81`\xA0`\0b\0'\xBFV[b\0!\x98b\0$\xFEV[`\x9ET`@Qc.\xFB\xE1-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\xEF\x84\xB4\x90`$\x01b\0\x12\xFAV[b\0!\xD4b\0$\xFEV[`\x9ET`@Qc/\x84\xEE\xB1`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\x13\xBA\xC4\x90`d\x01b\0\x11\x05V[b\0\"\x1Eb\0$\xFEV[`\x9DT`@Qca\x13\xED\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC2'\xDB\x96\x90`$\x01b\0\x12\xFAV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\"\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[`\xA0\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91Rb\0\x1C'\x90\x7Fx\xFD\xC8\xD4\"\xC4\x9C\xED\x03Z\x9E\xDF\x18\xD0\r<j\x8D\x81\xDF!\x0F>^D\x8E\x04^w\xB4\x1E\x88\x01\x83\x83b\0'\xEDV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0#<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x05kV[b\0\x08\x81`\xA1`\0b\0'\xBFV[b\0#Tb\0$\xFEV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x85\x16\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x10`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0#\xF7\x92\x91` \x01b\x004\xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0$8\x90\x84\x90`\x04\x01b\x003\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0$SW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1A\xBFW=`\0\x80>=`\0\xFD[b\0$rb\0$\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0$\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x05kV[b\0$\xFB\x81b\0%\xE8V[PV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x08\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x05kV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90b\0%\xA2WPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90b\0%\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x05k\x91\x90b\x003\xF6V[P\x90P[\x92\x91PPV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0&\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x05kV[b\0\x08\x81`\0Ta\x01\0\x90\x04`\xFF\x16b\0'%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x05kV[b\0\x08\x813b\0%\xE8V[\x82\x80Tb\0'>\x90b\x003_V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0'bW`\0\x85Ub\0'\xADV[\x82`\x1F\x10b\0'}W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0'\xADV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0'\xADW\x91\x82\x01[\x82\x81\x11\x15b\0'\xADW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0'\x90V[Pb\0'\xBB\x92\x91Pb\0(jV[P\x90V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0$\xFB\x91\x90b\0(\x81V[a\x0Bx\x80b\0>\x9E\x839\x01\x90V[\x82\x80Tb\0'\xFB\x90b\x003_V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0(\x1FW`\0\x85Ub\0'\xADV[\x82`\x1F\x10b\0(:W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ub\0'\xADV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0'\xADW\x91\x82\x01[\x82\x81\x11\x15b\0'\xADW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0(MV[[\x80\x82\x11\x15b\0'\xBBW`\0\x81U`\x01\x01b\0(kV[\x80\x82\x11\x15b\0'\xBBW`\0b\0(\x98\x82\x82b\0(\xA2V[P`\x01\x01b\0(\x81V[P\x80Tb\0(\xB0\x90b\x003_V[`\0\x82U\x80`\x1F\x10b\0(\xC1WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0$\xFB\x91\x90b\0(jV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0$\xFBW`\0\x80\xFD[\x80`\x0F\x0B\x81\x14b\0$\xFBW`\0\x80\xFD[\x805b\0)\x11\x81b\0(\xF4V[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15b\0\x0F\xB8W`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15b\0)AW`\0\x80\xFD[\x845b\0)N\x81b\0(\xE1V[\x93P` \x85\x015b\0)`\x81b\0(\xF4V[\x92P`@\x85\x015b\0)r\x81b\0(\xF4V[\x91Pb\0)\x83\x86``\x87\x01b\0)\x16V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15b\0)\xA1W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0)\xE4Wb\0)\xE4b\0)\xA8V[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0)\xE4Wb\0)\xE4b\0)\xA8V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0)\xE4Wb\0)\xE4b\0)\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*bWb\0*bb\0)\xA8V[`@R\x91\x90PV[\x80`\x03\x0B\x81\x14b\0$\xFBW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15b\0*\x8DW`\0\x80\xFD[b\0*\x97b\0)\xBEV[\x90P\x815b\0*\xA6\x81b\0*jV[\x81R` \x82\x015b\0*\xB8\x81b\0*jV[` \x82\x01R`@\x82\x015b\0*\xCD\x81b\0*jV[`@\x82\x01R``\x82\x015b\0*\xE2\x81b\0*jV[``\x82\x01R`\x80\x82\x015b\0*\xF7\x81b\0(\xF4V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15b\0+\x16W`\0\x80\xFD[\x825b\0+#\x81b\0(\xE1V[\x91Pb\0+4\x84` \x85\x01b\0*zV[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0+PW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0+iW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0+\x85W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15b\0+\xA6W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0+\xBFW`\0\x80\xFD[b\0+\xCD\x8A\x83\x8B\x01b\0+=V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15b\0+\xE7W`\0\x80\xFD[b\0+\xF5\x8A\x83\x8B\x01b\0+=V[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15b\0,\x0FW`\0\x80\xFD[Pb\0,\x1E\x89\x82\x8A\x01b\0+=V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0$\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15b\0,YW`\0\x80\xFD[\x815b\0,f\x81b\0,0V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0,\xADW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0,\x89V[P\x90\x96\x95PPPPPPV[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0)\x11W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0,\xF0W`\0\x80\xFD[\x835b\0,\xFD\x81b\0(\xE1V[\x92Pb\0-\r` \x85\x01b\0,\xB9V[\x91P`@\x84\x015b\0-\x1F\x81b\0,0V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0-@W`\0\x80\xFD[\x835b\0-M\x81b\0,0V[\x92P` \x84\x015b\0-_\x81b\0,0V[\x91P`@\x84\x015`\x02\x81\x10b\0-\x1FW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0-\x88W`\0\x80\xFD[b\0-\x93\x83b\0,\xB9V[\x91P` \x83\x015b\0-\xA5\x81b\0,0V[\x80\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0-\xCDWb\0-\xCDb\0)\xA8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0-\xE9W`\0\x80\xFD[\x815` b\0.\x02b\0-\xFC\x83b\0-\xB0V[b\0*6V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0.\"W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0.JW\x805b\0.<\x81b\0(\xE1V[\x83R\x91\x83\x01\x91\x83\x01b\0.&V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0.iW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0.\x82W`\0\x80\xFD[b\0.\x90\x86\x83\x87\x01b\0-\xD7V[\x93P` \x85\x015\x91P\x80\x82\x11\x15b\0.\xA7W`\0\x80\xFD[Pb\0.\xB6\x85\x82\x86\x01b\0-\xD7V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15b\0.\xDEW`\0\x80\xFD[\x885b\0.\xEB\x81b\0,0V[\x97P` \x89\x015b\0.\xFD\x81b\0,0V[\x96P`@\x89\x015b\0/\x0F\x81b\0,0V[\x95P``\x89\x015b\0/!\x81b\0,0V[\x94P`\x80\x89\x015b\0/3\x81b\0,0V[\x93P`\xA0\x89\x015b\0/E\x81b\0,0V[\x92P`\xC0\x89\x015b\0/W\x81b\0,0V[\x91P`\xE0\x89\x015b\0/i\x81b\0,0V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80` \x83\x85\x03\x12\x15b\0/\x8EW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0/\xA6W`\0\x80\xFD[b\0/\xB4\x85\x82\x86\x01b\0+=V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15b\0/\xDCW`\0\x80\xFD[\x875b\0/\xE9\x81b\0(\xE1V[\x96P` \x88\x015b\0/\xFB\x81b\0(\xE1V[\x95P`@\x88\x015b\x000\r\x81b\0(\xF4V[\x94P``\x88\x015b\x000\x1F\x81b\0(\xF4V[\x93P`\xE0`\x7F\x19\x82\x01\x12\x15b\x0004W`\0\x80\xFD[P`\x80\x87\x01\x91Pb\x000K\x88a\x01`\x89\x01b\0)\x16V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x82`\x1F\x83\x01\x12b\x000iW`\0\x80\xFD[\x815` b\x000|b\0-\xFC\x83b\0-\xB0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\x000\x9CW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0.JW\x805b\x000\xB6\x81b\0(\xF4V[\x83R\x91\x83\x01\x91\x83\x01b\x000\xA0V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\x000\xDBW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\x000\xF4W`\0\x80\xFD[b\x001\x02\x88\x83\x89\x01b\0-\xD7V[\x95P` \x87\x015\x91P\x80\x82\x11\x15b\x001\x19W`\0\x80\xFD[b\x001'\x88\x83\x89\x01b\0-\xD7V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\x001>W`\0\x80\xFD[b\x001L\x88\x83\x89\x01b\x000WV[\x93P``\x87\x015\x91P\x80\x82\x11\x15b\x001cW`\0\x80\xFD[Pb\x001r\x87\x82\x88\x01b\x000WV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\x001\x94W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15b\x001\xBFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\x001\xD8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\x001\xEDW`\0\x80\xFD[\x815\x81\x81\x11\x15b\x001\xFDW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15b\x002\x10W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\x002cWb\x002cb\x0028V[P`\x01\x01\x90V[`\0`\xA0\x82\x84\x03\x12\x15b\x002}W`\0\x80\xFD[b\0,f\x83\x83b\0*zV[`\0a\x01\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qb\x003\x03``\x84\x01\x82\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P\x92\x91PPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`\xC0\x81\x01b\0,f` \x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\x003tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x0F\xB8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\x003\xB2W\x81\x81\x01Q\x83\x82\x01R` \x01b\x003\x98V[\x83\x81\x11\x15b\x003\xC2W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\x003\xE2\x81` \x86\x01` \x86\x01b\x003\x95V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0,f` \x83\x01\x84b\x003\xC8V[`\0` \x82\x84\x03\x12\x15b\x004\x1EW`\0\x80\xFD[\x815b\0,f\x81b\0(\xE1V[`\0` \x82\x84\x03\x12\x15b\x004>W`\0\x80\xFD[\x815b\0,f\x81b\0(\xF4V[`\0` \x80\x83R`\x80\x83\x01c\xFF\xFF\xFF\xFF\x85Q\x16\x82\x85\x01R\x81\x85\x01Q`\x0F\x0B`@\x85\x01R`@\x85\x01Q``\x80\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P\x84\x83\x01\x93P`\0\x92P[\x80\x83\x10\x15b\0.JW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90b\x004\x8FV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qb\x004\xD4\x81`\x01\x85\x01` \x87\x01b\x003\x95V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15b\x004\xF6W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\x005\x0EW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13b\x005 W`\0\x80\xFD[\x80Qb\x0051b\0-\xFC\x82b\0-\xB0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15b\x005QW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15b\x005|W\x83Qb\x005l\x81b\0(\xE1V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90b\x005VV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\x005\x9AW`\0\x80\xFD[\x81Qb\0,f\x81b\0,0V[`\0` \x82\x84\x03\x12\x15b\x005\xBAW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15b\x005\xD7Wb\x005\xD7b\x0028V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\x005\xEFW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0,fW`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\x006\x1EWb\x006\x1Eb\x0028V[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15b\x006hW\x81`\0\x19\x04\x82\x11\x15b\x006LWb\x006Lb\x0028V[\x80\x85\x16\x15b\x006ZW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\x006,V[P\x92P\x92\x90PV[`\0\x82b\x006\x81WP`\x01b\0%\xE2V[\x81b\x006\x90WP`\0b\0%\xE2V[\x81`\x01\x81\x14b\x006\xA9W`\x02\x81\x14b\x006\xB4Wb\x006\xD4V[`\x01\x91PPb\0%\xE2V[`\xFF\x84\x11\x15b\x006\xC8Wb\x006\xC8b\x0028V[PP`\x01\x82\x1Bb\0%\xE2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\x006\xF9WP\x81\x81\nb\0%\xE2V[b\x007\x05\x83\x83b\x006'V[\x80`\0\x19\x04\x82\x11\x15b\x007\x1CWb\x007\x1Cb\x0028V[\x02\x93\x92PPPV[`\0b\0,f`\xFF\x84\x16\x83b\x006pV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\x007RWb\x007Rb\x0028V[P\x02\x90V[\x80Qb\0)\x11\x81b\0(\xF4V[`\0`\xA0\x82\x84\x03\x12\x15b\x007wW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x007\x9DWb\x007\x9Db\0)\xA8V[`@R\x82Qb\x007\xAD\x81b\0(\xF4V[\x81R` \x83\x01Qb\x007\xBF\x81b\0(\xF4V[` \x82\x01R`@\x83\x01Qb\x007\xD4\x81b\0(\xF4V[`@\x82\x01R``\x83\x01Qb\x007\xE9\x81b\0(\xF4V[``\x82\x01R`\x80\x83\x01Qb\x007\xFE\x81b\0(\xF4V[`\x80\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15b\08FWb\08Fb\x0028V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15b\08uWb\08ub\x0028V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15b\08\x94Wb\08\x94b\x0028V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15b\08\xADWb\08\xADb\x0028V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0`\xA0\x82\x84\x03\x12\x15b\08\xD0W`\0\x80\xFD[b\08\xDAb\0)\xBEV[\x90P\x81Qb\08\xE9\x81b\0*jV[\x81R` \x82\x01Qb\08\xFB\x81b\0*jV[` \x82\x01R`@\x82\x01Qb\09\x10\x81b\0*jV[`@\x82\x01R``\x82\x01Qb\09%\x81b\0*jV[``\x82\x01R`\x80\x82\x01Qb\0*\xF7\x81b\0(\xF4V[`\0\x81\x83\x03a\x02\0\x81\x12\x15b\09OW`\0\x80\xFD[b\09Yb\0)\xEAV[\x83Qb\09f\x81b\0(\xE1V[\x81R` \x84\x01Qb\09x\x81b\0(\xE1V[` \x82\x01R`@\x84\x01Qb\09\x8D\x81b\0(\xF4V[`@\x82\x01R``\x84\x01Qb\09\xA2\x81b\0(\xF4V[``\x82\x01R`\xE0`\x7F\x19\x83\x01\x12\x15b\09\xBAW`\0\x80\xFD[b\09\xC4b\0*\x10V[\x91P`\x80\x84\x01Qb\09\xD6\x81b\0,0V[\x82R`\xA0\x84\x01Qb\09\xE8\x81b\0(\xF4V[` \x83\x01R`\xC0\x84\x01Qb\09\xFD\x81b\0(\xF4V[`@\x83\x01R`\xE0\x84\x01Qb\0:\x12\x81b\0(\xF4V[``\x83\x01Rb\0:&a\x01\0\x85\x01b\x007WV[`\x80\x83\x01Rb\0::a\x01 \x85\x01b\x007WV[`\xA0\x83\x01Rb\0:Na\x01@\x85\x01b\x007WV[`\xC0\x83\x01R\x81`\x80\x82\x01Rb\0:i\x85a\x01`\x86\x01b\08\xBDV[`\xA0\x82\x01R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R\x83\x16` \x82\x01R``\x81\x01`\x02\x83\x10b\0:\xADWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82`@\x83\x01R\x94\x93PPPPV[`\0a\x01\0\x82\x84\x03\x12\x15b\0:\xCFW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0:\xF5Wb\0:\xF5b\0)\xA8V[`@R\x82Qb\0;\x05\x81b\0(\xE1V[\x81R` \x83\x01Qb\0;\x17\x81b\0(\xF4V[` \x82\x01R`@\x83\x01Qb\0;,\x81b\0(\xF4V[`@\x82\x01Rb\0;@\x84``\x85\x01b\08\xBDV[``\x82\x01R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x0F\x85\x81\x0B`@\x83\x01R\x84\x90\x0B``\x82\x01Ra\x02\0\x81\x01b\0;\xDB`\x80\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[\x82Q`\x03\x90\x81\x0Ba\x01`\x84\x01R` \x84\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x84\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x84\x01Q\x90\x0Ba\x01\xC0\x83\x01R`\x80\x83\x01Q`\x0F\x0Ba\x01\xE0\x83\x01Rb\x005|V[c\xFF\xFF\xFF\xFF\x85\x16\x81R`\x0F\x84\x81\x0B` \x83\x01R\x83\x90\x0B`@\x82\x01Ra\x01\0\x81\x01b\0<\x8B``\x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x95\x94PPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12b\0<\xACW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0<\xC8W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15b\0+\x85W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15b\0<\xF1W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0=\x17Wb\0=\x17b\0)\xA8V[`@R\x825b\0='\x81b\0,0V[\x81R` \x83\x015b\0=9\x81b\0(\xF4V[` \x82\x01R`@\x83\x015b\0=N\x81b\0(\xF4V[`@\x82\x01R``\x83\x015b\0=c\x81b\0(\xF4V[``\x82\x01Rb\0=v`\x80\x84\x01b\0)\x04V[`\x80\x82\x01Rb\0=\x89`\xA0\x84\x01b\0)\x04V[`\xA0\x82\x01Rb\0=\x9C`\xC0\x84\x01b\0)\x04V[`\xC0\x82\x01R\x93\x92PPPV[`\0a\x02\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Qb\0>O`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\xA0\x83\x01Q\x80Q`\x03\x90\x81\x0Ba\x01`\x85\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x85\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x85\x01R``\x82\x01Q\x90\x0Ba\x01\xC0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xE0\x84\x01Rb\x003\x03V\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0Bx8\x03\x80a\x0Bx\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x14V[a\083a\0\xACV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x03\x84\x90U`\x04\x80T\x92\x84\x16\x92\x90\x91\x16\x82\x17\x90U`@QG\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xA2W=`\0\x80>=`\0\xFD[PPPPPa\x01iV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x11W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01*W`\0\x80\xFD[\x84Qa\x015\x81a\0\xFCV[` \x86\x01Q\x90\x94Pa\x01F\x81a\0\xFCV[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa\x01^\x81a\0\xFCV[\x93\x96\x92\x95P\x90\x93PPV[a\n\0\x80a\x01x`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0CW\x80cqP\x18\xA6\x14a\x01\x04W\x80c\x8D\xA5\xCB[\x14a\x01\x19W\x80c\xF2\xFD\xE3\x8B\x14a\x01AW`\0\x80\xFD[\x80c&\x08o\x07\x14a\0\xAEW\x80cQ\xCF\xF8\xD9\x14a\0\xC3W\x80cT\xFDMP\x14a\0\xE3W`\0\x80\xFD[6a\0\xA9W`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\0\xA7W=`\0\x80>=`\0\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xBAW`\0\x80\xFD[Pa\0\xA7a\x01aV[4\x80\x15a\0\xCFW`\0\x80\xFD[Pa\0\xA7a\0\xDE6`\x04a\x07\x9EV[a\x04|V[4\x80\x15a\0\xEFW`\0\x80\xFD[P`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x10W`\0\x80\xFD[Pa\0\xA7a\x05\x05V[4\x80\x15a\x01%W`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xFBV[4\x80\x15a\x01MW`\0\x80\xFD[Pa\0\xA7a\x01\\6`\x04a\x07\x9EV[a\x05\x19V[`\x02T`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xD2\x91\x90\x81\x01\x90a\x07\xF1V[\x90P`\0[\x81Q\x81\x10\x15a\x04xW`\0\x82\x82\x81Q\x81\x10a\x01\xF4Wa\x01\xF4a\x08\xB6V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02w\x91\x90a\x08\xCCV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03A\x91\x90a\x08\xE9V[\x90P\x80\x15a\x04aW`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x90\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC0\x91\x90a\t\x02V[P`\x01T`\x03T`@Qc\"\x1F\t9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`D\x82\x01R`\x80`d\x82\x01R`\x02`\x84\x82\x01Ra-1`\xF0\x1B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\"\x1F\t9\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04HW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\\W=`\0\x80>=`\0\xFD[PPPP[PPPP\x80\x80a\x04p\x90a\t$V[\x91PPa\x01\xD7V[PPV[a\x04\x84a\x05\xA9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xEF\x91\x90a\x08\xE9V[\x90Pa\x04x`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x06\x03V[a\x05\ra\x05\xA9V[a\x05\x17`\0a\x07!V[V[a\x05!a\x05\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xCBV[a\x05\xA6\x81a\x07!V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xCBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x06t\x91\x90a\t{V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x06\xB1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xB6V[``\x91P[P\x91P\x91P\x81\x80\x15a\x06\xE0WP\x80Q\x15\x80a\x06\xE0WP\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xE0\x91\x90a\t\x02V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a*#`\xF1\x1B\x81RP\x90a\x07\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xCB\x91\x90a\t\x97V[PPPPPPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xB0W`\0\x80\xFD[\x815a\x07\xBB\x81a\x07\x89V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xECW`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x08\x04W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x1CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x080W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x08BWa\x08Ba\x07\xC2V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x08gWa\x08ga\x07\xC2V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\x08\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x08\xAAWa\x08\x9B\x85a\x07\xD8V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x08\x8AV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xDEW`\0\x80\xFD[\x81Qa\x07\xBB\x81a\x07\x89V[`\0` \x82\x84\x03\x12\x15a\x08\xFBW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\t\x14W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xBBW`\0\x80\xFD[`\0`\x01\x82\x01a\tDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\tfW\x81\x81\x01Q\x83\x82\x01R` \x01a\tNV[\x83\x81\x11\x15a\tuW`\0\x84\x84\x01R[PPPPV[`\0\x82Qa\t\x8D\x81\x84` \x87\x01a\tKV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t\xB6\x81`@\x85\x01` \x87\x01a\tKV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 x8\xA2\x94\x0CY\x0C\xBE\\o\x1A\x06F\xF3\xAC\xEE!\xF1\x87\xFF,S\x82\xD2t\xBFmw\x10\0&\xDFdsolcC\0\x08\r\x003\xA2dipfsX\"\x12 \x05W\xC3\x90\x9F~q\xCF\x1A\x9B\xB6\x96w\xF9\xC7\x844\xA7n\r\xCA$O\x04\xABY\x84|\x13\xD3B\xAFdsolcC\0\x08\r\x003";
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
        ///Calls the contract's `addProducts` (0x7b674c7a) function
        pub fn add_products(
            &self,
            spot_ids: ::std::vec::Vec<u32>,
            perp_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 103, 76, 122], (spot_ids, perp_ids))
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
        ///Calls the contract's `batchSubmitUpdateProductTxs` (0x8ca46725) function
        pub fn batch_submit_update_product_txs(
            &self,
            slow_mode_txs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 164, 103, 37], slow_mode_txs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearPerpAddProductCalls` (0x905d8a45) function
        pub fn clear_perp_add_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 93, 138, 69], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearSpotAddProductCalls` (0xe77bdc6e) function
        pub fn clear_spot_add_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 123, 220, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearUpdateProductTxs` (0xb496fbb4) function
        pub fn clear_update_product_txs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 150, 251, 180], ())
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
        ///Calls the contract's `hasPendingAddProductCalls` (0x0577559f) function
        pub fn has_pending_add_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([5, 119, 85, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasPendingUpdateProductTxs` (0xfb1cb049) function
        pub fn has_pending_update_product_txs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([251, 28, 176, 73], ())
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
        ///Calls the contract's `pendingPerpAddProductIds` (0x788d4248) function
        pub fn pending_perp_add_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([120, 141, 66, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingSpotAddProductIds` (0x50e82790) function
        pub fn pending_spot_add_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([80, 232, 39, 144], ())
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
        ///Calls the contract's `submitPerpAddProductCall` (0x05974976) function
        pub fn submit_perp_add_product_call(
            &self,
            product_id: u32,
            size_increment: i128,
            min_size: i128,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [5, 151, 73, 118],
                    (product_id, size_increment, min_size, risk_store),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitSpotAddProductCall` (0x8db48ecd) function
        pub fn submit_spot_add_product_call(
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
                    [141, 180, 142, 205],
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
        ///Calls the contract's `submitUpdateProductTx` (0xd4a5ea85) function
        pub fn submit_update_product_tx(
            &self,
            slow_mode_tx: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 165, 234, 133], slow_mode_tx)
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
        ///Calls the contract's `updateProducts` (0x3d141d38) function
        pub fn update_products(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 20, 29, 56], ())
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
    ///Container type for all input parameters for the `addProducts` function with signature `addProducts(uint32[],uint32[])` and selector `0x7b674c7a`
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
    #[ethcall(name = "addProducts", abi = "addProducts(uint32[],uint32[])")]
    pub struct AddProductsCall {
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
    ///Container type for all input parameters for the `batchSubmitUpdateProductTxs` function with signature `batchSubmitUpdateProductTxs(bytes[])` and selector `0x8ca46725`
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
        name = "batchSubmitUpdateProductTxs",
        abi = "batchSubmitUpdateProductTxs(bytes[])"
    )]
    pub struct BatchSubmitUpdateProductTxsCall {
        pub slow_mode_txs: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `clearPerpAddProductCalls` function with signature `clearPerpAddProductCalls()` and selector `0x905d8a45`
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
    #[ethcall(name = "clearPerpAddProductCalls", abi = "clearPerpAddProductCalls()")]
    pub struct ClearPerpAddProductCallsCall;
    ///Container type for all input parameters for the `clearSpotAddProductCalls` function with signature `clearSpotAddProductCalls()` and selector `0xe77bdc6e`
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
    #[ethcall(name = "clearSpotAddProductCalls", abi = "clearSpotAddProductCalls()")]
    pub struct ClearSpotAddProductCallsCall;
    ///Container type for all input parameters for the `clearUpdateProductTxs` function with signature `clearUpdateProductTxs()` and selector `0xb496fbb4`
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
    #[ethcall(name = "clearUpdateProductTxs", abi = "clearUpdateProductTxs()")]
    pub struct ClearUpdateProductTxsCall;
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
    ///Container type for all input parameters for the `hasPendingAddProductCalls` function with signature `hasPendingAddProductCalls()` and selector `0x0577559f`
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
        name = "hasPendingAddProductCalls",
        abi = "hasPendingAddProductCalls()"
    )]
    pub struct HasPendingAddProductCallsCall;
    ///Container type for all input parameters for the `hasPendingUpdateProductTxs` function with signature `hasPendingUpdateProductTxs()` and selector `0xfb1cb049`
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
        name = "hasPendingUpdateProductTxs",
        abi = "hasPendingUpdateProductTxs()"
    )]
    pub struct HasPendingUpdateProductTxsCall;
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
    ///Container type for all input parameters for the `pendingPerpAddProductIds` function with signature `pendingPerpAddProductIds()` and selector `0x788d4248`
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
    #[ethcall(name = "pendingPerpAddProductIds", abi = "pendingPerpAddProductIds()")]
    pub struct PendingPerpAddProductIdsCall;
    ///Container type for all input parameters for the `pendingSpotAddProductIds` function with signature `pendingSpotAddProductIds()` and selector `0x50e82790`
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
    #[ethcall(name = "pendingSpotAddProductIds", abi = "pendingSpotAddProductIds()")]
    pub struct PendingSpotAddProductIdsCall;
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
    ///Container type for all input parameters for the `submitPerpAddProductCall` function with signature `submitPerpAddProductCall(uint32,int128,int128,(int32,int32,int32,int32,int128))` and selector `0x05974976`
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
        name = "submitPerpAddProductCall",
        abi = "submitPerpAddProductCall(uint32,int128,int128,(int32,int32,int32,int32,int128))"
    )]
    pub struct SubmitPerpAddProductCallCall {
        pub product_id: u32,
        pub size_increment: i128,
        pub min_size: i128,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `submitSpotAddProductCall` function with signature `submitSpotAddProductCall(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `0x8db48ecd`
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
        name = "submitSpotAddProductCall",
        abi = "submitSpotAddProductCall(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct SubmitSpotAddProductCallCall {
        pub product_id: u32,
        pub quote_id: u32,
        pub size_increment: i128,
        pub min_size: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `submitUpdateProductTx` function with signature `submitUpdateProductTx(bytes)` and selector `0xd4a5ea85`
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
    #[ethcall(name = "submitUpdateProductTx", abi = "submitUpdateProductTx(bytes)")]
    pub struct SubmitUpdateProductTxCall {
        pub slow_mode_tx: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `updateProducts` function with signature `updateProducts()` and selector `0x3d141d38`
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
    #[ethcall(name = "updateProducts", abi = "updateProducts()")]
    pub struct UpdateProductsCall;
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
        AddProducts(AddProductsCall),
        AssignPubKey(AssignPubKeyCall),
        BatchSubmitUpdateProductTxs(BatchSubmitUpdateProductTxsCall),
        ClearPerpAddProductCalls(ClearPerpAddProductCallsCall),
        ClearSpotAddProductCalls(ClearSpotAddProductCallsCall),
        ClearUpdateProductTxs(ClearUpdateProductTxsCall),
        CreateDirectDepositV1(CreateDirectDepositV1Call),
        CreditDepositV1(CreditDepositV1Call),
        DeletePubkey(DeletePubkeyCall),
        DelistProduct(DelistProductCall),
        DirectDepositV1Address(DirectDepositV1AddressCall),
        DumpFees(DumpFeesCall),
        HasPendingAddProductCalls(HasPendingAddProductCallsCall),
        HasPendingUpdateProductTxs(HasPendingUpdateProductTxsCall),
        Initialize(InitializeCall),
        IsDirectDepositV1Ready(IsDirectDepositV1ReadyCall),
        Owner(OwnerCall),
        PendingPerpAddProductIds(PendingPerpAddProductIdsCall),
        PendingSpotAddProductIds(PendingSpotAddProductIdsCall),
        PerpUpdateRisk(PerpUpdateRiskCall),
        RebalanceXWithdraw(RebalanceXWithdrawCall),
        RemoveWithdrawPoolLiquidity(RemoveWithdrawPoolLiquidityCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetWithdrawPool(SetWithdrawPoolCall),
        SpotUpdateRisk(SpotUpdateRiskCall),
        SubmitPerpAddProductCall(SubmitPerpAddProductCallCall),
        SubmitSpotAddProductCall(SubmitSpotAddProductCallCall),
        SubmitUpdateProductTx(SubmitUpdateProductTxCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateProducts(UpdateProductsCall),
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
            if let Ok(decoded) = <AddProductsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddProducts(decoded));
            }
            if let Ok(decoded) = <AssignPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignPubKey(decoded));
            }
            if let Ok(decoded) =
                <BatchSubmitUpdateProductTxsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BatchSubmitUpdateProductTxs(decoded));
            }
            if let Ok(decoded) =
                <ClearPerpAddProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClearPerpAddProductCalls(decoded));
            }
            if let Ok(decoded) =
                <ClearSpotAddProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClearSpotAddProductCalls(decoded));
            }
            if let Ok(decoded) =
                <ClearUpdateProductTxsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClearUpdateProductTxs(decoded));
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
                <HasPendingAddProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasPendingAddProductCalls(decoded));
            }
            if let Ok(decoded) =
                <HasPendingUpdateProductTxsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasPendingUpdateProductTxs(decoded));
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
                <PendingPerpAddProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingPerpAddProductIds(decoded));
            }
            if let Ok(decoded) =
                <PendingSpotAddProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingSpotAddProductIds(decoded));
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
                <SubmitPerpAddProductCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitPerpAddProductCall(decoded));
            }
            if let Ok(decoded) =
                <SubmitSpotAddProductCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitSpotAddProductCall(decoded));
            }
            if let Ok(decoded) =
                <SubmitUpdateProductTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitUpdateProductTx(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateProducts(decoded));
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
                Self::AddProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssignPubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BatchSubmitUpdateProductTxs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClearPerpAddProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClearSpotAddProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClearUpdateProductTxs(element) => {
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
                Self::HasPendingAddProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasPendingUpdateProductTxs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDirectDepositV1Ready(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingPerpAddProductIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingSpotAddProductIds(element) => {
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
                Self::SetWithdrawPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SpotUpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitPerpAddProductCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitSpotAddProductCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitUpdateProductTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::AddProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchSubmitUpdateProductTxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearPerpAddProductCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearSpotAddProductCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearUpdateProductTxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateDirectDepositV1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditDepositV1(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeletePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelistProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::DirectDepositV1Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasPendingAddProductCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasPendingUpdateProductTxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDirectDepositV1Ready(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingPerpAddProductIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingSpotAddProductIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpUpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceXWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveWithdrawPoolLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotUpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitPerpAddProductCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitSpotAddProductCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitUpdateProductTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProducts(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AddProductsCall> for ContractOwnerCalls {
        fn from(value: AddProductsCall) -> Self {
            Self::AddProducts(value)
        }
    }
    impl ::core::convert::From<AssignPubKeyCall> for ContractOwnerCalls {
        fn from(value: AssignPubKeyCall) -> Self {
            Self::AssignPubKey(value)
        }
    }
    impl ::core::convert::From<BatchSubmitUpdateProductTxsCall> for ContractOwnerCalls {
        fn from(value: BatchSubmitUpdateProductTxsCall) -> Self {
            Self::BatchSubmitUpdateProductTxs(value)
        }
    }
    impl ::core::convert::From<ClearPerpAddProductCallsCall> for ContractOwnerCalls {
        fn from(value: ClearPerpAddProductCallsCall) -> Self {
            Self::ClearPerpAddProductCalls(value)
        }
    }
    impl ::core::convert::From<ClearSpotAddProductCallsCall> for ContractOwnerCalls {
        fn from(value: ClearSpotAddProductCallsCall) -> Self {
            Self::ClearSpotAddProductCalls(value)
        }
    }
    impl ::core::convert::From<ClearUpdateProductTxsCall> for ContractOwnerCalls {
        fn from(value: ClearUpdateProductTxsCall) -> Self {
            Self::ClearUpdateProductTxs(value)
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
    impl ::core::convert::From<HasPendingAddProductCallsCall> for ContractOwnerCalls {
        fn from(value: HasPendingAddProductCallsCall) -> Self {
            Self::HasPendingAddProductCalls(value)
        }
    }
    impl ::core::convert::From<HasPendingUpdateProductTxsCall> for ContractOwnerCalls {
        fn from(value: HasPendingUpdateProductTxsCall) -> Self {
            Self::HasPendingUpdateProductTxs(value)
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
    impl ::core::convert::From<PendingPerpAddProductIdsCall> for ContractOwnerCalls {
        fn from(value: PendingPerpAddProductIdsCall) -> Self {
            Self::PendingPerpAddProductIds(value)
        }
    }
    impl ::core::convert::From<PendingSpotAddProductIdsCall> for ContractOwnerCalls {
        fn from(value: PendingSpotAddProductIdsCall) -> Self {
            Self::PendingSpotAddProductIds(value)
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
    impl ::core::convert::From<SubmitPerpAddProductCallCall> for ContractOwnerCalls {
        fn from(value: SubmitPerpAddProductCallCall) -> Self {
            Self::SubmitPerpAddProductCall(value)
        }
    }
    impl ::core::convert::From<SubmitSpotAddProductCallCall> for ContractOwnerCalls {
        fn from(value: SubmitSpotAddProductCallCall) -> Self {
            Self::SubmitSpotAddProductCall(value)
        }
    }
    impl ::core::convert::From<SubmitUpdateProductTxCall> for ContractOwnerCalls {
        fn from(value: SubmitUpdateProductTxCall) -> Self {
            Self::SubmitUpdateProductTx(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ContractOwnerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateProductsCall> for ContractOwnerCalls {
        fn from(value: UpdateProductsCall) -> Self {
            Self::UpdateProducts(value)
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
    ///Container type for all return fields from the `hasPendingAddProductCalls` function with signature `hasPendingAddProductCalls()` and selector `0x0577559f`
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
    pub struct HasPendingAddProductCallsReturn(pub bool);
    ///Container type for all return fields from the `hasPendingUpdateProductTxs` function with signature `hasPendingUpdateProductTxs()` and selector `0xfb1cb049`
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
    pub struct HasPendingUpdateProductTxsReturn(pub bool);
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
    ///Container type for all return fields from the `pendingPerpAddProductIds` function with signature `pendingPerpAddProductIds()` and selector `0x788d4248`
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
    pub struct PendingPerpAddProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `pendingSpotAddProductIds` function with signature `pendingSpotAddProductIds()` and selector `0x50e82790`
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
    pub struct PendingSpotAddProductIdsReturn(pub ::std::vec::Vec<u32>);
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
