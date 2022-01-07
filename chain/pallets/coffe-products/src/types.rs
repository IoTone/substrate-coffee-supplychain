use codec::{Decode, Encode};
use core::fmt;
use fixed::types::{I16F16, U16F0};
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
pub struct Product<Moment> {
    pub product_id: ProductId,
    pub date: Moment,
    pub kind: Kind,
    pub sku: SKU,
    pub lb: Decimal,
    pub amount: Decimal,
    pub remaining_amount: Decimal,
    pub price:Decimal,
    pub packaging_id:Identifier
}

impl<Moment> Product<Moment> {
    pub fn new(
        id: ProductId,
        date: Moment,
        kind: Kind,
        sku: SKU,
        lb: Decimal,
        amount: Decimal,
        price:Decimal,
        packaging_id:Identifier
    ) -> Self {
        Self {
            product_id: id,
            sku,
            date,
            kind,
            lb,
            remaining_amount: amount.clone(),
            amount,
            price,
            packaging_id
        }
    }

    pub fn sell(mut self,amount:Decimal) -> Self {
        self.remaining_amount = self.remaining_amount-amount;
        self
    }
}
