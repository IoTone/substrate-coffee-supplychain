import React, { useEffect, useState } from 'react';
import { Table, Message, Button } from 'semantic-ui-react';
import { hexToString, u8aToString } from '@polkadot/util';

import { useSubstrate } from '../substrate-lib';

export default function Main(props) {
  const { organization, setSteps } = props;
  const { api } = useSubstrate();
  const [processes, setProcesses] = useState([]);

  useEffect(() => {
    let unsub = null;

    async function getProcesses(organization) {
      api.query.supplyChain.processesOfOrganization(organization, processIds => {
        api.query.supplyChain.processes.multi(processIds, process => {
          let validProcesses = process
            .filter(p => !p.isNone)
            .map(p => p.unwrap());
          setProcesses(validProcesses);
        });
      });
    }

    if (organization) {
      getProcesses(organization);
    } else {
      setProcesses([]);
    }

    return () => unsub && unsub();
  }, [organization, api.query.supplyChain]);

  if (!process || process.length === 0) {
    return <Message warning>
      <Message.Header>No Processes registered for your organisation.</Message.Header>
      <p>Please create one using the above form.</p>
    </Message>;
  }
  const getProcess = async process => {


    setSteps({
      processId: u8aToString(process.id),
      steps: []
    }
    )
  }
  return <Table color='blue'>
    <Table.Header>
      <Table.Row>
        <Table.HeaderCell>ID</Table.HeaderCell>
        <Table.HeaderCell>Process Type</Table.HeaderCell>
        <Table.HeaderCell>Amount</Table.HeaderCell>
        <Table.HeaderCell>Date</Table.HeaderCell>
        <Table.HeaderCell>Details</Table.HeaderCell>
      </Table.Row>
    </Table.Header>

    <Table.Body>{processes.map(process => {
      const id = u8aToString(process.id);
      return <Table.Row key={id} >
        <Table.Cell>{id}</Table.Cell>
        <Table.Cell>{process.processType.toString()}</Table.Cell>
        <Table.Cell>{process.amount.toNumber()}</Table.Cell>
        <Table.Cell>{new Date(process.date.toNumber()).toString()}</Table.Cell>
        <Table.Cell ><Button onClick={() => getProcess(process)}>Details</Button></Table.Cell>

      </Table.Row>;
    })}</Table.Body>
  </Table>;
}
