export function secondHighest(s: string): number {
  let max = -1;
  let res = -1;
  for (const val of s) {
    const num = Number(val);
    if (isNaN(num)) {
      continue;
    }

    if (num > max) {
      res = max;
      max = num;
    } else if (res < num && num < max) {
      res = num;
    }
  }

  return res;
}

console.log(secondHighest('077'));
