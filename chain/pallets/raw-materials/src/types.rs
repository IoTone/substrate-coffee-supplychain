use codec::{Decode, Encode};
use core::fmt;
use fixed::types::I16F16;
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*}; 
pub type Identifier = Vec<u8>;
pub type Quantity = I16F16;
pub type Certifications = Vec<Vec<u8>>;
pub type RawMaterialId = Identifier;
pub type OriginProcess = Identifier;
pub type Decimal = I16F16;
pub type Amount = Decimal;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]

pub enum State {
    Roasted,
    UnRoasted,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct RawMaterial<Moment> {
    pub id: RawMaterialId,
    pub origin_process: Option<OriginProcess>,
    pub date: Moment,
    pub state: State,
    pub amount: Amount,
    pub remaining_amount: Amount,
}

impl<Moment> RawMaterial<Moment> {
    pub fn new(
        id:RawMaterialId,
        amount: Amount,
        origin_process: Option<OriginProcess>,
        state: State,
        date: Moment,
    ) -> Self {
         
        Self {
            id ,
            remaining_amount: amount.clone(),
            amount,
            origin_process,
            state,
            date,
        }
    }

    pub fn subtract_amount(mut self, amount: Amount) -> Self {
        self.remaining_amount = self.remaining_amount - amount;
        self
    }
}
