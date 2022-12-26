export function numDifferentIntegers(word: string): number {
  const arr = word.replace(/[a-zA-Z]/g, ' ').matchAll(/\d+/g);

  const set = new Set();
  for (const val of Array.from(arr)) {
    set.add(val[0].replace(/^(0+)/, ''));
  }
  return set.size;
}

console.log(numDifferentIntegers('a1b01c001'));
