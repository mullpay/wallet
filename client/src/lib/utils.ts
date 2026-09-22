export function clamp(value: number, min: number, max: number) {
	return Math.min(Math.max(value, min), max);
}

export function debounce(callback: Function, wait = 1000) {
	let timeout: ReturnType<typeof setTimeout>;

	return (...args: any[]) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => callback(...args), wait);
	};
}

export function parse<T>(input: any): T | undefined {
	try {
		const data = String(input);
		return JSON.parse(data);
	} catch {
		return undefined;
	}
}

export function bytesToHex(bytes: Uint8Array): string {
	return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

export function hexToBytes(hex: string): Uint8Array {
	const bytes = new Uint8Array(hex.length / 2);

	for (let i = 0; i < hex.length; i += 2) {
		bytes[i / 2] = parseInt(hex.slice(i, i + 2), 16);
	}

	return bytes;
}

export function stringToUint8Array(text: string) {
	return new TextEncoder().encode(text);
}

export function uint8ArrayToString(bytes: Uint8Array) {
	return new TextDecoder().decode(bytes);
}
