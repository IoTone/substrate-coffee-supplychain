import React, { useEffect, useState } from 'react';
import { Form, Card } from 'semantic-ui-react';
import { TxButton } from '../substrate-lib/components';
import { v4 } from 'uuid';
import { hexToString, hexToU8a, stringToHex, u8aToString } from '@polkadot/util';
import { useSubstrate } from '../substrate-lib';

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
        if (data.state == "quantity" && sku) {
            if (parseInt(products.find(p => p.value === sku).remaining) < parseInt(data.value))
                return
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
    useEffect(() =>
        console.log(formState), [formState])
    return <Card fluid color='blue'>
        <Card.Content style={{ flexGrow: 0 }} header='Sales' />
        <Card.Content>
            <Card.Description>
                <Form>
                    <Form.Input

                        fluid required
                        label='Currency'
                        type='text'
                        state="currency"
                        value={currency}
                        onChange={onChange}
                    />
                    <Form.Field required >
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
                        <Form.Input
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



                    </Form.Field>
                    <Form.Field required>
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
                            setClean={()=>setFormState({ currency: null, quantity: 0, sku: null, serial_number: null, buyer: null })}
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
    </Card>;
}
