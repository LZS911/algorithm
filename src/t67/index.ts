export function maxRepeating(sequence: string, word: string): number {
  let max = Math.floor(sequence.length / word.length);

  while (max > 0) {
    let str = '';
    for (let i = 0; i < max; ++i) {
      str += word;
    }

    if (sequence.includes(str)) {
      break;
    }
    max--;
  }

  return max;
}

console.log(maxRepeating('ababc', 'ab'));
console.log(maxRepeating('ababc', 'ba'));
console.log(maxRepeating('ababc', 'ac'));
