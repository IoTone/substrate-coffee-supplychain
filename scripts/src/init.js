import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { v4 as uuidv4 } from 'uuid';

import submit from './lib/submit-signed-xt.js';
import types from './lib/types.js';

async function main() {
  const provider = new WsProvider("ws://186.117.170.215:9944");
  const api = await ApiPromise.create({ provider, types });
  const keyring = new Keyring({ type: 'sr25519' });

  const users = {
    admin: { key: keyring.addFromUri('//Alice', { name: 'ADMIN' }), nonce: 0 },
    bob: { key: keyring.addFromUri('//Bob', { name: 'Bob' }), nonce: 0 },
    bobBank: { key: keyring.addFromUri('//Bob//stash', { name: 'Bob-BANK' }), nonce: 0 },
    betty: { key: keyring.addFromUri('//Bert', { name: 'Bert' }), nonce: 0 },
    charlie: { key: keyring.addFromUri('//Charlie', { name: 'Charlie' }), nonce: 0 },
    charlieBank: { key: keyring.addFromUri('//Charlie//stash', { name: 'Charlie-BANK' }), nonce: 0 },
    clarice: { key: keyring.addFromUri('//Clarice', { name: 'Clarice' }), nonce: 0 },
    dave: { key: keyring.addFromUri('//Dave', { name: 'Dave' }), nonce: 0 },
    daveBank: { key: keyring.addFromUri('//Dave//stash', { name: 'Dave-BANK' }), nonce: 0 },
    daisy: { key: keyring.addFromUri('//Daisy', { name: 'Daisy' }), nonce: 0 },
    eve: { key: keyring.addFromUri('//Eve', { name: 'Eve' }), nonce: 0 },
    eveBank: { key: keyring.addFromUri('//Eve//stash', { name: 'Eve-BANL' }), nonce: 0 },
    erowid: { key: keyring.addFromUri('//Erowid', { name: 'Erowid' }), nonce: 0 },
    ferdie: { key: keyring.addFromUri('//Ferdie', { name: 'Ferdie' }), nonce: 0 },
    ferdieBank: { key: keyring.addFromUri('//Ferdie//stash', { name: 'Ferdie-BANK' }), nonce: 0 },
    francis: { key: keyring.addFromUri('//Francis', { name: 'Francis' }), nonce: 0 },
  }

  try {
    // in order to assign a role, it must be created first
    const executeRegistrar = api.registry.createType("Role", { pallet: 'Registrar', permission: 'Execute' });
    //submit(api, api.tx.rbac.createRole(`Registrar`, 'Execute'), users.admin);
    const executeProductRegistry = api.registry.createType("Role", { pallet: 'RawMaterials', permission: 'Execute' });
    submit(api, api.tx.rbac.createRole(`RawMaterials`, 'Execute'), users.admin);
    // const executeProductTracking = api.registry.createType("Role", { pallet: 'ProductTracking', permission: 'Execute' });
    // submit(api, api.tx.rbac.createRole(`ProductTracking`, 'Execute'), users.admin);
    // const executeBalances = api.registry.createType("Role", { pallet: 'Balances', permission: 'Execute' });
    // submit(api, api.tx.rbac.createRole(`Balances`, 'Execute'), users.admin);

    const second = 1000;
    const block = 6.5 * second;
    const minute = 60 * second;
    const hour = 60 * minute;
    const day = 24 * hour;
    await new Promise(r => setTimeout(r, block));

    // assign roles
    submit(api, api.tx.rbac.assignRole(users.bob.key.address, executeRegistrar), users.admin);
    submit(api, api.tx.rbac.assignRole(users.bob.key.address, executeProductRegistry), users.admin);

    submit(api, api.tx.rbac.assignRole(users.charlie.key.address, executeRegistrar), users.admin);
    submit(api, api.tx.rbac.assignRole(users.charlie.key.address, executeProductRegistry), users.admin);
    submit(api, api.tx.rbac.assignRole(users.clarice.key.address, executeProductRegistry), users.admin);
    submit(api, api.tx.rbac.assignRole(users.dave.key.address, executeRegistrar), users.admin);
    submit(api, api.tx.rbac.assignRole(users.dave.key.address, executeProductRegistry), users.admin);
    submit(api, api.tx.rbac.assignRole(users.daisy.key.address, executeProductRegistry), users.admin);

    submit(api, api.tx.rbac.assignRole(users.eve.key.address, executeRegistrar), users.admin);
    submit(api, api.tx.rbac.assignRole(users.eve.key.address, executeProductRegistry), users.admin);
    submit(api, api.tx.rbac.assignRole(users.erowid.key.address, executeProductRegistry), users.admin);

    submit(api, api.tx.rbac.assignRole(users.ferdie.key.address, executeRegistrar), users.admin);
    submit(api, api.tx.rbac.assignRole(users.ferdie.key.address, executeProductRegistry), users.admin);
    submit(api, api.tx.rbac.assignRole(users.francis.key.address, executeProductRegistry), users.admin);

    await new Promise(r => setTimeout(r, block));

    const salary = 100_000_000_000_000;

    // create organizations & add members
    submit(api, api.tx.registrar.createOrganization(`Bob's Burgers`), users.bob);
    submit(api, api.tx.registrar.addToOrganization(users.betty.key.address), users.bob);
    submit(api, api.tx.balances.transfer(users.betty.key.address, salary), users.bobBank);

    submit(api, api.tx.registrar.createOrganization(`Charlie's Cheese`), users.charlie);
    submit(api, api.tx.registrar.addToOrganization(users.clarice.key.address), users.charlie);
    submit(api, api.tx.balances.transfer(users.clarice.key.address, salary), users.charlieBank);

    submit(api, api.tx.registrar.createOrganization(`Dave's Dough`), users.dave);
    submit(api, api.tx.registrar.addToOrganization(users.daisy.key.address), users.dave);
    submit(api, api.tx.balances.transfer(users.daisy.key.address, salary), users.daveBank);

    submit(api, api.tx.registrar.createOrganization(`Eve's Leaves`), users.eve);
    submit(api, api.tx.registrar.addToOrganization(users.erowid.key.address), users.eve);
    submit(api, api.tx.balances.transfer(users.erowid.key.address, salary), users.eveBank);

    submit(api, api.tx.registrar.createOrganization(`Ferdie's Flowers`), users.ferdie);
    submit(api, api.tx.registrar.addToOrganization(users.francis.key.address), users.ferdie);
    submit(api, api.tx.balances.transfer(users.francis.key.address, salary), users.ferdieBank);

    await new Promise(r => setTimeout(r, block));

    // create products
    const beef = uuidv4();
    submit(api, api.tx.rawMaterials.registerRawMaterial(beef, "UnRoasted", users.bob.key.address, undefined, 500), users.betty);
    const veggie = uuidv4();


    await new Promise(r => setTimeout(r, block));
  } catch (e) {
    throw e;
  }
}

main().catch(console.error).finally(() => process.exit());
