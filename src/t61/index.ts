export function kthGrammar(n: number, k: number): number {
  if (k < 1 || k > Math.pow(2, n - 1)) {
    throw Error('expect k');
  }

  if (k === 1) {
    return 0;
  }
  if (k === 2) {
    return 1;
  }

  //每一行的长度
  const len = Math.pow(2, n - 1);

  //前半部分与上一行的内容相同
  if (k <= len / 2) {
    return kthGrammar(n - 1, k);
  }

  //后半部分与上一行的内容相反 (1 to 0, 0 to 1)
  return kthGrammar(n - 1, k - len / 2) === 1 ? 0 : 1;
}

console.log(kthGrammar(1, 1));
console.log(kthGrammar(2, 1));
console.log(kthGrammar(2, 2));
