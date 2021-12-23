import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { stringToHex, u8aToString } from '@polkadot/util';
import { useSubstrate } from '../substrate-lib';

export default function Main(props) {
  const [status, setStatus] = useState(null);
  const [rawMaterials, setRawMaterials] = useState([]);
  const [formState, setFormState] = useState({ certifications: null, amount: 0, process: null, attributeValue: null, input_amount: 0, rawMaterial: null });
  const { accountPair, organization } = props;
  const { api } = useSubstrate();

  const onChange = (_, data) =>
    setFormState(prev => ({ ...prev, [data.state]: data.value }));

  const { amount, process, attributeValue, certifications, input_amount, rawMaterial } = formState;
  const processTypes = [{ text: "Harvesing", value: "Harvesing" },
  { text: "Processing", value: "Processing" },
  { text: "Packaging", value: "Packaging" },
  { text: "Transporting", value: "Transporting" },
  { text: "Roasting", value: "Roasting" },
  { text: "Grinding", value: "Grinding" }]
  const attributeName = [
    "Location",
    "Methodology",
    "PackagingType",
    "FreightMethod",
    "RoastingMethod",
    "GrindingMethod",]
  const getAttributeName = () => {
    switch (process) {
      case processTypes[0].value: return attributeName[0]
      case processTypes[1].value: return attributeName[1]
      case processTypes[2].value: return attributeName[2]
      case processTypes[3].value: return attributeName[3]
      case processTypes[4].value: return attributeName[4]
      case processTypes[5].value: return attributeName[5]
    }
  }
  
  useEffect(() => {
    api.query.rawMaterials.rawMaterialsOfOrganization(organization, async rawMaterialsId => {
      api.query.rawMaterials.rawMaterials.multi(rawMaterialsId, async rawMaterials => {
        let validRawMaterials = rawMaterials
          .filter(shipment => !shipment.isNone)
          .map(shipment => shipment.unwrap()).filter(p => p.remaining_amount.toNumber() > 0);
        await Promise.all(validRawMaterials.map(async material => {
           const process = await api.query.supplyChain.processes((material.origin_process.toString()))
          if (!process.isNone) {
            const processUwrap = process.unwrap()
            material.originProcess=processUwrap.processType.toString()
          }
         
        }))
        
        validRawMaterials=validRawMaterials.filter(m=>processTypes.findIndex(pt=> pt.value===m.originProcess)<processTypes.findIndex(pt=> pt.value===process))
         const convertedMaterials = validRawMaterials.map(rawMaterial => ({ text: (rawMaterial.state.toString() === "UnRoasted" ? "Unroasted beans " : "Roasted beans ") + ": remaining amount " + rawMaterial.remaining_amount.toNumber() + " lb"+(rawMaterial.originProcess? " from: "+rawMaterial.originProcess:""), value: u8aToString(rawMaterial.id) }))
        setRawMaterials(convertedMaterials)
      });
    });

  }, [organization,process]);


  return <Card fluid color='blue'>
    <Card.Content style={{ flexGrow: 0 }} header='Register new process' />
    <Card.Content>
      <Card.Description>
        <Form>
          <Form.Field>
            <h3>Process</h3>
            <Form.Dropdown selection fluid
              placeholder='Select Organization'
              options={processTypes}
              onChange={(_, dropdown) => setFormState({ ...formState, process: dropdown.value })}
              value={process}
            />
          </Form.Field>
          <Form.Field>
            <h3>Raw Material</h3>
            <Form.Dropdown selection fluid
              placeholder='Select Raw Material'
              options={rawMaterials}
              onChange={(_, dropdown) => setFormState({ ...formState, rawMaterial: dropdown.value })}
              value={rawMaterial}
            />
          </Form.Field>
          <Form.Input
            fluid required
            label={getAttributeName}
            type='text'
            state='attributeValue'
            onChange={onChange}
          />

          <Form.Input
            fluid required
            label='Certifications'
            type='text'
            state='certifications'
            onChange={onChange}
          />
          <Form.Input
            fluid required
            label='Input amount'
            state='input_amount'
            onChange={onChange}
          />
          <Form.Input
            fluid required
            label='Produced amount'
            state='amount'
            onChange={onChange}
          />
          <Form.Field>
            <TxButton
              accountPair={accountPair}
              label='Submit'
              type='SIGNED-TX'
              setStatus={setStatus}
              style={{ display: 'block', margin: 'auto' }}
              attrs={{
                palletRpc: 'supplyChain',
                callable: 'registerProcess',
                inputParams: [v4(), [getAttributeName(), stringToHex(attributeValue)], [stringToHex(certifications)], amount, input_amount, process, rawMaterial, organization],
                paramFields: [true, true, true, true, true, true, true, true]
              }}
            />
          </Form.Field>
          <div style={{ overflowWrap: 'break-word' }}>{status}</div>
        </Form>
      </Card.Description>
    </Card.Content>
  </Card>;
}
