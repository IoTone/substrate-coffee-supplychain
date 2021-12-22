use codec::{Decode, Encode};
use core::fmt;
use fixed::types::I16F16;
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*};
// Custom types
pub type Identifier = Vec<u8>;
pub type Quantity = I16F16;
pub type Certifications = Vec<Vec<u8>>; 
pub type ProductId = Identifier;
pub type Decimal = I16F16;
pub type Amount = Decimal;
pub type SKU = Vec<u8>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]

pub enum Kind {
    Bag,
    BagRoasted,
    Grinded,
    Whole,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]

pub enum Status {
    Sold,
    Available,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct Product<Moment> {
    pub product_id: ProductId,
    pub date: Moment,
    pub kind: Kind,
    pub SKU: SKU,
    pub lb: Decimal,
    pub status: Status,
}

impl<Moment> Product<Moment> {
    pub fn new(id:ProductId,date: Moment, kind: Kind, SKU: SKU, lb: Decimal ) -> Self {
       
        Self {
            product_id: id ,
            SKU,
            date,
            kind,
            lb,
            status:Status::Available,
        }
    }

    pub fn sell(mut self ) -> Self {
        self.status = Status::Sold;
        self
    }
}
