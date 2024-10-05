use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Copy, Serialize, Deserialize, Display, EnumString)]
pub enum Result {
    Success,
    DoesntExist,
    Exists,

    TableOccupied,
    TableUnoccupied,

    VariantDoesntExist,
    SizeDoesntExist,

    NoPermission,
    NoSession
}
