import {
  CasperServiceByJsonRPC,
  CLValue,
  CLKey,
  CLAccountHash,
  Keys,
  CLPublicKey,
} from "casper-js-sdk";

import fs from "fs";


/**
 * Returns a binary as u8 array.
 * @param pathToBinary - Path to binary file to be loaded into memory.
 * @return Uint8Array Byte array.
 */
 export const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};

/**
 * Returns global state root hash at current block.
 * @param {Object} client - JS SDK client for interacting with a node.
 * @return {String} Root hash of global state at most recent block.
 */
export const getStateRootHash = async (nodeAddress: string) => {
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const { block } = await client.getLatestBlockInfo();
  if (block) {
    return block.header.state_root_hash;
  } else {
    throw Error("Problem when calling getLatestBlockInfo");
  }
};

export const getAccountInfo = async (
  nodeAddress: string,
  publicKey: CLPublicKey
) => {
  const stateRootHash = await getStateRootHash(nodeAddress);
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const accountHash = publicKey.toAccountHashStr();
  const blockState = await client.getBlockState(stateRootHash, accountHash, []);
  return blockState.Account;
};

/**
 * Returns a value under an on-chain account's storage.
 * @param accountInfo - On-chain account's info.
 * @param namedKey - A named key associated with an on-chain account.
 */
export const getAccountNamedKeyValue = (accountInfo: any, namedKey: string) => {
  const found = accountInfo.namedKeys.find((i: any) => i.name === namedKey);
  if (found) {
    return found.key;
  }
  return undefined;
};



