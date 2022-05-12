import { config } from "dotenv";
config();
import { STAKINGREWARDSFACTORYClient ,utils} from "../src";
import { getDeploy } from "./utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
  Contracts,
  CLByteArray
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH,
  STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT,
  STAKINGREWARDSFACTORY_CONTRACT,
  STAKINGREWARDSFACTORY_CONTRACT_PACKAGE,
  DUALREWARDSDISTRIBUTION,
  STAKING_TOKEN,
  REWARD_TOKEN_A,
  REWARD_TOKEN_B,
  TOKEN
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const stakingrewardsfactory = new STAKINGREWARDSFACTORYClient(
  NODE_ADDRESS!,
  CHAIN_NAME!,
  EVENT_STREAM_ADDRESS!
);

const test = async () => {

  await stakingrewardsfactory.setContractHash(STAKINGREWARDSFACTORY_CONTRACT!);

  // //totalsupply
  let totalSupply = await stakingrewardsfactory.totalSupply();
  console.log(`... Total supply: ${totalSupply}`);

  // //balanceof
  let balance = await stakingrewardsfactory.balanceOf("24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1");
  console.log(`... Balance: ${balance}`);

  //nominateNewOwner
  const nominateNewOwnerDeployHash = await stakingrewardsfactory.nominateNewOwner(
    KEYS,
    "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... nominateNewOwner deploy hash: ", nominateNewOwnerDeployHash);

  await getDeploy(NODE_ADDRESS!, nominateNewOwnerDeployHash);
  console.log("... nominateNewOwner called successfully");

  //acceptOwnership
  const acceptOwnershipDeployHash = await stakingrewardsfactory.acceptOwnership(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... acceptOwnership deploy hash: ", acceptOwnershipDeployHash);

  await getDeploy(NODE_ADDRESS!, acceptOwnershipDeployHash);
  console.log("... acceptOwnership called successfully");

  //deploy
  const deployDeployHash = await stakingrewardsfactory.deploy(
    KEYS,
    "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
    STAKING_TOKEN!,
    REWARD_TOKEN_A!,
    REWARD_TOKEN_B!,
    "1000000000",
    "1000000000",
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... deploy deploy hash: ", deployDeployHash);

  await getDeploy(NODE_ADDRESS!, deployDeployHash);
  console.log("... deploy called successfully");

  //update
  const updateDeployHash = await stakingrewardsfactory.update(
    KEYS,
    STAKING_TOKEN!,
    "1000000000",
    "1000000000",
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... update deploy hash: ", updateDeployHash);

  await getDeploy(NODE_ADDRESS!, updateDeployHash);
  console.log("... update called successfully");

  //notifyRewardAmounts
  const notifyRewardAmountsDeployHash = await stakingrewardsfactory.notifyRewardAmounts(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... notifyRewardAmounts deploy hash: ", notifyRewardAmountsDeployHash);

  await getDeploy(NODE_ADDRESS!, notifyRewardAmountsDeployHash);
  console.log("... notifyRewardAmounts called successfully");

  //notifyRewardAmount
  const notifyRewardAmountDeployHash = await stakingrewardsfactory.notifyRewardAmount(
    KEYS,
    STAKING_TOKEN!,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... notifyRewardAmount deploy hash: ", notifyRewardAmountDeployHash);

  await getDeploy(NODE_ADDRESS!, notifyRewardAmountDeployHash);
  console.log("... notifyRewardAmount called successfully");

  //pullExtraTokens
  const pullExtraTokensDeployHash = await stakingrewardsfactory.pullExtraTokens(
    KEYS,
    TOKEN!,
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... pullExtraTokens deploy hash: ", pullExtraTokensDeployHash);

  await getDeploy(NODE_ADDRESS!, pullExtraTokensDeployHash);
  console.log("... pullExtraTokens called successfully");

  //constructorSdr
  const constructorSdrDeployHash = await stakingrewardsfactory.constructorSdr(
    KEYS,
    "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
    DUALREWARDSDISTRIBUTION!,
    STAKING_TOKEN!,
    REWARD_TOKEN_A!,
    REWARD_TOKEN_B!,
    STAKINGREWARDSFACTORY_CONTRACT,
    STAKINGREWARDSFACTORY_CONTRACT_PACKAGE,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... constructorSdr deploy hash: ", constructorSdrDeployHash);

  await getDeploy(NODE_ADDRESS!, constructorSdrDeployHash);
  console.log("... constructorSdr called successfully");

  //lastTimeRewardApplicableJsClient
  const lastTimeRewardApplicableJsClientDeployHash = await stakingrewardsfactory.lastTimeRewardApplicableJsClient(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... lastTimeRewardApplicableJsClient deploy hash: ", lastTimeRewardApplicableJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, lastTimeRewardApplicableJsClientDeployHash);
  console.log("... lastTimeRewardApplicableJsClient called successfully");

  //lastTimeRewardApplicable
  let lastTimeRewardApplicable = await stakingrewardsfactory.lastTimeRewardApplicable();
  console.log(`... lastTimeRewardApplicable : ${lastTimeRewardApplicable}`);


  //rewardPerTokenAJsClient
  const rewardPerTokenAJsClientDeployHash = await stakingrewardsfactory.rewardPerTokenAJsClient(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... rewardPerTokenAJsClient deploy hash: ", rewardPerTokenAJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, rewardPerTokenAJsClientDeployHash);
  console.log("... rewardPerTokenAJsClient called successfully");

  //rewardPerTokenA
  let rewardPerTokenA = await stakingrewardsfactory.rewardPerTokenA();
  console.log(`... rewardPerTokenA : ${rewardPerTokenA}`);


  //rewardPerTokenBJsClient
  const rewardPerTokenBJsClientDeployHash = await stakingrewardsfactory.rewardPerTokenBJsClient(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... rewardPerTokenBJsClient deploy hash: ", rewardPerTokenBJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, rewardPerTokenBJsClientDeployHash);
  console.log("... rewardPerTokenBJsClient called successfully");

  //rewardPerTokenB
  let rewardPerTokenB = await stakingrewardsfactory.rewardPerTokenB();
  console.log(`... rewardPerTokenB : ${rewardPerTokenB}`);


  //earnedAJsClient
  const earnedAJsClientDeployHash = await stakingrewardsfactory.earnedAJsClient(
    KEYS,
    "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... earnedAJsClient deploy hash: ", earnedAJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, earnedAJsClientDeployHash);
  console.log("... earnedAJsClient called successfully");

  //earnedA
  let earnedA = await stakingrewardsfactory.earnedA();
  console.log(`... earnedA : ${earnedA}`);

  //earnedBJsClient
  const earnedBJsClientDeployHash = await stakingrewardsfactory.earnedBJsClient(
    KEYS,
    "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... earnedBJsClient deploy hash: ", earnedBJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, earnedBJsClientDeployHash);
  console.log("... earnedBJsClient called successfully");

  //earnedB
  let earnedB = await stakingrewardsfactory.earnedB();
  console.log(`... earnedB : ${earnedB}`);


  //stake
  const stakeDeployHash = await stakingrewardsfactory.stake(
    KEYS,
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... stake deploy hash: ", stakeDeployHash);

  await getDeploy(NODE_ADDRESS!, stakeDeployHash);
  console.log("... stake called successfully");


  //withdraw
  const withdrawDeployHash = await stakingrewardsfactory.withdraw(
    KEYS,
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... withdraw deploy hash: ", withdrawDeployHash);

  await getDeploy(NODE_ADDRESS!, withdrawDeployHash);
  console.log("... withdraw called successfully");

  //getReward
  const getRewardDeployHash = await stakingrewardsfactory.getReward(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... getReward deploy hash: ", getRewardDeployHash);

  await getDeploy(NODE_ADDRESS!, getRewardDeployHash);
  console.log("... getReward called successfully");

  //exit
  const exitDeployHash = await stakingrewardsfactory.exit(
    KEYS,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... exit deploy hash: ", exitDeployHash);

  await getDeploy(NODE_ADDRESS!, exitDeployHash);
  console.log("... exit called successfully");

  //notifyRewardAmountSdr
  const notifyRewardAmountSdrDeployHash = await stakingrewardsfactory.notifyRewardAmountSdr(
    KEYS,
    "1000000000",
    "1000000000",
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... notifyRewardAmountSdr deploy hash: ", notifyRewardAmountSdrDeployHash);

  await getDeploy(NODE_ADDRESS!, notifyRewardAmountSdrDeployHash);
  console.log("... notifyRewardAmountSdr called successfully");

  //recoverErc20
  const recoverErc20DeployHash = await stakingrewardsfactory.recoverErc20(
    KEYS,
    TOKEN!,
    "1000000000",
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... recoverErc20 deploy hash: ", recoverErc20DeployHash);

  await getDeploy(NODE_ADDRESS!, recoverErc20DeployHash);
  console.log("... recoverErc20 called successfully");

  //paused
  const pausedDeployHash = await stakingrewardsfactory.paused(
    KEYS,
    true,
    STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... paused deploy hash: ", pausedDeployHash);

  await getDeploy(NODE_ADDRESS!, pausedDeployHash);
  console.log("... paused called successfully");

};

test();

