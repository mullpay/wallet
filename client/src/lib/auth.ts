import { deleteCookie, getCookie, setCookie } from "$lib/cookie";

export function login(mnemonic: string[]) {
	setCookie("mnemonic", mnemonic.join(" "));
}

export function logout() {
	deleteCookie("mnemonic");
}

export function auth(): string | undefined {
	return getCookie("mnemonic");
}
