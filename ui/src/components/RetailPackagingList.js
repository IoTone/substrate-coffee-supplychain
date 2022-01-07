import React, { useEffect, useState } from 'react';
import { Table, Message, Button } from 'semantic-ui-react';
import { hexToString, u8aToString } from '@polkadot/util';

import { useSubstrate } from '../substrate-lib';

export default function Main(props) {
  const { organization, setSteps } = props;
  const { api } = useSubstrate();
  const [retails, setRetails] = useState([]);

  useEffect(() => {
    let unsub = null;
      const getRetails = async (organization) => {
      api.query.retailTransaction.retailPackagingsByOrg(organization, async retails => {
        api.query.retailTransaction.retailPackagings.multi(retails, async retailsPackagings => {
          let validPackagings = retailsPackagings
            .filter(retail => !retail.isNone)
            .map(retail => retail.unwrap()) 
            setRetails(validPackagings)
        });
      });
    }
    if (organization) {
      getRetails(organization);
    } else {
      setRetails([]);
    }

    return () => unsub && unsub();
  }, [organization, api.query.retailTransaction]);

  if (!process || process.length === 0) {
    return <Message warning>
      <Message.Header>No Retail Packagings registered for your organisation.</Message.Header>
      <p>Please create one using the above form.</p>
    </Message>;
  }
  const getProcess = async packaging_id => {


    setSteps({
      packaging_id,
      steps: []
    }
    )
  }
  return <Table color='blue'>
    <Table.Header>
      <Table.Row>
        <Table.HeaderCell>ID</Table.HeaderCell>
         <Table.HeaderCell>Amount</Table.HeaderCell>
        <Table.HeaderCell>Amount of products</Table.HeaderCell>
        <Table.HeaderCell>Date</Table.HeaderCell>
        <Table.HeaderCell>Brand</Table.HeaderCell>
        <Table.HeaderCell>Serial number</Table.HeaderCell>
        <Table.HeaderCell>Sku</Table.HeaderCell>
        <Table.HeaderCell>Details</Table.HeaderCell>
      </Table.Row>
    </Table.Header>

    <Table.Body>{retails.map(retail => {
      const id = u8aToString(retail.id);
      return <Table.Row key={id} >
        <Table.Cell>{id}</Table.Cell>
         <Table.Cell>{retail.amount.toNumber()}</Table.Cell>
        <Table.Cell>{retail.amount_of_products.toNumber()}</Table.Cell>
       <Table.Cell>{""+new Date(retail.date.toNumber())}</Table.Cell>
         <Table.Cell>{hexToString( retail.brand.toString())}</Table.Cell>
        <Table.Cell>{hexToString(retail.serial_number.toString())}</Table.Cell>
        <Table.Cell>{hexToString(retail.sku.toString())}</Table.Cell>
        <Table.Cell ><Button onClick={() => getProcess(id)}>Details</Button></Table.Cell>

      </Table.Row>;
    })}</Table.Body>
  </Table>;
}
