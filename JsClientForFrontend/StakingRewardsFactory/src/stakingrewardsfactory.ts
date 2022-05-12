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

class STAKINGREWARDSFACTORYClient {
  private contractName: string = "stakingrewardsfactory";
  private contractHash: string= "stakingrewardsfactory";
  private contractPackageHash: string= "stakingrewardsfactory";
  private namedKeys: {
    balances:string,
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
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["total_supply"]
    );
    return result.value();
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

  public async lastTimeRewardApplicableJsClient(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
		
    const runtimeArgs = RuntimeArgs.fromMap({});

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "last_time_reward_applicable_Jsclient",
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
  public async lastTimeRewardApplicable() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["last_time_reward_applicable"]
    );
    return result.value();
  }

  public async rewardPerTokenAJsClient(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
		
    const runtimeArgs = RuntimeArgs.fromMap({});

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "reward_per_token_a_Jsclient",
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

  public async rewardPerTokenA() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["reward_per_token_a"]
    );
    return result.value();
  }

  public async rewardPerTokenBJsClient(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
		
    const runtimeArgs = RuntimeArgs.fromMap({});

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "reward_per_token_b_Jsclient",
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

  public async rewardPerTokenB() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["reward_per_token_b"]
    );
    return result.value();
  }

  public async earnedAJsClient(
    keys: Keys.AsymmetricKey,
    account: string,
    paymentAmount: string
  ) {
		
    const _account = new CLByteArray(
			Uint8Array.from(Buffer.from(account, "hex"))
		);

    const runtimeArgs = RuntimeArgs.fromMap({
      account: utils.createRecipientAddress(_account)
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "earned_a_Jsclient",
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

  public async earnedA() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["earned_a"]
    );
    return result.value();
  }

  public async earnedBJsClient(
    keys: Keys.AsymmetricKey,
    account: string,
    paymentAmount: string
  ) {
		
    const _account = new CLByteArray(
			Uint8Array.from(Buffer.from(account, "hex"))
		);

    const runtimeArgs = RuntimeArgs.fromMap({
      account: utils.createRecipientAddress(_account)
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "earned_b_Jsclient",
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

  public async earnedB() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["earned_b"]
    );
    return result.value();
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

  public async paused(
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
      entryPoint: "paused",
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
