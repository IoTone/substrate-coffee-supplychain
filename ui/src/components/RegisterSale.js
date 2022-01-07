import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToString, hexToU8a, stringToHex, u8aToString } from '@polkadot/util';
import { useSubstrate } from '../substrate-lib';
import ReactTooltip from 'react-tooltip';

export default function Main(props) {

    const [status, setStatus] = useState(null);
    const [products, setProducts] = useState([]);
    const [formState, setFormState] = useState({ currency: null, quantity: 0, sku: null, serial_number: null, buyer: null });
    const { accountPair, organization } = props;
    const { keyring } = useSubstrate();

    const keyringOptions = keyring.getPairs().map(account => ({
        key: account.address,
        value: account.address,
        text: account.meta.name.toUpperCase(),
        icon: 'user'
    }));
    const onChange = (_, data) => {
        if(!data.value.match(/^[a-zA-Z0-9_.-]*$/))
        return
        if (data.state == "quantity") {
            if (!data.value.match(/^[0-9]*$/))
                return
            if (parseFloat(data.value) <= 0)
                return

            if (sku) {
                if (parseInt(products.find(p => p.value === sku).remaining) < parseInt(data.value))
                    return
            } else {
                if (parseFloat(data.value) > 0)
                    return
            }
        }
        setFormState(prev => ({ ...prev, [data.state]: data.value }));
    }
    const { currency, quantity, sku, serial_number, buyer } = formState;
    useEffect(() => {

        api.query.coffeProducts.productsOfOrganization(organization, processIds => {
            api.query.coffeProducts.products.multi(processIds, async rawMaterials => {
                let validProducts = rawMaterials
                    .filter(material => !material.isNone).map(m => m.unwrap());
                validProducts = validProducts.map(v => ({ text: "SKU: " + hexToString(v.sku.toString()) + " (remaining products: " + v.remaining_amount.toNumber() + ")", value: v.sku, remaining: v.remaining_amount.toNumber() })).filter(p => p.remaining > 0)
                setProducts(validProducts)

            })
        })


    }, [organization, sku]);
    const maxAmount = () => {
        if (sku) {
            let product = products.find(p => p.value === sku)
            if (product)
                return product.remaining
        }
        return 0
    }

    return <Card fluid color='blue'>
        <Card.Content style={{ flexGrow: 0 }} header='Sales' />
        <Card.Content>
            <Card.Description>
                <Form>
                    <Form.Input
                        data-for="currency"
                        data-tip="Currency<br />of<br /> the payment"
                        data-iscapture="true"
                        fluid required
                        label='Currency'
                        type='text'
                        state="currency"
                        value={currency}
                        onChange={onChange}
                    />
                    <Form.Field required
                        data-for="sku"
                        data-tip="SKU<br />of<br />the product to sale"
                        data-iscapture="true"
                    >
                        <label>Sku</label>
                        <Form.Dropdown selection fluid
                            placeholder='Select process'
                            options={products}
                            onChange={(_, dropdown) => {
                                setFormState({

                                    ...formState, sku: dropdown.value
                                })
                            }}
                            value={sku}
                        />
                    </Form.Field>

                    <Form.Input
                        data-for="amount"
                        data-tip="Amount <br />of<br />products to sale"
                        data-iscapture="true"
                        fluid required
                        label={'Max amount of products (' + maxAmount() + ")"}
                        type='text'
                        state="quantity"
                        value={quantity}
                        onChange={onChange}
                    />



                    <Form.Input

                        fluid required
                        label='Serial number'
                        type='text'
                        state="serial_number"
                        value={serial_number}
                        onChange={onChange}
                    />



                    <Form.Field required
                        data-for="buyer"
                        data-tip="buyer <br />of<br />the products"
                        data-iscapture="true"
                    >
                        <label>Buyer</label>
                        <Form.Dropdown selection fluid
                            placeholder='Select process'
                            options={keyringOptions}
                            onChange={(_, dropdown) => {
                                setFormState({

                                    ...formState, buyer: dropdown.value
                                })
                            }}
                            value={buyer}
                        />
                    </Form.Field>


                    <Form.Field>
                        <TxButton
                            accountPair={accountPair}
                            label='Submit'
                            type='SIGNED-TX'
                            setClean={() => setFormState({ currency: "", quantity: 0, sku: null, serial_number: "", buyer: null })}
                            setStatus={setStatus}
                            style={{ display: 'block', margin: 'auto' }}
                            attrs={{
                                palletRpc: 'retailTransaction',
                                callable: 'createSale',
                                inputParams: [v4(), currency, quantity, sku, stringToHex(serial_number), buyer, organization],
                                paramFields: [true, true, true, true, true, true, true]
                            }}
                        />
                    </Form.Field>
                    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
                </Form>
            </Card.Description>
        </Card.Content>

        <ReactTooltip
            id="currency"
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
            id="amount"
            place="left"
            type="info"
            effect="solid"
            multiline={true}
        />

        <ReactTooltip
            id="buyer"
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
    </Card>;
}
