const { u8aToHex } = require('@polkadot/util');
const { xxhashAsU8a } = require('@polkadot/util-crypto');

function createStorageKey(pallet, storage) {
    const palletHash = xxhashAsU8a(pallet, 128);
    const storageHash = xxhashAsU8a(storage, 128);

    // The final key is the concatenation of the two hashes
    const finalKey = new Uint8Array(palletHash.length + storageHash.length);
    finalKey.set(palletHash);
    finalKey.set(storageHash, palletHash.length);

    return u8aToHex(finalKey);
}

const palletName = process.argv[2];
const storageName = process.argv[3];

if (!palletName || !storageName) {
    console.error('Usage: node get_storage_key.js <PalletName> <StorageName>');
    process.exit(1);
}

const storageKey = createStorageKey(palletName, storageName);
console.log(storageKey);
