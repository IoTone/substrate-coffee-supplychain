import React, { useEffect, useState } from 'react';
import { Button, Card, Feed } from 'semantic-ui-react';
import { useSubstrate } from '../substrate-lib';

// Events to be filtered from feed
const FILTERED_EVENTS = [
  'system:ExtrinsicSuccess:: (phase={"ApplyExtrinsic":0})',
  'system:ExtrinsicSuccess:: (phase={"ApplyExtrinsic":1})'
];

function Main (props) {
  const { api } = useSubstrate();
  const [eventFeed, setEventFeed] = useState([]);

  useEffect(() => {
    let unsub = null;
    const allEvents = async () => {
      unsub = await api.query.system.events(events => {
        // loop through the Vec<EventRecord>
        events.forEach(record => {
          // extract the phase, event and the event types
          const { event, phase } = record;
          const types = event.typeDef;

          // show what we are busy with
          const eventName = `${event.section}:${
            event.method
          }:: (phase=${phase.toString()})`;

          if (FILTERED_EVENTS.includes(eventName)) return;

          // loop through each of the parameters, displaying the type and data
          const params = event.data.map(
            (data, index) => `${types[index].type}: ${data.toString()}`
          );
          console.log({eventName});
           setEventFeed(e => [{
            icon: eventName.includes("ExtrinsicFailed")?'bug':'bell',
            summary: `${eventName}-${e.length}`,
            extraText: event && event.meta && event.meta.documentation ? event.meta.documentation.join(', ').toString() : '',
            content: params.join(', ')
          }, ...e]);
        });
      });
    };

    allEvents();
    return () => unsub && unsub();
  }, [api.query.system]);

  const maxHeight = props.maxHeight || 250;

  return <Card fluid color = 'blue'>
    <Card.Content style={{ flexGrow: 0 }}>
      <Card.Header>
        Events
        <Button
          basic circular
          size='mini'
          color='grey'
          floated='right'
          icon='erase'
          onClick={ _ => setEventFeed([]) }
        />
      </Card.Header>
    </Card.Content>
    <Card.Content>
      <Feed style={{ clear: 'both', overflow: 'auto', maxHeight }} events={eventFeed} />
    </Card.Content>
  </Card>;
}

export default function Events (props) {
  const { api } = useSubstrate();
  return api.query && api.query.system && api.query.system.events
    ? (
    <Main {...props} />
      )
    : null;
}
