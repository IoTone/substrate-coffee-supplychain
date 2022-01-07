import React, { useEffect, useState } from 'react';
import { Table, Message, Button } from 'semantic-ui-react';
import { hexToString, u8aToString } from '@polkadot/util';

import { useSubstrate } from '../substrate-lib';

export default function Main(props) {
  const { organization, setSteps } = props;
  const { api } = useSubstrate();
  const [materials, setMaterials] = useState([]);

  useEffect(() => {
    let unsub = null;
      const getMaterials = async (organization) => {
      api.query.rawMaterials.rawMaterialsOfOrganization(organization, async rawMaterialsId => {
        api.query.rawMaterials.rawMaterials.multi(rawMaterialsId, async rawMaterials => {
          let validRawMaterials = rawMaterials
            .filter(shipment => !shipment.isNone)
            .map(shipment => shipment.unwrap()) 
          await Promise.all(validRawMaterials.map(async material => {
            const process = await api.query.supplyChain.processes((material.origin_process.toString()))
            if (!process.isNone) {
              const processUwrap = process.unwrap()
              material.originProcess = processUwrap.processType.toString()
            }
            setMaterials(validRawMaterials)
          }))
        });
      });
    }
    if (organization) {
      getMaterials(organization);
    } else {
      setMaterials([]);
    }

    return () => unsub && unsub();
  }, [organization, api.query.supplyChain]);

  if (!process || process.length === 0) {
    return <Message warning>
      <Message.Header>No Rawmaterials registered for your organisation.</Message.Header>
      <p>Please create one using the above form.</p>
    </Message>;
  }
  const getProcess = async materialId => {


    setSteps({
      materialId,
      steps: []
    }
    )
  }
  return <Table color='blue'>
    <Table.Header>
      <Table.Row>
        <Table.HeaderCell>ID</Table.HeaderCell>
        <Table.HeaderCell>Origin process</Table.HeaderCell>
        <Table.HeaderCell>Amount</Table.HeaderCell>
        <Table.HeaderCell>Reimaining amount</Table.HeaderCell>
        <Table.HeaderCell>Date</Table.HeaderCell>
        <Table.HeaderCell>Details</Table.HeaderCell>
      </Table.Row>
    </Table.Header>

    <Table.Body>{materials.map(material => {
      const id = u8aToString(material.id);
      return <Table.Row key={id} >
        <Table.Cell>{id}</Table.Cell>
        <Table.Cell>{material.originProcess }</Table.Cell>
        <Table.Cell>{material.amount.toNumber()}</Table.Cell>
        <Table.Cell>{material.remaining_amount.toNumber()}</Table.Cell>
        <Table.Cell>{""+new Date(material.date.toNumber())}</Table.Cell>
        <Table.Cell ><Button onClick={() => getProcess(id)}>Details</Button></Table.Cell>

      </Table.Row>;
    })}</Table.Body>
  </Table>;
}
