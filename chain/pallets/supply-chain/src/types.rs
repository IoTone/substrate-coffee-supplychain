use codec::{Decode, Encode};
use core::fmt;
use fixed::types::I16F16;
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*}; 
// Custom types
pub type Identifier = Vec<u8>;
pub type Decimal = I16F16;
pub type SupplyProcessId = Identifier;
pub type AttributeValue = Vec<u8>;
pub type RawMaterialId = Identifier;
pub type Amount = Decimal;
pub type Certifications = Vec<Vec<u8>>;
 


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]

pub enum AttributeName {
    Location,
    Methodology,
    PackagingType,
    FreightMethod,
    RoastingMethod,
    GrindingMethod,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]

pub enum ProcessType {
    Harvesing,
    Processing,
    Packaging,
    Transporting,
    Roasting,
    Grinding,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct SupplyProcess<Moment> {
    pub id: SupplyProcessId,
    pub attribute: ProcessAttribute,
    pub certifications: Certifications,
    pub date: Moment,
    pub amount: Amount,
    pub archived:bool,
    pub inputAmount: Amount,
    pub processType: ProcessType,
    pub rawMaterialId: RawMaterialId,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct ProcessAttribute {
    pub name: AttributeName,
    pub value: AttributeValue,
}

impl<Moment> SupplyProcess<Moment> {
    pub fn new(
        id:SupplyProcessId,
        attribute: ProcessAttribute,
        certifications: Certifications,
        date: Moment,
        amount: Amount,
        inputAmount: Amount,
        processType: ProcessType,
        rawMaterialId: RawMaterialId,
        
    ) -> Self {
     
        Self {
            id , 
            amount,
            attribute,
            certifications,
            date,
            inputAmount,
            processType,
            rawMaterialId,
            archived:false
        }
    }
}
