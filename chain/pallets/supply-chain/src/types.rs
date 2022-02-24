use codec::{Decode, Encode};
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
    Harvesting,
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
    pub input_amount: Amount,
    pub process_type: ProcessType,
    pub raw_material_id: RawMaterialId,
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
        input_amount: Amount,
        process_type: ProcessType,
        raw_material_id: RawMaterialId,
        
    ) -> Self {
     
        Self {
            id , 
            amount,
            attribute,
            certifications,
            date,
            input_amount,
            process_type,
            raw_material_id,
            archived:false
        }
    }
}
