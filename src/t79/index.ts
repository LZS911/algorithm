export function checkIfPangram(sentence: string): boolean {
  if (sentence.length < 26) {
    return false;
  }

  const pangram = 'abcdefghijklmnopqrstuvwxyz';

  return Array.from(pangram).every(v => sentence.indexOf(v) > -1);
}

console.log(
  checkIfPangram(
    'jwtucoucmdfwxxqnxzkaxoglszmfrcvjoiunqqausaxxaaijyqdqgvdnqcaihwilqkpivenpnekioyqujrdrovqrlxovcucjqzjsxmllfgndfprctxvxwlzjtciqxgsxfwhmuzqvlksyuztoetyjugmswfjtawwaqmwyxmvo'
  )
);
