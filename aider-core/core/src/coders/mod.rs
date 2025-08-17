pub mod search_replace;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("search text not found")]
    NotFound,
}
