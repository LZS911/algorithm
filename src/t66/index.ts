export function nthUglyNumber(n: number): number {
  //timeout
  // const isUgly = (target: number): boolean => {
  //   if (target === 0) {
  //     return false;
  //   }
  //   if (target % 2 === 0) {
  //     return isUgly(target / 2);
  //   }
  //   if (target % 3 === 0) {
  //     return isUgly(target / 3);
  //   }
  //   if (target % 5 === 0) {
  //     return isUgly(target / 5);
  //   }

  //   return target === 1;
  // };

  // let res = 0;

  // while (n > 0) {
  //   res += 1;
  //   if (isUgly(res)) {
  //     n = n - 1;
  //   }
  // }

  // return res;

  let a = 0,
    b = 0,
    c = 0;
  const arr: number[] = [1];

  for (let i = 1; i < n; ++i) {
    const n2 = arr[a] * 2,
      n3 = arr[b] * 3,
      n5 = arr[c] * 5;
    arr.push(Math.min(n2, n3, n5));

    if (arr[i] === n2) a++;
    if (arr[i] === n3) b++;
    if (arr[i] === n5) c++;
  }
  return arr[arr.length - 1];
}

console.log(nthUglyNumber(1352));
