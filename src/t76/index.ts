export function checkPowersOfThree(n: number): boolean {
  while (n !== 0) {
    if (n % 3 === 0 || n % 3 === 1) {
      n = Math.floor(n / 3);
    } else return false;
  }
  return true;
}
