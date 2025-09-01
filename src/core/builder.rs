use crate::builders::execute::burn_nlp::BurnNlpBuilder;
use crate::builders::execute::cancellation::CancellationBuilder;
use crate::builders::execute::cancellation_products::CancellationProductsBuilder;
use crate::builders::execute::deposit_collateral::DepositCollateralBuilder;
use crate::builders::execute::deposit_insurance::DepositInsuranceBuilder;
use crate::builders::execute::link_signer::LinkSignerBuilder;
use crate::builders::execute::liquidate_subaccount::LiquidateSubaccountBuilder;
use crate::builders::execute::mint_nlp::MintNlpBuilder;
use crate::builders::execute::place_order::PlaceOrderBuilder;
use crate::builders::execute::slow_mode::SubmitSlowModeTxBuilder;
use crate::builders::execute::transfer_quote::TransferQuoteBuilder;
use crate::builders::execute::withdraw_collateral::WithdrawCollateralBuilder;
use crate::builders::indexer::account_snapshots::AccountSnapshotsBuilder;
use crate::builders::indexer::candlesticks::CandlesticksBuilder;
use crate::builders::indexer::events::EventsBuilder;
use crate::builders::indexer::foundation_taker_rewards::FoundationTakerRewardsBuilder;
use crate::builders::indexer::historical_orders::HistoricalOrdersBuilder;
use crate::builders::indexer::interest_and_funding::InterestAndFundingTicksBuilder;
use crate::builders::indexer::leaderboard::LeaderboardBuilder;
use crate::builders::indexer::maker_statistics::MakerStatisticsBuilder;
use crate::builders::indexer::market_snapshots::MarketSnapshotsBuilder;
use crate::builders::indexer::matches::MatchesBuilder;
use crate::builders::indexer::multi_product_snapshots::MultiProductSnapshotsBuilder;
use crate::builders::indexer::product_snapshots::ProductSnapshotsBuilder;
use crate::builders::indexer::rewards::RewardsBuilder;
use crate::builders::indexer::subaccounts::SubaccountsBuilder;
use crate::builders::indexer::trades::TradesParamsBuilder;
use crate::builders::query::max_nlp_mintable::MaxNlpMintableBuilder;
use crate::builders::query::max_order_size::MaxOrderSizeBuilder;
use crate::builders::query::max_withdrawable::MaxWithdrawableBuilder;
use crate::builders::query::nlp_pool_info::NlpPoolInfoBuilder;
use crate::builders::trigger::list_trigger_orders::ListTriggerOrdersBuilder;
use crate::builders::utils::fee_calculator::FeeCalculator;

use crate::core::execute::NadoExecute;
use crate::core::indexer::NadoIndexer;

#[doc(hidden)]
#[macro_export]
macro_rules! get_nado_builder {
    (pub $method_name:ident, $builder_name:ident) => {
        pub fn $method_name(&self) -> $builder_name<Self> {
            $builder_name::new(&self)
        }
    };
    ($method_name:ident, $builder_name:ident) => {
        fn $method_name(&self) -> $builder_name<Self> {
            $builder_name::new(&self)
        }
    };
}

pub trait NadoBuilder: NadoExecute + NadoIndexer {
    get_nado_builder!(deposit_collateral_builder, DepositCollateralBuilder);
    get_nado_builder!(place_order_builder, PlaceOrderBuilder);
    get_nado_builder!(list_trigger_orders_builder, ListTriggerOrdersBuilder);
    get_nado_builder!(cancellation_builder, CancellationBuilder);
    get_nado_builder!(cancellation_products_builder, CancellationProductsBuilder);
    get_nado_builder!(liquidate_subaccount_builder, LiquidateSubaccountBuilder);
    get_nado_builder!(withdraw_collateral_builder, WithdrawCollateralBuilder);
    get_nado_builder!(mint_nlp_builder, MintNlpBuilder);
    get_nado_builder!(burn_nlp_builder, BurnNlpBuilder);
    get_nado_builder!(link_signer_builder, LinkSignerBuilder);
    get_nado_builder!(transfer_quote_builder, TransferQuoteBuilder);
    get_nado_builder!(submit_slow_mode_tx_builder, SubmitSlowModeTxBuilder);
    get_nado_builder!(deposit_insurance_builder, DepositInsuranceBuilder);

    get_nado_builder!(get_max_order_size_builder, MaxOrderSizeBuilder);
    get_nado_builder!(get_max_withdrawable_builder, MaxWithdrawableBuilder);
    get_nado_builder!(get_max_nlp_mintable_builder, MaxNlpMintableBuilder);
    get_nado_builder!(get_nlp_pool_info_builder, NlpPoolInfoBuilder);

    get_nado_builder!(get_candlesticks_builder, CandlesticksBuilder);
    get_nado_builder!(get_historical_orders_builder, HistoricalOrdersBuilder);
    get_nado_builder!(get_events_builder, EventsBuilder);
    get_nado_builder!(get_maker_statistics_builder, MakerStatisticsBuilder);
    get_nado_builder!(get_matches_builder, MatchesBuilder);
    get_nado_builder!(get_product_snapshots_builder, ProductSnapshotsBuilder);
    get_nado_builder!(
        get_multi_product_snapshots_builder,
        MultiProductSnapshotsBuilder
    );
    get_nado_builder!(get_account_snapshots_builder, AccountSnapshotsBuilder);
    get_nado_builder!(get_subaccounts_builder, SubaccountsBuilder);
    get_nado_builder!(
        get_interest_and_funding_builder,
        InterestAndFundingTicksBuilder
    );
    get_nado_builder!(get_market_snapshots_builder, MarketSnapshotsBuilder);
    get_nado_builder!(get_trades_builder, TradesParamsBuilder);
    get_nado_builder!(fee_calculator, FeeCalculator);
    get_nado_builder!(get_leaderboard_builder, LeaderboardBuilder);
    get_nado_builder!(get_rewards_builder, RewardsBuilder);
    get_nado_builder!(
        get_foundation_taker_rewards_builder,
        FoundationTakerRewardsBuilder
    );
}
