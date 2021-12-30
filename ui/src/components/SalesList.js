import React, { useEffect, useState } from 'react';
import { Table, Message, Button } from 'semantic-ui-react';
import { hexToString, u8aToString } from '@polkadot/util';

import { useSubstrate } from '../substrate-lib';

export default function Main(props) {
  const { organization, setSteps } = props;
  const { api } = useSubstrate();
  const [sales, setSales] = useState([]);

  useEffect(() => {
    let unsub = null;

    async function getSales(organization) {
      unsub = await api.query.retailTransaction.salesByOrg(organization, salesIds => {
        api.query.retailTransaction.sales.multi(salesIds, salesList => {
          const validSales = salesList
            .filter(sale => !sale.isNone)
            .map(sale => sale.unwrap());
          setSales(validSales);
        });
      });
    }

    if (organization) {
      getSales(organization);
    } else {
      setSales([]);
    }

    return () => unsub && unsub();
  }, [organization, api.query.retailTransaction]);

  if (!sales || sales.length === 0) {
    return <Message warning>
      <Message.Header>No sales registered for your organisation.</Message.Header>
      <p>Please create one using the above form.</p>
    </Message>;
  }
  const getProduct = async sale => {
    const sku = sale.sku.toString()
    const product = await api.query.coffeProducts.products(sku)
    const unwrapProduct = product.unwrap()
console.log({unwrapProduct});
    setSteps({
      packaging_id: unwrapProduct.packaging_id.toString(),
      steps:[{type:4,value:sale},{type:3,value:unwrapProduct}]
  }
    )
  }
  return <Table color='blue'>
    <Table.Header>
      <Table.Row>
        <Table.HeaderCell>ID</Table.HeaderCell>
        <Table.HeaderCell>Buyer</Table.HeaderCell>
        <Table.HeaderCell>Currency</Table.HeaderCell>
        <Table.HeaderCell>SKU</Table.HeaderCell>
        <Table.HeaderCell>Details</Table.HeaderCell>
      </Table.Row>
    </Table.Header>

    <Table.Body>{sales.map(sale => {
      const id = u8aToString(sale.id);
      return <Table.Row key={id} >
        <Table.Cell>{id}</Table.Cell>
        <Table.Cell>{sale.buyer.toString()}</Table.Cell>
        <Table.Cell>{hexToString(sale.currency_type.toString()) }</Table.Cell>
        <Table.Cell>{hexToString(sale.sku.toString())}</Table.Cell>
        <Table.Cell ><Button onClick={() => getProduct(sale)}>Details</Button></Table.Cell>

      </Table.Row>;
    })}</Table.Body>
  </Table>;
}
