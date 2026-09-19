const REGEX_BITCOIN = /\b((bc1[qp])[a-z0-9]{11,71}|[13][a-km-zA-HJ-NP-Z1-9]{25,34})\b/i;
const REGEX_SPARK = /^spark[0-9a-z]{1,}[a-z0-9]$/i;
const REGEX_LIGHTNING = /^ln(bc|tb|bcrt)[0-9a-z]{10,}$/i;

export type AddressType = "SPARK" | "LIGHTNING" | "ON-CHAIN";

export function identifyAddress(address: string): AddressType | undefined {
	if (REGEX_SPARK.test(address)) return "SPARK";
	if (REGEX_LIGHTNING.test(address)) return "LIGHTNING";
	if (REGEX_BITCOIN.test(address)) return "ON-CHAIN";

	return undefined;
}
