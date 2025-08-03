const MORSE_CODE = {};

export function decodeMorse(morseCode: string): string {
	return morseCode
		.trim()
		.split(" ".repeat(3))
		.map((sentence) =>
			sentence
				.trim()
				.split(" ")
				.map((word) => MORSE_CODE[word] ?? word)
				.join("")
		)
		.join(" ");
}
