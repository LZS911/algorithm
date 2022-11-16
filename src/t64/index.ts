export function magicalString(n: number): number {
  let str = '122';
  let index = 2;
  let res = 1;

  while (str.length < n) {
    const cur = str[index];
    if ((index & 1) === 0) {
      if (str.length + 2 > n) {
        res += 1;
      } else {
        res += Number(cur);
      }
      // 添加 1
      cur === '1' ? (str += '1') : (str += '11');
    } else {
      // 添加 2
      cur === '1' ? (str += '2') : (str += '22');
    }
    index++;
  }

  return res;
}

console.log(magicalString(20));
console.log(magicalString(4));
