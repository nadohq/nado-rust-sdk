pub mod base;
pub mod builder;
pub mod execute;
pub mod indexer;
pub mod query;

pub use base::NadoBase;
pub use builder::NadoBuilder;
pub use execute::NadoExecute;
pub use indexer::NadoIndexer;
pub use query::NadoQuery;

#[doc(hidden)]
#[macro_export]
macro_rules! map_response {
    ($response:expr, $expected_type:path) => {
        match $response {
            $expected_type(data) => Ok(data),
            _ => Err(eyre!(concat!(
                "expected ",
                stringify!($expected_type),
                " response"
            ))),
        }
    };
}
