export default {
  "Address": "MultiAddress",
  "LookupSource": "MultiAddress",
  "Signature": "Vec<u8>",
  "Attribute": {
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "BlockNumber",
    "creation": "Moment",
    "nonce": "u64"
  },
  "ProcessAttribute": {
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "BlockNumber",
    "creation": "Moment",
    "nonce": "u64"
  },
  "AttributeTransaction": {
    "signature": "Signature",
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "u32",
    "signer": "AccountId",
    "identity": "AccountId"
  },
  "PropName": "Vec<u8>",
  "PropValue": "Vec<u8>",

  "Product": {
    "product_id": "ProductId",
    "kind": "Kind",
    "SKU": "SKU",
    "lb": "Decimal",
    "date": "Moment",
    "status": "Status",
  },
  "Identifier": "Vec<u8>",

  "Decimal": "f32",
  "ShipmentId": "Identifier",

  "ProductId": "Identifier",
  "Id": "Identifier",
  "ShippingEventIndex": "u128",
  "DeviceId": "Identifier",
  "SaleId": "Identifier",
  "Cost": "Decimal",
  "SerialNumber": "Vec<u8>",
  "SKU": "Vec<u8>",
  "AttributeValue": "Vec<u8>",
  "Quantity": "Decimal",
  "Amount": "Decimal",
  "AmountForProducts": "Decimal",
  "AmountOfProducts": "u128",
  "RawMaterialId": "Identifier",
  "OriginProcess": "Identifier",
  "SupplyProcessId": "Identifier",

  "Certifications": "Vec<Vec<u8>>",
  "CurrencyType": "Vec<u8>",
  "Brand": "Vec<u8>",
  "Kind": {
    "_emun": [
      "Bag",
      "BagRoasted",
      "Grinded",
      "Whole",
    ]
  },
  "Status": {
    "_enum": [
      "Sold",
      "Available",
    ]
  },
  "ProcessType": {
    "_enum": [
      "Harvesing",
      "Processing",
      "Packaging",
      "Transporting",
      "Roasting",
      "Grinding",
    ]
  },
  "AttributeName": {
    "_enum": [
      "Location",
      "Methodology",
      "PackagingType",
      "FreightMethod",
      "RoastingMethod",
      "GrindingMethod",
    ]
  },
  "State": {
    "_enum": [
      "Roasted",
      "UnRoasted",
    ]
  },
  "ShipmentStatus": {
    "_enum": [
      "Pending",
      "InTransit",
      "Delivered"
    ]
  },
  "Attribute": {
    "name": "AttributeName",
    "value": "AttributeValue",
  }
  ,
  "SupplyProcess": {
    "id": "SupplyProcessId",
    "attribute": "Attribute",
    "certifications": "Certifications",
    "date": "Moment",
    "amount": "Amount",
    "inputAmount": "Amount",
    "processType": "ProcessType",
    "rawMaterialId": "RawMaterialId",
  },
  "RawMaterial": {
    "id": "RawMaterialId",
    "origin_process": "Option<OriginProcess>",
    "date": "Moment",
    "state": "State",
    "amount": "Amount",
    "remaining_amount": "Amount"
  },
  "Sale": {
    "id": "Id",
    "date": "Moment",
    "currency_type": "CurrencyType",
    "cost": "Cost",
    "quantity": "Quantity",
    "sku": "SKU",
    "serial_number": "SerialNumber",
    "product_id": "ProductId",
    "user": "AccountId",
    "buyer": "AccountId",
  }
  ,
  "RetailPackaging": {
    "id": "Id",
    "date": "Moment",
    "certifications": "Certifications",
    "amount": "Quantity",
    "amount_of_products": "AmountOfProducts",
    "amount_for_products": "AmountForProducts",
    "sku": "SKU",
    "serial_number": "SerialNumber",
    "brand": "Brand",
    "origin_process": "OriginProcess",
  },
  "ShippingOperation": {
    "_enum": [
      "Pickup",
      "Scan",
      "Deliver"
    ]
  },
  "ShippingEventType": {
    "_enum": [
      "ShipmentRegistration",
      "ShipmentPickup",
      "ShipmentScan",
      "ShipmentDeliver"
    ]
  },
  "ShippingEvent": {
    "event_type": "ShippingEventType",
    "shipment_id": "ShipmentId",
    "location": "Option<ReadPoint>",
    "readings": "Vec<Reading<Moment>>",
    "timestamp": "Moment"
  },
  "ReadPoint": {
    "latitude": "Decimal",
    "longitude": "Decimal"
  },
  "ReadingType": {
    "_enum": [
      "Humidity",
      "Pressure",
      "Shock",
      "Tilt",
      "Temperature",
      "Vibration"
    ]
  },
  "Reading": {
    "device_id": "DeviceId",
    "reading_type": "ReadingType",
    "timestamp": "Moment",
    "value": "Decimal"
  },
  "Permission": {
    "_enum": { "Execute": 1, "Manage": 2 }
  },
  "Role": {
    "pallet": "Vec<u8>",
    "permission": "Permission"
  },
  "Sale": {
    "id": "SaleId",
    "date": "Moment",
    "currency_type": "CurrencyType",
    "cost": "Cost",
    "quantity": "Quantity",
    "sku": "SKU",
    "serial_number": "SerialNumber",
    "user": "AccountId",
    "buyer": "AccountId",
  },
  "Keys": "SessionKeys2"
};
