use codec::{Decode, Encode};
use core::fmt;
use fixed::types::I16F16;
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*};

// Custom types
pub type Identifier = Vec<u8>;
pub type Decimal = I16F16;

pub type CurrencyType = Vec<u8>;
pub type Quantity = Decimal;
pub type SKU = Vec<u8>;
pub type SerialNumber = Vec<u8>;
pub type Certifications = Vec<u8>;
pub type Brand = Vec<u8>;
pub type Cost = Decimal;
pub type AmountOfProducts = u128;
pub type AmountForProducts = Decimal;
pub type Id = Identifier;
pub type OriginProcess = Identifier;
pub type ProductId = Identifier;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]

pub struct Sale<AccountId, Moment> {
    pub id: Id,
    pub date: Moment,
    pub currency_type: CurrencyType,
    pub cost: Cost,
    pub quantity: Quantity,
    pub sku: SKU,
    pub serial_number: SerialNumber,
    pub product_id: ProductId,
    pub user: AccountId,
    pub buyer: AccountId,
}

pub struct RetailPackaging<Moment> {
    pub id: Id,
    pub date: Moment,
    pub certifications: Certifications,
    pub amount: Quantity,
    pub amount_of_products: AmountOfProducts,
    pub amount_for_products: AmountForProducts,
    pub sku: SKU,
    pub serial_number: SerialNumber,
    pub brand: Brand,
    pub origin_process: OriginProcess,
}

impl<Moment> RetailPackaging<Moment> {
    pub fn new(
        id:Id,
        date: Moment,
        certifications: Certifications,
        amount: Quantity,
        amount_of_products: AmountOfProducts,
        amount_for_products: AmountForProducts,
        sku: SKU,
        serial_number: SerialNumber,
        brand: Brand,
        origin_process: OriginProcess,
    ) -> Self {
     
        Self {
            id,
            amount,
            date,
            certifications,
            brand,
            origin_process,
            serial_number,
            sku,
            amount_of_products,
            amount_for_products,
        }
    }
}

impl<AccountId, Moment> Sale<AccountId, Moment> {
    pub fn new(
        id:Id,
        date: Moment,
        currency_type: CurrencyType,
        cost: Cost,
        quantity: Quantity,
        sku: SKU,
        serial_number: SerialNumber,
        product_id: ProductId,
        user: AccountId,
        buyer: AccountId,
    ) -> Self {
       
        Self {
            id,
            buyer,
            user,
            date,
            product_id,
            cost,
            quantity,
            sku,
            currency_type,
            serial_number,
        }
    }
}
