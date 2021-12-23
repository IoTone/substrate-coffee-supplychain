import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToU8a, u8aToString } from '@polkadot/util';

export default function Main(props) {
  const [status, setStatus] = useState(null);
  const [processes, setProcesses] = useState([]);
  const [formState, setFormState] = useState({ state: null, originProcess: null, amount: 0.0, organization: null });
  const { accountPair, organization } = props;
  const materialStates = ["Roasted",
    "UnRoasted"]
  const onChange = (_, data) => {
    console.log(data);
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
          
              validProcesses=validProcesses.filter(vp=>(  !validRawMaterials.find(vrm=>u8aToString(hexToU8a( vrm.origin_process.toString()))=== u8aToString(vp.id))))
            const convertedProcesses = validProcesses.map(p => ({ text: p.processType.toString() + " Amount " + p.amount.toNumber() + " lb", value: u8aToString(p.id), amount: p.amount.toNumber(), type: p.processType.toString() }))
            convertedProcesses.push({ text: "Null", value: "null", amount: null })
            setProcesses(convertedProcesses)

          })
        })


      });
    })


  }, [organization]);
  const materialState = processes.find(p => p.value === originProcess)
  return <Card fluid color='blue'>
    <Card.Content style={{ flexGrow: 0 }} header='Register raw material' />
    <Card.Content>
      <Card.Description>
        <Form>
          <Form.Input
            fluid required
            label='Amount (lb)'
            type='text'
            state="amount"
            value={amount}
            disabled={originProcess !== null}
            onChange={onChange}
          />
          <Form.Field  >
            <h5>Process</h5>
            <Form.Dropdown selection fluid
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
            fluid required
            label='State'
            type='select'
            disabled={true}
            value={materialState && materialState.type === "Roasting" ? "Roasted" : "Unroasted"}
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
                palletRpc: 'rawMaterials',
                callable: 'registerRawMaterial',
                inputParams: [v4(), "UnRoasted", organization, originProcess === "null" ? null : originProcess, amount],
                paramFields: [true, true, true, { optional: true }, true]
              }}
            />
          </Form.Field>
          <div style={{ overflowWrap: 'break-word' }}>{status}</div>
        </Form>
      </Card.Description>
    </Card.Content>
  </Card>;
}
