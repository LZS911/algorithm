export function getLucky(s: string, k: number): number {
  let str = '',
    count = 0;

  for (const char of s) {
    str += String(char.charCodeAt(0) - 96);
  }

  while (k--) {
    count = 0;
    for (const char of str) {
      count += +char;
    }
    str = String(count);
  }

  return count;
}

console.log(getLucky('iiii', 1));
