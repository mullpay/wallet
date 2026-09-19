function formatArgs(args: any[]): string {
	return args.map((arg) => (typeof arg === "string" ? arg : JSON.stringify(arg, Object.getOwnPropertyNames(arg)))).join(" ");
}

export function error(...args: any[]) {
	console.log(`[error]: ${formatArgs(args)}`);
}

export function warn(...args: any[]) {
	console.log(`[warn]: ${formatArgs(args)}`);
}

export function info(...args: any[]) {
	console.log(`[info]: ${formatArgs(args)}`);
}

export function debug(...args: any[]) {
	console.log(`[debug]: ${formatArgs(args)}`);
}
