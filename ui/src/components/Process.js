import React, { useState } from 'react';
import { Container, Divider, Grid, Header } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';
import OrganizationSelector from './OrganizationSelector';

 import ProcessList from './ProcessList';
import StepTracking from './StepTracking';
import CreateNewProcess from './CreateNewProcess';
import Events from './Events';

export default function Main(props) {
  const { accountPair } = props;
  const [selectedOrganization, setSelectedOrganization] = useState('');
  const [steps, setSteps] = useState()
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
          <CreateNewProcess accountPair={accountPair} organization={selectedOrganization} />
        </Grid.Column>
        <Grid.Column style={{ display: 'flex' }}>
          <Events />
        </Grid.Column>
      </Grid.Row>


    </Grid>
    <Divider style={{ marginTop: '2em' }} />
    <Header as='h2'>Process Listing</Header>
    <ProcessList accountPair={accountPair}
      organization={selectedOrganization}
      setSteps={setSteps}
    />
    <StepTracking prevStepsConfig={steps} />
  </Container>;
}
