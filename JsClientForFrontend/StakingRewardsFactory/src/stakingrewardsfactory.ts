import {
  CasperClient,
  CLPublicKey,
  CLAccountHash,
  CLByteArray,
  CLKey,
  CLString,
  CLTypeBuilder,
  CLValue,
  CLValueBuilder,
  CLValueParsers,
  CLMap,
  DeployUtil,
  EventName,
  EventStream,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";
import { Some, None } from "ts-results";
import * as blake from "blakejs";
import { concat } from "@ethersproject/bytes";
import * as utils from "./utils";
import { RecipientType, IPendingDeploy } from "./types";
import {createRecipientAddress } from "./utils";
import { consoleTestResultHandler } from "tslint/lib/test";

class STAKINGREWARDSFACTORYClient {
  private contractName: string = "stakingrewardsfactory";
  private contractHash: string= "stakingrewardsfactory";
  private contractPackageHash: string= "stakingrewardsfactory";
  private namedKeys: {
    balances:string
  };

  constructor(

    private nodeAddress: string,
    private chainName: string,
    private eventStreamAddress?: string,
    
  ) 
  {
    this.namedKeys= {
      balances:"null",
    }; 
  }

  public async setContractHash(hash: string) {
    const stateRootHash = await utils.getStateRootHash(this.nodeAddress);
    const contractData = await utils.getContractData(
      this.nodeAddress,
      stateRootHash,
      hash
    );

    const { contractPackageHash, namedKeys } = contractData.Contract!;
    this.contractHash = hash;
    this.contractPackageHash = contractPackageHash.replace(
      "contract-package-wasm",
      ""
    );
    const LIST_OF_NAMED_KEYS = [
      'balances',
      `${this.contractName}_package_hash`,
      `${this.contractName}_package_hash_wrapped`,
      `${this.contractName}_contract_hash`,
      `${this.contractName}_contract_hash_wrapped`,
      `${this.contractName}_package_access_token`,
    ];
    // @ts-ignore
    this.namedKeys = namedKeys.reduce((acc, val) => {
      if (LIST_OF_NAMED_KEYS.includes(val.name)) {
        return { ...acc, [utils.camelCased(val.name)]: val.key };
      }
      return acc;
    }, {});
  }

  public async totalSupply() {
    try {
      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["total_supply"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async balanceOf(account: string) {
    try {
      
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        account,
        this.namedKeys.balances
      );
      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();

    } catch (error) {
      return "0";
    }
    
  }

  public async nominateNewOwner(
    keys: Keys.AsymmetricKey,
    owner:  string,
    paymentAmount: string
  ) {

    const _owner = new CLByteArray(
			Uint8Array.from(Buffer.from(owner, "hex"))
		);

    const runtimeArgs = RuntimeArgs.fromMap({
      owner: utils.createRecipientAddress(_owner),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "nominate_new_owner",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }
  
  public async acceptOwnership(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {

    const runtimeArgs = RuntimeArgs.fromMap({});

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "accept_ownership",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async deploy(
    keys: Keys.AsymmetricKey,
    owner:  string,
    stakingToken:  string,
    rewardsTokenA:  string,
    rewardsTokenB:  string,
    rewardAmountA:  string,
    rewardAmountB:  string,
    rewardDuration:  string,
    paymentAmount: string
  ) {

    const _owner = new CLByteArray(
			Uint8Array.from(Buffer.from(owner, "hex"))
		);
    const _stakingToken = new CLByteArray(
			Uint8Array.from(Buffer.from(stakingToken, "hex"))
		);
		const _rewardsTokenA = new CLByteArray(
			Uint8Array.from(Buffer.from(rewardsTokenA, "hex"))
		);
    const _rewardsTokenB = new CLByteArray(
			Uint8Array.from(Buffer.from(rewardsTokenB, "hex"))
		);

    const runtimeArgs = RuntimeArgs.fromMap({
      owner: utils.createRecipientAddress(_owner),
      staking_token: new CLKey(_stakingToken),
      rewards_token_a: new CLKey(_rewardsTokenA),
			rewards_token_b: new CLKey(_rewardsTokenB),
      reward_amount_a: CLValueBuilder.u256(rewardAmountA),
      reward_amount_b: CLValueBuilder.u256(rewardAmountB),
      rewards_duration: CLValueBuilder.u256(rewardDuration),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "deploy",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }
  public async getStakingDualRewardsContractHash() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["StakingDualRewards0_contract"]
    );
    return result.value();
  }

  public async update(
    keys: Keys.AsymmetricKey,
    stakingToken:  string,
    rewardAmountA:  string,
    rewardAmountB:  string,
    rewardDuration:  string,
    paymentAmount: string
  ) {
  
    const _stakingToken = new CLByteArray(
			Uint8Array.from(Buffer.from(stakingToken, "hex"))
		);
		
    const runtimeArgs = RuntimeArgs.fromMap({
      staking_token: new CLKey(_stakingToken),
      reward_amount_a: CLValueBuilder.u256(rewardAmountA),
      reward_amount_b: CLValueBuilder.u256(rewardAmountB),
      rewards_duration: CLValueBuilder.u256(rewardDuration),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "update",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async notifyRewardAmounts(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
  
		
    const runtimeArgs = RuntimeArgs.fromMap({});

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "notify_reward_amounts",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async notifyRewardAmount(
    keys: Keys.AsymmetricKey,
    stakingToken:  string,
    paymentAmount: string
  ) {
  
    const _stakingToken = new CLByteArray(
			Uint8Array.from(Buffer.from(stakingToken, "hex"))
		);
		
    const runtimeArgs = RuntimeArgs.fromMap({
      staking_token: new CLKey(_stakingToken),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "notify_reward_amount",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }
  
  public async pullExtraTokens(
    keys: Keys.AsymmetricKey,
    token:  string,
    amount: string,
    paymentAmount: string
  ) {
  
    const _token = new CLByteArray(
			Uint8Array.from(Buffer.from(token, "hex"))
		);
		
    const runtimeArgs = RuntimeArgs.fromMap({
      token: new CLKey(_token),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "pull_extra_tokens",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async constructorSdr(
    keys: Keys.AsymmetricKey,
    owner:  string,
    dualRewardsDistribution: string,
    stakingToken:  string,
    rewardsTokenA:  string,
    rewardsTokenB:  string,
    contractHash:  string,
    packageHash:  string,
    paymentAmount: string
  ) {

    const _owner = new CLByteArray(
			Uint8Array.from(Buffer.from(owner, "hex"))
		);
    const _dualRewardsDistribution = new CLByteArray(
			Uint8Array.from(Buffer.from(dualRewardsDistribution, "hex"))
		);
    const _stakingToken = new CLByteArray(
			Uint8Array.from(Buffer.from(stakingToken, "hex"))
		);
		const _rewardsTokenA = new CLByteArray(
			Uint8Array.from(Buffer.from(rewardsTokenA, "hex"))
		);
    const _rewardsTokenB = new CLByteArray(
			Uint8Array.from(Buffer.from(rewardsTokenB, "hex"))
		);
    const _contractHash = new CLByteArray(
			Uint8Array.from(Buffer.from(contractHash, "hex"))
		);
    const _packageHash = new CLByteArray(
			Uint8Array.from(Buffer.from(packageHash, "hex"))
		);

    const runtimeArgs = RuntimeArgs.fromMap({
      owner: utils.createRecipientAddress(_owner),
      dual_rewards_distribution: new CLKey(_dualRewardsDistribution),
      staking_token: new CLKey(_stakingToken),
      rewards_token_a: new CLKey(_rewardsTokenA),
			rewards_token_b: new CLKey(_rewardsTokenB),
      contract_hash: new CLKey(_contractHash),// issue (it should be contractHash)
			package_hash: new CLKey(_packageHash),// issue (it should be contractPackageHash)
    });
 
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "constructor_sdr",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async periodFinish() {
    try{

      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["period_finish"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async lastTimeRewardApplicable(
    blockTimeStampInMiliSeconds:number,
  ) {

    let periodFinishResult= await this.periodFinish();
    console.log("periodFinishResult: ",`${periodFinishResult}`);

    if(blockTimeStampInMiliSeconds < periodFinishResult)
    {
      return blockTimeStampInMiliSeconds;
    }
    else{
      return periodFinishResult;
    }

  }
  
  public async getRewardPerTokenAStored() {
    try{

      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["reward_per_token_a_stored"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async getLastUpdateTime() {
    try{

      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["last_update_time"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async getRewardRateA() {
    try{

      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["reward_rate_a"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async rewardPerTokenA(
    blockTimeStampInMiliSeconds:number,
  ) {
    
    let totalSupplyResult = await this.totalSupply();
    console.log("totalSupply: ",`${totalSupplyResult}`);

    if(totalSupplyResult == 0)
    {
      return await this.getRewardPerTokenAStored();
    }
    else{
      let getRewardPerTokenAStoredResult =await this.getRewardPerTokenAStored();
      let lastTimeRewardApplicableResult =await this.lastTimeRewardApplicable(blockTimeStampInMiliSeconds);
      let getLastUpdateTimeResult =await this.getLastUpdateTime();
      let getRewardRateAResult =await this.getRewardRateA();
      let TenENine = 1000000000;
      let totalSupplyResult =await this.totalSupply();

      console.log("getRewardPerTokenAStoredResult: ",`${getRewardPerTokenAStoredResult}`);
      console.log("lastTimeRewardApplicableResult: ",`${lastTimeRewardApplicableResult}`);
      console.log("getLastUpdateTimeResult: ",`${getLastUpdateTimeResult}`);
      console.log("getRewardRateAResult: ",`${getRewardRateAResult}`);
      console.log("totalSupplyResult: ",`${totalSupplyResult}`);

      if(lastTimeRewardApplicableResult > getLastUpdateTimeResult)
      {
        return getRewardPerTokenAStoredResult + ((((lastTimeRewardApplicableResult-getLastUpdateTimeResult)*getRewardRateAResult)*TenENine)/totalSupplyResult);
      }
      else{
        return getRewardPerTokenAStoredResult + ((((getLastUpdateTimeResult-lastTimeRewardApplicableResult)*getRewardRateAResult)*TenENine)/totalSupplyResult);
      }
      
    }
  }

  public async getRewardPerTokenBStored() {
    try{

      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["reward_per_token_b_stored"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async getRewardRateB() {
    try{

      const result = await contractSimpleGetter(
        this.nodeAddress,
        this.contractHash,
        ["reward_rate_b"]
      );
      return result.value();
    } catch (error) {
      return "0";
    }
  }

  public async rewardPerTokenB(
    blockTimeStampInMiliSeconds:number,
  ) {
    
    let totalSupplyResult = await this.totalSupply();
    console.log("totalSupply: ",`${totalSupplyResult}`);

    if(totalSupplyResult == 0)
    {
      return await this.getRewardPerTokenBStored();
    }
    else{
      let getRewardPerTokenBStoredResult =await this.getRewardPerTokenBStored();
      let lastTimeRewardApplicableResult =await this.lastTimeRewardApplicable(blockTimeStampInMiliSeconds);
      let getLastUpdateTimeResult =await this.getLastUpdateTime();
      let getRewardRateBResult =await this.getRewardRateB();
      let TenENine = 1000000000;
      let totalSupplyResult =await this.totalSupply();

      console.log("getRewardPerTokenBStoredResult: ",`${getRewardPerTokenBStoredResult}`);
      console.log("lastTimeRewardApplicableResult: ",`${lastTimeRewardApplicableResult}`);
      console.log("getLastUpdateTimeResult: ",`${getLastUpdateTimeResult}`);
      console.log("getRewardRateBResult: ",`${getRewardRateBResult}`);
      console.log("totalSupplyResult: ",`${totalSupplyResult}`);

      if(lastTimeRewardApplicableResult > getLastUpdateTimeResult)
      {
        return getRewardPerTokenBStoredResult + ((((lastTimeRewardApplicableResult-getLastUpdateTimeResult)*getRewardRateBResult)*TenENine)/totalSupplyResult);
      }
      else{
        return getRewardPerTokenBStoredResult + ((((getLastUpdateTimeResult-lastTimeRewardApplicableResult)*getRewardRateBResult)*TenENine)/totalSupplyResult);
      }
      
    }
  }

  public async userRewardPerTokenAPaid(account: string) {
    try {
      
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        account,
        "user_reward_per_token_a_paid"
      );
      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();

    } catch (error) {
      return "0";
    }
    
  }

  public async rewardsA(account: string) {
    try {
      
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        account,
        "rewards_a"
      );
      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();

    } catch (error) {
      return "0";
    }
    
  }

  public async earnedA(
    account: string,
    blockTimeStampInMiliSeconds:number,
  ) {
		let balanceResult=await this.balanceOf(account);
    let rewardPerTokenAResult=await this.rewardPerTokenA(blockTimeStampInMiliSeconds);
    let userRewardPerTokenAPaidResult=await this.userRewardPerTokenAPaid(account);
    let TenENine = 1000000000;
    let rewardsAResult=await this.rewardsA(account);

    console.log("balanceResult: ", `${balanceResult}`);
    console.log("rewardPerTokenAResult: ", `${rewardPerTokenAResult}`);
    console.log("userRewardPerTokenAPaidResult: ", `${userRewardPerTokenAPaidResult}`);
    console.log("rewardsAResult: ", `${rewardsAResult}`);

    if(rewardPerTokenAResult > userRewardPerTokenAPaidResult)
    {
      return balanceResult*(((rewardPerTokenAResult-userRewardPerTokenAPaidResult)/TenENine)+rewardsAResult);
    }
    else{
      return balanceResult*(((userRewardPerTokenAPaidResult-rewardPerTokenAResult)/TenENine)+rewardsAResult);
    }
  }

  public async userRewardPerTokenBPaid(account: string) {
    try {
      
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        account,
        "user_reward_per_token_b_paid"
      );
      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();

    } catch (error) {
      return "0";
    }
    
  }
  public async rewardsB(account: string) {
    try {
      
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        account,
        "rewards_b"
      );
      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();

    } catch (error) {
      return "0";
    }
    
  }

  public async earnedB(
    account: string,
    blockTimeStampInMiliSeconds:number,
  ) {
		let balanceResult=await this.balanceOf(account);
    let rewardPerTokenBResult=await this.rewardPerTokenB(blockTimeStampInMiliSeconds);
    let userRewardPerTokenBPaidResult=await this.userRewardPerTokenBPaid(account);
    let TenENine = 1000000000;
    let rewardsBResult=await this.rewardsB(account);

    console.log("balanceResult: ", `${balanceResult}`);
    console.log("rewardPerTokenBResult: ", `${rewardPerTokenBResult}`);
    console.log("userRewardPerTokenBPaidResult: ", `${userRewardPerTokenBPaidResult}`);
    console.log("rewardsBResult: ", `${rewardsBResult}`);

    if(rewardPerTokenBResult > userRewardPerTokenBPaidResult)
    {
      return balanceResult*(((rewardPerTokenBResult-userRewardPerTokenBPaidResult)/TenENine)+rewardsBResult);
    }
    else{
      return balanceResult*(((userRewardPerTokenBPaidResult-rewardPerTokenBResult)/TenENine)+rewardsBResult);
    }
  }

  public async stake(
    keys: Keys.AsymmetricKey,
    amount: string,
    paymentAmount: string
  ) {
		

    const runtimeArgs = RuntimeArgs.fromMap({
      amount: CLValueBuilder.u256(amount)
    });
    
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "stake",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async withdraw(
    keys: Keys.AsymmetricKey,
    amount: string,
    paymentAmount: string
  ) {
		

    const runtimeArgs = RuntimeArgs.fromMap({
      amount: CLValueBuilder.u256(amount)
    });
    
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "withdraw",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async getReward(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
		

    const runtimeArgs = RuntimeArgs.fromMap({
    });
    
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "get_reward",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }
  
  public async exit(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
		

    const runtimeArgs = RuntimeArgs.fromMap({
    });
    
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "exit",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async notifyRewardAmountSdr(
    keys: Keys.AsymmetricKey,
    rewardA:  string,
    rewardB:  string,
    rewardDuration:  string,
    paymentAmount: string
  ) {
  
    const runtimeArgs = RuntimeArgs.fromMap({
      reward_a: CLValueBuilder.u256(rewardA),
      reward_b: CLValueBuilder.u256(rewardB),
      rewards_duration: CLValueBuilder.u256(rewardDuration),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "notify_reward_amount_sdr",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async recoverErc20(
    keys: Keys.AsymmetricKey,
    tokenAddress:  string,
    tokenAmount: string,
    paymentAmount: string
  ) {
  
    const _tokenAddress = new CLByteArray(
			Uint8Array.from(Buffer.from(tokenAddress, "hex"))
		);
		
    const runtimeArgs = RuntimeArgs.fromMap({
      token_address: new CLKey(_tokenAddress),
      token_amount: CLValueBuilder.u256(tokenAmount),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "recover_erc20",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async setPaused(
    keys: Keys.AsymmetricKey,
    paused:  boolean,
    paymentAmount: string
  ) {
  
    
    const runtimeArgs = RuntimeArgs.fromMap({
      paused: CLValueBuilder.bool(paused),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "set_paused",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

}
interface IContractCallParams {
  nodeAddress: string;
  keys: Keys.AsymmetricKey;
  chainName: string;
  entryPoint: string;
  runtimeArgs: RuntimeArgs;
  paymentAmount: string;
  contractHash: string;
}

const contractCall = async ({
  nodeAddress,
  keys,
  chainName,
  contractHash,
  entryPoint,
  runtimeArgs,
  paymentAmount,
}: IContractCallParams) => {
  const client = new CasperClient(nodeAddress);
  const contractHashAsByteArray = utils.contractHashToByteArray(contractHash);

  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(keys.publicKey, chainName),
    DeployUtil.ExecutableDeployItem.newStoredContractByHash(
      contractHashAsByteArray,
      entryPoint,
      runtimeArgs
    ),
    DeployUtil.standardPayment(paymentAmount)
  );

  // Sign deploy.
  deploy = client.signDeploy(deploy, keys);

  // Dispatch deploy to node.
  const deployHash = await client.putDeploy(deploy);

  return deployHash;
};

const contractSimpleGetter = async (
  nodeAddress: string,
  contractHash: string,
  key: string[]
) => {
  const stateRootHash = await utils.getStateRootHash(nodeAddress);
  const clValue = await utils.getContractData(
    nodeAddress,
    stateRootHash,
    contractHash,
    key
  );

  if (clValue && clValue.CLValue instanceof CLValue) {
    return clValue.CLValue!;
  } else {
    throw Error("Invalid stored value");
  }
};

const toCLMap = (map: Map<string, string>) => {
  const clMap = CLValueBuilder.map([
    CLTypeBuilder.string(),
    CLTypeBuilder.string(),
  ]);
  for (const [key, value] of Array.from(map.entries())) {
    clMap.set(CLValueBuilder.string(key), CLValueBuilder.string(value));
  }
  return clMap;
};

const fromCLMap = (map: Map<CLString, CLString>) => {
  const jsMap = new Map();
  for (const [key, value] of Array.from(map.entries())) {
    jsMap.set(key.value(), value.value());
  }
  return jsMap;
};

export default STAKINGREWARDSFACTORYClient;
