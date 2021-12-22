import React, { useState } from 'react';
import { Grid } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';
import OrganizationSelector from './OrganizationSelector';

import AddToOrg from './AddToOrg';
import CreateNewProcess from './CreateNewProcess';
import Events from './Events';

export default function Main(props) {
  const { accountPair } = props;
  const [selectedOrganization, setSelectedOrganization] = useState('');

  return <Grid columns="2">

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

 
  </Grid>;
}
