import React, { useState } from 'react';
import { Container, Divider, Grid, Header } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';
import OrganizationSelector from './OrganizationSelector';

import RegisterSale from './RegisterSale';
import SalesList from './SalesList';
import ProductTracking from './ProductTracking';
import Events from './Events';

export default function Main(props) {
  const { accountPair } = props;
  const [steps,setSteps]=useState(null)
  const [selectedOrganization, setSelectedOrganization] = useState('');

  return <Container>
    <Grid columns="2">
      <Grid.Row>
        <Grid.Column width={16}>
          <OrganizationSelector
            accountPair={accountPair}
            setSelectedOrganization={setSelectedOrganization}
          />
        </Grid.Column>
      </Grid.Row>
      <Grid.Row>

        <Grid.Column style={{ display: 'flex' }} >
          <RegisterSale accountPair={accountPair} organization={selectedOrganization} />
        </Grid.Column>
        <Grid.Column style={{ display: 'flex' }}>
          <Events />
        </Grid.Column>
      </Grid.Row>
    </Grid>
    <Divider style={{ marginTop: '2em' }} />
    <Header as='h2'>Sales Listing</Header>
    <SalesList accountPair={accountPair}
      organization={selectedOrganization}
      setSteps={setSteps}
    />
    <ProductTracking prevStepsConfig={steps}/>
  </Container>
    ;
}
