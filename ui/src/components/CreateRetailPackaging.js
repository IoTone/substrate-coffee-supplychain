import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToString, hexToU8a, stringToHex, u8aToString } from '@polkadot/util';
import ReactTagInput from "@pathofdev/react-tag-input";
import "@pathofdev/react-tag-input/build/index.css";

export default function Main(props) {
  const processesTypes = ["Harvesting",
    "Processing",
    "Packaging",
    "Transporting",
    "Roasting",
    "Grinding",]
  const [status, setStatus] = useState(null);
  const [processes, setProcesses] = useState([]);
  const [tags, setTags] = useState([]);
  const [formState, setFormState] = useState({ certifiations: null, amount: null, amount_of_products: null, price_for_products: null, amount_for_products: null, sku: null, serial_number: null, brand: null, origin_process: null, kind: null });
  const { accountPair, organization } = props;

  const onChange = (_, data) => {
    if ((data.state === "amount" || data.state === "amount_of_products" || data.state === "amount_for_products" || data.state === "price_for_products") ) {
      if(!data.value.match(/^[0-9]*$/))
      return
      if ( parseFloat(data.value) <= 0)
      return
    }
    setFormState(prev => ({ ...prev, [data.state]: data.value }));
  }
  const { certifiations, price_for_products, amount, amount_of_products, amount_for_products, sku, serial_number, brand, origin_process, kind } = formState;
  useEffect(() => {


    api.query.supplyChain.processesOfOrganization(organization, processIds => {
      console.log({ processIds });

      api.query.supplyChain.processes.multi(processIds, process => {
        let validProcesses = process
          .filter(p => !p.isNone)
          .map(p => p.unwrap());

        api.query.rawMaterials.rawMaterialsOfOrganization(organization, async rawMaterialsId => {
          api.query.rawMaterials.rawMaterials.multi(rawMaterialsId, async rawMaterials => {
            let validRawMaterials = rawMaterials
              .filter(material => !material.isNone).map(m => m.unwrap());

            const retailPackagingsIds = await api.query.retailTransaction.retailPackagingsByOrg(organization)
            const retailPackagings = await api.query.retailTransaction.retailPackagings.multi(retailPackagingsIds)
            let validateRetailPackagings = retailPackagings
              .filter(packaging => !packaging.isNone).map(m => m.unwrap());
            validProcesses = validProcesses.filter(vp => (!validRawMaterials.find(vrm => u8aToString(hexToU8a(vrm.origin_process.toString())) === u8aToString(vp.id)) && !validateRetailPackagings.find(vrm => u8aToString(hexToU8a(vrm.origin_process.toString())) === u8aToString(vp.id))))
            const convertedProcesses = validProcesses.map(p => ({ text: p.processType.toString() + " Amount " + p.amount.toNumber() + " lb", value: u8aToString(p.id), amount: p.amount.toNumber(), type: p.processType.toString() }))
            setProcesses(convertedProcesses)

          })
        })


      });
    })


  }, [organization]);
  useEffect(() => {
    if (origin_process && processes) {
      const currentProcess = processes.find(p => p.value === origin_process)
      switch (currentProcess.type) {
        case processesTypes[0]: setFormState({ ...formState, kind: "Bag" })
          break;
        case processesTypes[1]: setFormState({ ...formState, kind: "Bag" })
          break;
        case processesTypes[2]: setFormState({ ...formState, kind: "Bag" })
          break;
        case processesTypes[3]: setFormState({ ...formState, kind: "Bag" })
          break;
        case processesTypes[4]: setFormState({ ...formState, kind: "Bag Roasted" })
          break;
        case processesTypes[5]: setFormState({ ...formState, kind: "Grinded" })
          break;
      }

    }
    console.log([[stringToHex(certifiations)], amount, amount_of_products, amount_for_products, stringToHex(sku), stringToHex(serial_number), stringToHex(brand), origin_process, stringToHex(kind), kind, organization]);
    if (amount_of_products && amount)
      setFormState({ ...formState, amount_for_products: parseFloat(amount) / parseInt(amount_of_products) })
  }, [origin_process, amount, amount_of_products, processes])
  return <Card fluid color='blue'>
    <Card.Content style={{ flexGrow: 0 }} header='Retail Packaging' />
    <Card.Content>
      <Card.Description>
        <Form>
          <Form.Field required >
            <label>Process</label>
            <Form.Dropdown selection fluid
              placeholder='Select process'
              options={processes}
              onChange={(_, dropdown) => {
                setFormState({

                  ...formState, origin_process: dropdown.value, amount: processes.find(p => p.value === dropdown.value).amount
                })
              }}
              value={origin_process}
            />
          </Form.Field>
          <Form.Input
            fluid required
             label='Kind'
            type='text'
            state="kind"
            value={kind}
           />
          <Form.Input
             
            fluid required
            label='Input amount (lb)'
            type='text'
            state="amount"
            value={amount}
           />
          <Form.Input
            fluid required
            label='Amount of products'
            type='text'
            state="amount_of_products"
            value={amount_of_products}
            onChange={onChange}
          />
          <Form.Input
            fluid required
             label='Amount of products (lb)'
            type='text'
            state="amount_for_products"
            value={amount_for_products}
           />
          <Form.Input
            fluid required
            label='Price for products'
            type='text'
            state="price_for_products"
            value={price_for_products}
            onChange={onChange}
          />
          <Form.Input
            fluid required
            label='SKU'
            type='text'
            state="sku"
            value={sku}
            onChange={onChange}
          />
          <Form.Input
            fluid required
            label='Serial number'
            type='text'
            state="serial_number"
            value={serial_number}
            onChange={onChange}
          />
          <Form.Input
            fluid required
            label='Brand'
            type='text'
            state="brand"
            value={brand}
            onChange={onChange}
          />
          <Form.Field required>
            <label>Certifications</label>
            <ReactTagInput
              tags={tags}
              placeholder="Certifications"
              onChange={(newTags) => setTags(newTags)}
            />
          </Form.Field>



          <Form.Field>
            <TxButton
              accountPair={accountPair}
              label='Submit'
              type='SIGNED-TX'
              setStatus={setStatus}
              style={{ display: 'block', margin: 'auto' }}
              attrs={{
                palletRpc: 'retailTransaction',
                callable: 'createRetailPackaging',
                inputParams: [v4(), tags.map(stringToHex), amount, amount_of_products, amount_for_products, price_for_products, stringToHex(sku), stringToHex(serial_number), stringToHex(brand), origin_process, kind, organization],
                paramFields: [true, true, true, true, true, true, true, true, true, true, true, true]
              }}
            />
          </Form.Field>
          <div style={{ overflowWrap: 'break-word' }}>{status}</div>
        </Form>
      </Card.Description>
    </Card.Content>
  </Card>;
}
