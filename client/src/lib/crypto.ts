import { secp256k1 } from "@noble/curves/secp256k1.js";
import { HDKey } from "@scure/bip32";
import { mnemonicToSeedSync } from "@scure/bip39";
export { sha256 } from "@noble/hashes/sha2.js";

export function deriveKeys(mnemonic: string, path = "m/8797555'/0'/0'") {
	const seed = mnemonicToSeedSync(mnemonic);
	const key = HDKey.fromMasterSeed(seed).derive(path);

	if (!key.privateKey || !key.publicKey) {
		throw new Error("Unable to derive keys");
	}

	return { privateKey: key.privateKey, publicKey: key.publicKey };
}

export function sign(privateKey: Uint8Array, data: Uint8Array) {
	return secp256k1.sign(data, privateKey, { format: "compact", prehash: false });
}
