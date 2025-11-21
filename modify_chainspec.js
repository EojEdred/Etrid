const fs = require('fs');

const localTestnetPath = 'local-testnet.json';
const v108SpecPath = 'chainspec_v108_21VAL.json';

try {
    // Read both files
    const localTestnetContent = fs.readFileSync(localTestnetPath, 'utf8');
    const v108SpecContent = fs.readFileSync(v108SpecPath, 'utf8');

    // Parse both JSON files
    const localTestnet = JSON.parse(localTestnetContent);
    const v108Spec = JSON.parse(v108SpecContent);

    // Get the validator_committee from the v108 spec
    const validatorCommitteeToInject = v108Spec.genesis.runtimeGenesis.validator_committee;

    // Replace the validatorCommittee in the local-testnet spec
    // Note: The key in the dev spec is camelCase 'validatorCommittee', but the v108 spec uses snake_case 'validator_committee'.
    // We will use the camelCase version as it is from the newer 'dev' spec.
    localTestnet.genesis.runtimeGenesis.validatorCommittee = validatorCommitteeToInject;

    // Also, ensure the pallet key is correct if it was different
    if (localTestnet.genesis.runtimeGenesis.validator_committee) {
        delete localTestnet.genesis.runtimeGenesis.validator_committee;
    }


    // Stringify the modified object with pretty printing
    const newLocalTestnetContent = JSON.stringify(localTestnet, null, 2);

    // Write the modified content back to the file
    fs.writeFileSync(localTestnetPath, newLocalTestnetContent);

    console.log(`Successfully updated ${localTestnetPath} with the validator committee from ${v108SpecPath}`);

} catch (error) {
    console.error('Error modifying chainspec:', error);
    process.exit(1);
}