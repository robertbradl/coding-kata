/**
 * "ATTGC" --> "TAACG"
 * "GTAT" --> "CATA"
 */

export class Kata {
	static COMPLEMENTS: Record<string, string> = {
		A: "T",
		T: "A",
		G: "C",
		C: "G",
	};
	static dnaStrand(dna: string) {
		return dna
			.split("")
			.map((c: string) => Kata.COMPLEMENTS[c] ?? c)
			.join("");
	}
}
