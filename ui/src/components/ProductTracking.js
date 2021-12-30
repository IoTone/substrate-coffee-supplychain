import React, { useEffect, useState } from 'react';
import { Form, Card, Step, Icon, List } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToString, hexToU8a, stringToHex, u8aToString } from '@polkadot/util';

export default function Main(props) {

  const [steps, setSteps] = useState([]);
  const { prevStepsConfig, organization } = props;

  const { steps: prevSteps, packaging_id } = prevStepsConfig || {}
  useEffect(() => {
    (async function () {
      if (packaging_id) {
        const newSteps = [...prevSteps]

        let packaging = await api.query.retailTransaction.retailPackagings(packaging_id)
        packaging = packaging.unwrap()
        newSteps.push({ type: 2, value: packaging })
        let prevStep = packaging.origin_process.toString()
        while (prevStep) {
          let process = await api.query.supplyChain.processes(prevStep)
          process = process.unwrap()
          newSteps.push({ type: 1, value: process })
          prevStep = process.rawMaterialId.toString()
          let material = await api.query.rawMaterials.rawMaterials(prevStep)
          material = material.unwrap()
          newSteps.push({ type: 0, value: material })
          prevStep = material.origin_process ? material.origin_process.toString() : null
          setSteps([...newSteps])
        }
      }
    })()




  }, [prevStepsConfig]);

  const getTitle = step => {
    switch (step.type) {
      case 0:
        return "Raw material: " + step.value.state

      case 1:

        return "Process " + step.value.processType.toString()

      case 2:

        return "Retail Packaging"
      case 3:

        return "Product"
      case 4:

        return "Sale"
    }


  }
  const getIcon = step => {
    switch (step.type) {
      case 0:
        return "arrow alternate circle up outline"

      case 1:

        return "setting"

      case 2:

        return "box"
      case 3:

        return "coffee"
      case 4:

        return "shop"
    }


  }

  const getDescription = step => {
    const value = step.value
    switch (step.type) {
      case 0:
        return <>

          <List bulleted>
            <List.Item>{"Amount produced: " + value.amount.toNumber()}</List.Item>
            <List.Item>{"Remaining amount: " + value.remaining_amount.toNumber()}</List.Item>
            <List.Item>{"Date: " + new Date(parseInt(value.date))}</List.Item>
          </List>
        </>


      case 1:
        return <>
          <List bulleted>
            <List.Item>{value.attribute.name.toString() + ": " + hexToString(value.attribute.value.toString())}</List.Item>
            <List.Item>{"Input amount: " + value.inputAmount.toNumber()}</List.Item>
            <List.Item>{"Amount produced: " + value.amount.toNumber()}</List.Item>
            <List.Item>Certifications:
              <List >
              </List>
              {value.certifications.map(c => (
                <List.Item>{hexToString(c.toString())}</List.Item>
              ))}
            </List.Item>
            <List.Item>{"Date: " + new Date(parseInt(value.date))}</List.Item>
          </List>
        </>


      case 2:
        return <>
          <List bulleted>
            <List.Item>{"Input amount: " + value.amount.toNumber()}</List.Item>
            <List.Item>{"Amount of products: " + value.amount_of_products.toNumber()}</List.Item>
            <List.Item>{"Amount for products: " + value.amount_for_products.toNumber()}</List.Item>
            <List.Item>{"SKU: " + hexToString(value.sku.toString())}</List.Item>
            <List.Item>{"Serial number: " + hexToString(value.serial_number.toString())}</List.Item>
            <List.Item>{"Brand: " + hexToString(value.brand.toString())}</List.Item>
            <List.Item>Certifications:
              <List >
              </List>
              {value.certifications.map(c => (
                <List.Item>{hexToString(c.toString())}</List.Item>
              ))}
            </List.Item>
            <List.Item>{"Date: " + new Date(parseInt(value.date))}</List.Item>
          </List>
        </>
      case 3:
        return <>
          <List bulleted>
            <List.Item>{"Kind: " + value.kind.toString()}</List.Item>
            <List.Item>{"SKU: " + hexToString(value.sku.toString())}</List.Item>
            <List.Item>{"lb: " + value.lb.toNumber()}</List.Item>
            <List.Item>{"Amount: " + value.amount.toNumber()}</List.Item>
            <List.Item>{"Quantity available: " + value.remaining_amount.toNumber()}</List.Item>
            <List.Item>{"Price: " + value.price.toNumber()}</List.Item>
            <List.Item>{"Date: " + new Date(parseInt(value.date))}</List.Item>
          </List>
        </>
      case 4:
        return <>
          <List bulleted>
            <List.Item>{"Currency: " + hexToString(value.currency_type.toString())}</List.Item>
            <List.Item>{"Cost: " + value.cost.toNumber()}</List.Item>
            <List.Item>{"Quantity of products: " + value.quantity.toNumber()}</List.Item>
            <List.Item>{"SKU: " + hexToString(value.sku.toString())}</List.Item>
            <List.Item>{"Serial number: " + hexToString(value.serial_number.toString())}</List.Item>
            <List.Item>{"buyer: " + value.buyer.toString()}</List.Item>
            <List.Item>{"Date: " + new Date(parseInt(value.date))}</List.Item>
          </List>
        </>
    }


  }
  return <Card fluid color='blue'>
    <Card.Content style={{ flexGrow: 0 }} header='Steps' />
    <Card.Content>
      <Card.Description>
        {steps.map(step => (
          <Step.Group fluid vertical>
            <Step >
              <Icon name={getIcon(step)} />
              <Step.Content>
                <Step.Title>{getTitle(step)}</Step.Title>
                <Step.Description>{getDescription(step)}</Step.Description>
              </Step.Content>
            </Step>
          </Step.Group>
        ))}
      </Card.Description>
    </Card.Content>
  </Card>;
}
