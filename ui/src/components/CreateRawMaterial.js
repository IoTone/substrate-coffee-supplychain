import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToU8a, u8aToString } from '@polkadot/util';
import ReactTooltip from "react-tooltip";

export default function Main(props) {
  const [status, setStatus] = useState(null);
  const [processes, setProcesses] = useState([]);
  const [formState, setFormState] = useState({ state: null, originProcess: null, amount: 0, organization: null });
  const { accountPair, organization } = props;
  const materialStates = ["Roasted",
    "UnRoasted", "Grinded"]
  const onChange = (_, data) => {
    if(!data.value.match(/^[a-zA-Z0-9_.-]*$/))
    return
    if (data.state === "amount") {
      if (!data.value.match(/^[0-9]*$/))
        return
      if (parseFloat(data.value) <= 0)
        return

    }
    setFormState(prev => ({ ...prev, [data.state]: data.value }));
  }
  const { amount, originProcess, state, } = formState;
  useEffect(() => {

    api.query.supplyChain.processesOfOrganization(organization, processIds => {
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

  const getState = () => {
    if (materialState)
      switch (materialState.type) {
        case "Roasting":
          return "Roasted"
        case "Grinding":
          return "Grinded"

        default: return "Unroasted"
      }
    return "Unroasted"
  }
  const materialState = processes.find(p => p.value === originProcess)
  console.log({ amount });
  return <Card fluid color='blue'>
    <Card.Content style={{ flexGrow: 0 }} header='Register raw material' />
    <Card.Content>
      <Card.Description>
        <Form>
          <Form.Input
            data-for="amount"
            data-tip="Amount<br />of<br />raw material"
            data-iscapture="true"
            fluid required
            label='Amount (lb)'
            type="text"
            state="amount"
            value={amount}

            disabled={originProcess !== null}
            onChange={onChange}
          />
          <Form.Field  >
            <h5>Process</h5>
            <Form.Dropdown
              data-for="process"
              data-tip="process where <br />comes from <br />the<br />raw material"
              data-iscapture="true"
              selection fluid disabled={processes.length === 0}
              placeholder='Select process'
              options={processes}
              onChange={(_, dropdown) => {
                setFormState({

                  ...formState, originProcess: dropdown.value, amount: processes.find(p => p.value === dropdown.value).amount
                })
              }}
              value={originProcess}
            />
          </Form.Field>

          <Form.Input
           data-for="kind"
           data-tip="Kind <br />of<br />raw material"
           data-iscapture="true"
            fluid required
            label='State'
            type='select'
            className="disable-input"
            value={getState()}
          />
          <Form.Field>
            <TxButton
              setClean={(e) => {
                e.preventDefault()
                setFormState({ amount: 0, originProcess: null, attributeValue: "", })
              }}
              accountPair={accountPair}
              label='Submit'
              type='SIGNED-TX'
              setStatus={setStatus}
              style={{ display: 'block', margin: 'auto' }}
              attrs={{

                palletRpc: 'rawMaterials',
                callable: 'registerRawMaterial',
                inputParams: [v4(), getState(), organization, originProcess === "null" ? null : originProcess, amount],
                paramFields: [true, true, true, { optional: true }, true]
              }}
            />
          </Form.Field>
          <div style={{ overflowWrap: 'break-word' }}>{status}</div>
        </Form>
      </Card.Description>
    </Card.Content>
    <ReactTooltip
      id="amount"
      place="left"
      type="info"
      effect="solid"
      multiline={true}
    />
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
  </Card>;
}
