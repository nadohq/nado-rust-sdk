use crate::math::str_to_x18;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ProductRaw {
    #[serde(rename_all = "camelCase")]
    Spot {
        product_id: u32,
        symbol: String,
        name: String,
        quote: String,
        long_weight_initial: String,
        long_weight_maintenance: String,
        short_weight_initial: String,
        short_weight_maintenance: String,
        size_increment: String,
        price_increment: String,
        interest_inflection_util: String,
        interest_floor: String,
        interest_small_cap: String,
        interest_large_cap: String,
        initial_price: String,
        address: String,
        decimals: u8,
        price_asset_id: String,
        min_size: String,
        feed: Option<String>,
        feed_id: Option<String>,
        withdraw_fee: String,
        min_deposit_rate: String,
        multiplier: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Perp {
        product_id: u32,
        symbol: String,
        name: String,
        quote: String,
        long_weight_initial: String,
        long_weight_maintenance: String,
        short_weight_initial: String,
        short_weight_maintenance: String,
        size_increment: String,
        price_increment: String,
        initial_price: String,
        price_asset_id: String,
        spot_index_asset_id: String,
        min_size: String,
        // "0" for no limit
        max_open_interest: Option<String>,
        feed: Option<String>,
        feed_id: Option<String>,
        multiplier: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub enum Product {
    Spot {
        product_id: u32,
        symbol: String,
        name: String,
        quote: String,
        long_weight_initial_x18: i128,
        long_weight_maintenance_x18: i128,
        short_weight_initial_x18: i128,
        short_weight_maintenance_x18: i128,
        size_increment_x18: i128,
        price_increment_x18: i128,
        interest_inflection_util_x18: i128,
        interest_floor_x18: i128,
        interest_small_cap_x18: i128,
        interest_large_cap_x18: i128,
        initial_price_x18: i128,
        address: String,
        decimals: u8,
        price_asset_id: String,
        min_size_x18: i128,
        feed: Option<String>,
        feed_id: Option<String>,
        withdraw_fee_x18: i128,
        min_deposit_rate_x18: i128,
        multiplier_x18: Option<i128>,
    },
    Perp {
        product_id: u32,
        symbol: String,
        name: String,
        quote: String,
        long_weight_initial_x18: i128,
        long_weight_maintenance_x18: i128,
        short_weight_initial_x18: i128,
        short_weight_maintenance_x18: i128,
        size_increment_x18: i128,
        price_increment_x18: i128,
        initial_price_x18: i128,
        price_asset_id: String,
        spot_index_asset_id: String,
        min_size_x18: i128,
        max_open_interest_x18: Option<i128>,
        feed: Option<String>,
        feed_id: Option<String>,
        multiplier_x18: Option<i128>,
    },
}

impl From<ProductRaw> for Product {
    fn from(raw: ProductRaw) -> Self {
        match raw {
            ProductRaw::Spot {
                product_id,
                symbol,
                name,
                quote,
                long_weight_initial,
                long_weight_maintenance,
                short_weight_initial,
                short_weight_maintenance,
                size_increment,
                price_increment,
                interest_inflection_util,
                interest_floor,
                interest_small_cap,
                interest_large_cap,
                initial_price,
                address,
                decimals,
                price_asset_id,
                min_size,
                feed,
                feed_id,
                withdraw_fee,
                min_deposit_rate,
                multiplier,
            } => Product::Spot {
                product_id,
                symbol,
                name,
                quote,
                long_weight_initial_x18: str_to_x18(&long_weight_initial),
                long_weight_maintenance_x18: str_to_x18(&long_weight_maintenance),
                short_weight_initial_x18: str_to_x18(&short_weight_initial),
                short_weight_maintenance_x18: str_to_x18(&short_weight_maintenance),
                size_increment_x18: str_to_x18(&size_increment),
                price_increment_x18: str_to_x18(&price_increment),
                interest_inflection_util_x18: str_to_x18(&interest_inflection_util),
                interest_floor_x18: str_to_x18(&interest_floor),
                interest_small_cap_x18: str_to_x18(&interest_small_cap),
                interest_large_cap_x18: str_to_x18(&interest_large_cap),
                initial_price_x18: str_to_x18(&initial_price),
                address,
                decimals,
                price_asset_id,
                min_size_x18: str_to_x18(&min_size),
                feed,
                feed_id,
                withdraw_fee_x18: str_to_x18(&withdraw_fee),
                min_deposit_rate_x18: str_to_x18(&min_deposit_rate),
                multiplier_x18: multiplier.map(|s| str_to_x18(&s)),
            },
            ProductRaw::Perp {
                product_id,
                symbol,
                name,
                quote,
                long_weight_initial,
                long_weight_maintenance,
                short_weight_initial,
                short_weight_maintenance,
                size_increment,
                price_increment,
                initial_price,
                price_asset_id,
                spot_index_asset_id,
                min_size,
                max_open_interest,
                feed,
                feed_id,
                multiplier,
            } => Product::Perp {
                product_id,
                symbol,
                name,
                quote,
                long_weight_initial_x18: str_to_x18(&long_weight_initial),
                long_weight_maintenance_x18: str_to_x18(&long_weight_maintenance),
                short_weight_initial_x18: str_to_x18(&short_weight_initial),
                short_weight_maintenance_x18: str_to_x18(&short_weight_maintenance),
                size_increment_x18: str_to_x18(&size_increment),
                price_increment_x18: str_to_x18(&price_increment),
                initial_price_x18: str_to_x18(&initial_price),
                price_asset_id,
                spot_index_asset_id,
                min_size_x18: str_to_x18(&min_size),
                max_open_interest_x18: max_open_interest.map(|s| str_to_x18(&s)),
                feed,
                feed_id,
                multiplier_x18: multiplier.map(|s| str_to_x18(&s)),
            },
        }
    }
}

impl Product {
    pub fn id(&self) -> u32 {
        match self {
            Product::Spot { product_id, .. } | Product::Perp { product_id, .. } => *product_id,
        }
    }

    pub fn min_size(&self) -> i128 {
        match self {
            Product::Spot { min_size_x18, .. } | Product::Perp { min_size_x18, .. } => {
                *min_size_x18
            }
        }
    }

    pub fn size_increment(&self) -> i128 {
        match self {
            Product::Spot {
                size_increment_x18, ..
            }
            | Product::Perp {
                size_increment_x18, ..
            } => *size_increment_x18,
        }
    }

    pub fn long_weight_initial(&self) -> i128 {
        match self {
            Product::Spot {
                long_weight_initial_x18,
                ..
            }
            | Product::Perp {
                long_weight_initial_x18,
                ..
            } => *long_weight_initial_x18,
        }
    }

    pub fn long_weight_maintenance(&self) -> i128 {
        match self {
            Product::Spot {
                long_weight_maintenance_x18,
                ..
            }
            | Product::Perp {
                long_weight_maintenance_x18,
                ..
            } => *long_weight_maintenance_x18,
        }
    }

    pub fn short_weight_initial(&self) -> i128 {
        match self {
            Product::Spot {
                short_weight_initial_x18,
                ..
            }
            | Product::Perp {
                short_weight_initial_x18,
                ..
            } => *short_weight_initial_x18,
        }
    }

    pub fn short_weight_maintenance(&self) -> i128 {
        match self {
            Product::Spot {
                short_weight_maintenance_x18,
                ..
            }
            | Product::Perp {
                short_weight_maintenance_x18,
                ..
            } => *short_weight_maintenance_x18,
        }
    }

    pub fn is_spot(&self) -> bool {
        matches!(self, Product::Spot { .. })
    }

    pub fn quote(&self) -> String {
        match self {
            Product::Spot { quote, .. } | Product::Perp { quote, .. } => quote.clone(),
        }
    }

    pub fn feed_id(&self) -> Option<String> {
        match self {
            Product::Spot { feed_id, .. } | Product::Perp { feed_id, .. } => feed_id.clone(),
        }
    }

    pub fn feed(&self) -> Option<String> {
        match self {
            Product::Spot { feed, .. } | Product::Perp { feed, .. } => feed.clone(),
        }
    }

    pub fn asset_id(&self) -> String {
        match self {
            Product::Spot { price_asset_id, .. } | Product::Perp { price_asset_id, .. } => {
                price_asset_id.clone()
            }
        }
    }

    pub fn multiplier(&self) -> Option<i128> {
        match self {
            Product::Spot { multiplier_x18, .. } | Product::Perp { multiplier_x18, .. } => {
                *multiplier_x18
            }
        }
    }

    pub fn max_open_interest(&self) -> Option<i128> {
        match self {
            Product::Perp {
                max_open_interest_x18,
                ..
            } => *max_open_interest_x18,
            _ => None,
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Product::Spot { symbol, .. } | Product::Perp { symbol, .. } => symbol.clone(),
        }
    }
}
