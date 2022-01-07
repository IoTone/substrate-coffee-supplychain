import React, { useDebugValue, useEffect, useState } from 'react';
import { Form, Card, Button } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { stringToHex, u8aToString } from '@polkadot/util';
import { useSubstrate } from '../substrate-lib';

import ReactTagInput from "@pathofdev/react-tag-input";
import "@pathofdev/react-tag-input/build/index.css";
import ReactTooltip from 'react-tooltip';
export default function Main(props) {
  const [status, setStatus] = useState(null);
  const [rawMaterials, setRawMaterials] = useState([]);
  const [formState, setFormState] = useState({ certifications: null, amount: 0, process: null, attributeValue: null, input_amount: 0, rawMaterial: null });
  const { accountPair, organization } = props;
  const { api } = useSubstrate();
  const [tags, setTags] = useState([]);
  const { amount, process, attributeValue, certiications, input_amount, rawMaterial } = formState;
  const onChange = (_, data) => {
    if(!data.value.match(/^[a-zA-Z0-9_.-]*$/))
    return
    if ((data.state === "input_amount" || data.state === "amount")) {
      if (!data.value.match(/^[0-9]*$/)) {
        return
      }

      if (parseFloat(data.value) <= 0)
        return


    }
    if (process && data.state === "input_amount" && parseInt(rawMaterials.find(r => r.value === rawMaterial).remaining_amount) < parseInt(data.value))

      return

    setFormState(prev => ({ ...prev, [data.state]: data.value }));
  }

  const processTypes = [{ text: "Harvesting", value: "Harvesting" },
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
      default: return "Procces value"
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
            material.originProcess = processUwrap.processType.toString()
          }

        }))
        if (process === processTypes[0].value)
          validRawMaterials = validRawMaterials.filter(m => m.originProcess === undefined)
        else if (process !== processTypes[3].value)
          validRawMaterials = validRawMaterials.filter(m => m.originProcess && (processTypes.findIndex(pt => pt.value === m.originProcess) < processTypes.findIndex(pt => pt.value === process)))

        const convertedMaterials = validRawMaterials.map(rawMaterial => ({ text: (rawMaterial.state.toString() === "UnRoasted" ? "Unroasted beans " : "Roasted beans ") + ": remaining amount " + rawMaterial.remaining_amount.toNumber() + " lb" + (rawMaterial.originProcess ? " from: " + rawMaterial.originProcess : ""), value: u8aToString(rawMaterial.id), remaining_amount: rawMaterial.remaining_amount.toNumber() }))
        setRawMaterials(convertedMaterials||[])
      });
    });

  }, [organization, process]);
  useEffect(() => {
    console.log({ formState });
  }, [formState])

  return <Card fluid color='blue'>
    <Card.Content style={{ flexGrow: 0 }} header='Register new process' />
    <Card.Content>
      <Card.Description>
        <Form>
          <Form.Field required>

            <label>Process</label>
            <Form.Dropdown
              data-for="process"
              data-tip="Kind <br />of<br />process"
              data-iscapture="true"
              selection fluid
              placeholder='Select Organization'
              options={processTypes}
              onChange={(_, dropdown) => setFormState({ ...formState, process: dropdown.value })}
              value={process}
            />
          </Form.Field>
          <Form.Field required>
            <label>Raw Material</label>
            <Form.Dropdown
              data-for="kind"
              data-tip="Kind <br />of<br />raw material"
              data-iscapture="true"
              selection fluid
              placeholder='Select Raw Material'
              options={rawMaterials}
              onChange={(_, dropdown) => setFormState({ ...formState, rawMaterial: dropdown.value })}
              value={rawMaterial}
            />
          </Form.Field>
          <Form.Input
            fluid required
            label={getAttributeName()}
            type='text'
            state='attributeValue'
            value={attributeValue}
            onChange={onChange}
          />
          <Form.Field required
            data-for="certs"
            data-tip="certifications <br />for <br />the process"
            data-iscapture="true">
            <label>Certifications</label>
            <ReactTagInput

              tags={tags}
              placeholder="Certifications"
              onChange={(newTags) => setTags(newTags)}
            /></Form.Field>
          <Form.Input
            fluid required
            data-for="input"
            data-tip="the input amount of<br /> raw material for <br />the process"
            data-iscapture="true"
            disabled={!rawMaterial}
            type="text"
            label='Input amount'
            state='input_amount'
            value={input_amount}
            onChange={onChange}
          />
          <Form.Input
            data-for="produced"
            data-tip="the Produced amount of<br /> of <br />the process"
            data-iscapture="true"
            fluid required
            type="text"
            value={amount}
            label='Produced amount'
            state='amount'
            onChange={onChange}
          />
          <Form.Field>

            <TxButton
              accountPair={accountPair}
              setClean={(e) => {
                e.preventDefault()
                setTags([])
                setFormState({ certifications: null, amount: 0, process: null, attributeValue: "", input_amount: 0, rawMaterial: null })
              }}
              label='Submit'
              type='SIGNED-TX'
              setStatus={setStatus}
              style={{ display: 'block', margin: 'auto' }}
              attrs={{
                palletRpc: 'supplyChain',
                callable: 'registerProcess',
                inputParams: [v4(), [getAttributeName(), stringToHex(attributeValue)], tags.map(stringToHex), amount, input_amount, process, rawMaterial, organization],
                paramFields: [true, true, true, true, true, true, true, true]
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
      id="certs"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="amount"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="input"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
    <ReactTooltip
      id="produced"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
  </Card>;
}
