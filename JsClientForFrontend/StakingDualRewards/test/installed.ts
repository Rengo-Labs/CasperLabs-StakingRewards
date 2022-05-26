import { config } from "dotenv";
config();
import { STAKINGDUALREWARDSClient} from "../src";
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
  STAKINGDUALREWARDS_MASTER_KEY_PAIR_PATH,
  STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT,
  STAKINGDUALREWARDS_CONTRACT,
  STAKINGDUALREWARDS_CONTRACT_PACKAGE,
  DUALREWARDSDISTRIBUTION,
  STAKING_TOKEN,
  REWARD_TOKEN_A,
  REWARD_TOKEN_B,
  TOKEN
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${STAKINGDUALREWARDS_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${STAKINGDUALREWARDS_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const stakingdualrewards = new STAKINGDUALREWARDSClient(
  NODE_ADDRESS!,
  CHAIN_NAME!,
  EVENT_STREAM_ADDRESS!
);

const test = async () => {

  await stakingdualrewards.setContractHash(STAKINGDUALREWARDS_CONTRACT!);

  //totalsupply
  // let totalSupply = await stakingdualrewards.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // //balanceof
  // let balance = await stakingdualrewards.balanceOf("24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1");
  // console.log(`... Balance: ${balance}`);

  // //constructorSdr
  // const constructorSdrDeployHash = await stakingdualrewards.constructorSdr(
  //   KEYS,
  //   "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
  //   DUALREWARDSDISTRIBUTION!,
  //   STAKING_TOKEN!,
  //   REWARD_TOKEN_A!,
  //   REWARD_TOKEN_B!,
  //   STAKINGDUALREWARDS_CONTRACT,
  //   STAKINGDUALREWARDS_CONTRACT_PACKAGE,
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... constructorSdr deploy hash: ", constructorSdrDeployHash);

  // await getDeploy(NODE_ADDRESS!, constructorSdrDeployHash);
  // console.log("... constructorSdr called successfully");

  //lastTimeRewardApplicable
  console.log("Calling lastTimeRewardApplicable Function: \n");
  
  let lastTimeRewardApplicable = await stakingdualrewards.lastTimeRewardApplicable(1653471616000);
  console.log(`... lastTimeRewardApplicable : ${lastTimeRewardApplicable}`);
  console.log("\n");

  // //rewardPerTokenA
  console.log("Calling rewardPerTokenA Function: \n");
  let rewardPerTokenA = await stakingdualrewards.rewardPerTokenA(1653471616000);
  console.log(`... rewardPerTokenA : ${rewardPerTokenA}`);
  console.log("\n");

  // //rewardPerTokenB
  console.log("Calling rewardPerTokenB Function: \n");
  let rewardPerTokenB = await stakingdualrewards.rewardPerTokenB(1653471616000);
  console.log(`... rewardPerTokenB : ${rewardPerTokenB}`);
  console.log("\n");

  // //earnedA
  console.log("Calling earnedA Function: \n");
  let earnedA = await stakingdualrewards.earnedA("24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",1653471616000);
  console.log(`... earnedA : ${earnedA}`);
  console.log("\n"); 

  //earnedB
  console.log("Calling earnedB Function: \n");
  let earnedB = await stakingdualrewards.earnedB("24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",1653471616000);
  console.log(`... earnedB : ${earnedB}`);
  console.log("\n");

  // //stake
  // const stakeDeployHash = await stakingdualrewards.stake(
  //   KEYS,
  //   "20000000000",
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... stake deploy hash: ", stakeDeployHash);

  // await getDeploy(NODE_ADDRESS!, stakeDeployHash);
  // console.log("... stake called successfully");


  // //withdraw
  // //Flow: Call stake first
  // const withdrawDeployHash = await stakingdualrewards.withdraw(
  //   KEYS,
  //   "10000000000",
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... withdraw deploy hash: ", withdrawDeployHash);

  // await getDeploy(NODE_ADDRESS!, withdrawDeployHash);
  // console.log("... withdraw called successfully");

  // //getReward
  // const getRewardDeployHash = await stakingdualrewards.getReward(
  //   KEYS,
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... getReward deploy hash: ", getRewardDeployHash);

  // await getDeploy(NODE_ADDRESS!, getRewardDeployHash);
  // console.log("... getReward called successfully");

  // //exit
  // //Flow: Call stake first
  // const exitDeployHash = await stakingdualrewards.exit(
  //   KEYS,
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... exit deploy hash: ", exitDeployHash);

  // await getDeploy(NODE_ADDRESS!, exitDeployHash);
  // console.log("... exit called successfully");

  // //paused
  // const pausedDeployHash = await stakingdualrewards.setPaused(
  //   KEYS,
  //   true,
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... paused deploy hash: ", pausedDeployHash);

  // await getDeploy(NODE_ADDRESS!, pausedDeployHash);
  // console.log("... paused called successfully");
  
  // //notifyRewardAmountSdr
  // //Flow: Call stake first
  // const notifyRewardAmountSdrDeployHash = await stakingdualrewards.notifyRewardAmountSdr(
  //   KEYS,
  //   "1000000000000",
  //   "1000000000000",
  //   "100",
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... notifyRewardAmountSdr deploy hash: ", notifyRewardAmountSdrDeployHash);

  // await getDeploy(NODE_ADDRESS!, notifyRewardAmountSdrDeployHash);
  // console.log("... notifyRewardAmountSdr called successfully");

  // //recoverErc20
  // //Flow: Call stake first
  // const recoverErc20DeployHash = await stakingdualrewards.recoverErc20(
  //   KEYS,
  //   TOKEN!,
  //   "1000000000",
  //   STAKINGDUALREWARDS_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... recoverErc20 deploy hash: ", recoverErc20DeployHash);

  // await getDeploy(NODE_ADDRESS!, recoverErc20DeployHash);
  // console.log("... recoverErc20 called successfully");

};

test();

