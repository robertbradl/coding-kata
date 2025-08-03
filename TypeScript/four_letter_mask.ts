export function maskify(cc: string): string {
	if (cc.length > 4)
		return `${"#".repeat(cc.length - 4)}${cc.slice(cc.length - 4)}`;
	else return cc;
}

console.log(maskify("asdafeasdada"));
