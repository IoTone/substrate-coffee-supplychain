import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToString, hexToU8a, stringToHex, u8aToString } from '@polkadot/util';
import ReactTagInput from "@pathofdev/react-tag-input";
import "@pathofdev/react-tag-input/build/index.css";
import ReactTooltip from 'react-tooltip';

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
  const [formState, setFormState] = useState({ certifiations: null, amount: 0, amount_of_products: 0, price_for_products: 0, amount_for_products: 0, sku: null, serial_number: null, brand: null, origin_process: null, kind: null });
  const { accountPair, organization } = props;

  const onChange = (_, data) => {
    if(!data.value.match(/^[a-zA-Z0-9_.-]*$/))
    return
    if ((data.state === "amount" || data.state === "amount_of_products" || data.state === "amount_for_products" || data.state === "price_for_products")) {
      if (!data.value.match(/^[0-9]*$/))
        return
      if (parseFloat(data.value) <= 0)
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
        case processesTypes[4]: setFormState({ ...formState, kind: "BagRoasted" })
          break;
        case processesTypes[5]: setFormState({ ...formState, kind: "Grinded" })
          break;
      }

    }
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
             data-for="process"
             data-tip="Kind <br />of<br />process where comes from"
             data-iscapture="true"
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
               data-for="kind"
               data-tip="kind of product"
               data-iscapture="true"
            fluid required
            label='Kind'
            type='text'
            state="kind"
            value={kind}
          />
          <Form.Input
            data-for="inputamount"
            data-tip="input amount of material"
            data-iscapture="true"
            disabled={!origin_process}
            fluid required
            label='Input amount (lb)'
            type='text'
            state="amount"

            value={amount}
          />
          <Form.Input
            data-for="amountforsale"
            data-tip="amount of products<br />to sale"
            data-iscapture="true"
            fluid required
            label='Amount of products'
            type='text'
            state="amount_of_products"
            value={amount_of_products}
            onChange={onChange}
          />
          <Form.Input
            data-for="amountoflb"
            data-tip="amount of lb<br />for product"
            data-iscapture="true"
            fluid required
            disabled={!amount_of_products}
            label='Amount for products (lb)'
            type='text'
            state="amount_for_products"
            value={amount_for_products}
          />
          <Form.Input
            data-for="price"
            data-tip="price for each<br />product for sale"
            data-iscapture="true"
            fluid required
            label='Price for products'
            type='text'
            state="price_for_products"
            value={price_for_products}
            onChange={onChange}
          />
          <Form.Input
            data-for="sku"
            data-tip="sku is a distinct <br />type of item for sale"
            data-iscapture="true"
            fluid required
            label='SKU'
            type='text'
            state="sku"
            value={sku}
            onChange={onChange}
          />
          <Form.Input
            data-for="brand"
            data-tip="Serial number"
            data-iscapture="true"
            fluid required
            label='Serial number'
            type='text'
            state="serial_number"
            value={serial_number}
            onChange={onChange}
          />
          <Form.Input
            data-for="brand"
            data-tip="brand <br />for <br />the product"
            data-iscapture="true"
            fluid required
            label='Brand'
            type='text'
            state="brand"
            value={brand}
            onChange={onChange}
          />
          <Form.Field required data-for="certs"
            data-tip="certifications <br />for <br />the process"
            data-iscapture="true">
            <label>Certifications</label>
            <ReactTagInput
              tags={tags}
              placeholder="Certifications"
              onChange={(newTags) => setTags(newTags)}
            />
          </Form.Field>



          <Form.Field>
            <TxButton
              setClean={(e) => {
                e.preventDefault()
                setTags([])
                setFormState({ certifiations: null, amount: 0, amount_of_products: 0, price_for_products: 0, amount_for_products: 0, sku: "", serial_number: "", brand: "", origin_process: null, kind: null })
              }}
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
   
    
    <ReactTooltip
      id="process"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="kind"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="inputamount"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="amountforsale"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="price"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="sku"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="serialnumber"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="certs"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
  </Card>;
}
