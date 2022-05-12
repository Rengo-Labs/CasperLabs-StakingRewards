import { config } from "dotenv";
config();
import { STAKINGREWARDSFACTORYClient, utils} from "../src";
import { getDeploy } from "./utils";

import {
  Keys,
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  STAKINGREWARDSFACTORY_WASM_PATH,
  STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH,
  STAKINGREWARDSFACTORY_INSTALL_PAYMENT_AMOUNT,
  STAKINGREWARDSFACTORY_CONTRACT_NAME,
  STAKING_REWARDS_GENESIS
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const test = async () => {
  const stakingrewardsfactory = new STAKINGREWARDSFACTORYClient(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  const installDeployHash = await stakingrewardsfactory.install(
    KEYS,
    STAKING_REWARDS_GENESIS!,
    STAKINGREWARDSFACTORY_CONTRACT_NAME!,
    STAKINGREWARDSFACTORY_INSTALL_PAYMENT_AMOUNT!,
    STAKINGREWARDSFACTORY_WASM_PATH!
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${STAKINGREWARDSFACTORY_CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);

  const packageHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${STAKINGREWARDSFACTORY_CONTRACT_NAME!}_package_hash`
  );

  console.log(`... Package Hash: ${packageHash}`);
};

test();
