# Casperlabs-StakingRewardsFactoryAndStakingDualRewards-JsClient

This repo has the code to deploy all the functions of StakingRewardsFactory contracts using StakingRewardsFactory contract hash and StakingDualRewards contracts using StakingDualRewards contract hash

## Prerequisite

Make sure you have created, pasted and funded the keys before testing.


## Testing

Use the script file in package.json to perform the testing
```
"scripts": {
    "test:stakingrewardsfactoryinstalled": "ts-node StakingRewardsFactory/test/installed.ts"
    "test:stakingdualrewardsinstalled": "ts-node StakingDualRewards/test/installed.ts"
  },
```

Use the following commands to perform testing
```
npm run test:stakingrewardsfactoryinstalled
npm run test:stakingdualrewardsinstalled

```

* CONFIGURE .env BEFORE TESTING

