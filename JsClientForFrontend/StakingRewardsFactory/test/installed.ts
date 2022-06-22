import { config } from "dotenv";
config();
import { STAKINGREWARDSFACTORYClient} from "../src";
import { getDeploy } from "./utils";

import {
  Keys,
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  STAKINGREWARDSFACTORY_MASTER_KEY_PAIR_PATH,
  STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT,
  STAKINGREWARDSFACTORY_DEPLOY_FUNCTION_PAYMENT_AMOUNT,
  STAKINGREWARDSFACTORY_CONTRACT,
  STAKING_TOKEN,
  REWARD_TOKEN_A,
  REWARD_TOKEN_B,
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

  //nominateNewOwner
  // const nominateNewOwnerDeployHash = await stakingrewardsfactory.nominateNewOwner(
  //   KEYS,
  //   "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
  //   STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... nominateNewOwner deploy hash: ", nominateNewOwnerDeployHash);

  // await getDeploy(NODE_ADDRESS!, nominateNewOwnerDeployHash);
  // console.log("... nominateNewOwner called successfully");

  //acceptOwnership
  // const acceptOwnershipDeployHash = await stakingrewardsfactory.acceptOwnership(
  //   KEYS,
  //   STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... acceptOwnership deploy hash: ", acceptOwnershipDeployHash);

  // await getDeploy(NODE_ADDRESS!, acceptOwnershipDeployHash);
  // console.log("... acceptOwnership called successfully");

  // //deploy
  // const deployDeployHash = await stakingrewardsfactory.deploy(
  //   KEYS,
  //   "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
  //   STAKING_TOKEN!,
  //   REWARD_TOKEN_A!,
  //   REWARD_TOKEN_B!,
  //   "10000000000",
  //   "10000000000",
  //   "2000000000",
  //   STAKINGREWARDSFACTORY_DEPLOY_FUNCTION_PAYMENT_AMOUNT!
  // );
  // console.log("... deploy deploy hash: ", deployDeployHash);

  // await getDeploy(NODE_ADDRESS!, deployDeployHash);
  // console.log("... deploy called successfully");

  // //getStakingDualRewardsData
  // const getStakingDualRewardsData = await stakingrewardsfactory.getStakingDualRewardsContractHash(STAKING_TOKEN!);
  // console.log("... getStakingDualRewardsData: ", `${getStakingDualRewardsData}`);

  // //update
  // //Flow: Call Deploy Function First
  // const updateDeployHash = await stakingrewardsfactory.update(
  //   KEYS,
  //   STAKING_TOKEN!,
  //   "10000000000",
  //   "10000000000",
  //   "2000000000",
  //   STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... update deploy hash: ", updateDeployHash);

  // await getDeploy(NODE_ADDRESS!, updateDeployHash);
  // console.log("... update called successfully");

  // //notifyRewardAmounts
  // //Flow: Call Deploy Function First
  // //Call RewardTokenA Mint function first against stakingrewardsfactory PackageHash
  // //Call RewardTokenB Mint function first against stakingrewardsfactory PackageHash
  // const notifyRewardAmountsDeployHash = await stakingrewardsfactory.notifyRewardAmounts(
  //   KEYS,
  //   STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... notifyRewardAmounts deploy hash: ", notifyRewardAmountsDeployHash);

  // await getDeploy(NODE_ADDRESS!, notifyRewardAmountsDeployHash);
  // console.log("... notifyRewardAmounts called successfully");

  // //notifyRewardAmount
  // //Flow: Call Deploy Function First
  // //Call RewardTokenA Mint function first against stakingrewardsfactory PackageHash
  // //Call RewardTokenB Mint function first against stakingrewardsfactory PackageHash
  // const notifyRewardAmountDeployHash = await stakingrewardsfactory.notifyRewardAmount(
  //   KEYS,
  //   STAKING_TOKEN!,
  //   STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... notifyRewardAmount deploy hash: ", notifyRewardAmountDeployHash);

  // await getDeploy(NODE_ADDRESS!, notifyRewardAmountDeployHash);
  // console.log("... notifyRewardAmount called successfully");

  
  // //pullExtraTokens
  // //Flow: Call StakingToken Mint function first against stakingrewardsfactory PackageHash
  // const pullExtraTokensDeployHash = await stakingrewardsfactory.pullExtraTokens(
  //   KEYS,
  //   STAKING_TOKEN!,
  //   "10000000000",
  //   STAKINGREWARDSFACTORY_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... pullExtraTokens deploy hash: ", pullExtraTokensDeployHash);

  // await getDeploy(NODE_ADDRESS!, pullExtraTokensDeployHash);
  // console.log("... pullExtraTokens called successfully");

};

test();

