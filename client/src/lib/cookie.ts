export function getCookie(name: string): string | undefined {
	const cDecoded = decodeURIComponent(document.cookie);
	const cArray = cDecoded.split("; ");
	let result = undefined;

	cArray.forEach((element) => {
		if (element.indexOf(name) == 0) {
			result = element.substring(name.length + 1);
		}
	});

	return result;
}

export function setCookie(name: string, value: string, unixTime?: number) {
	if (unixTime === undefined) {
		document.cookie = `${name}=${value}; SameSite=Strict; path=/`;
	} else {
		const date = new Date(unixTime * 1000);
		const expires = "expires=" + date.toUTCString();

		document.cookie = `${name}=${value}; ${expires}; SameSite=Strict; path=/`;
	}
}

export function deleteCookie(name: string) {
	document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/`;
}
