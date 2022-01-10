import { numberStrEmpty, numberStr } from '../data/str.data';
const map = new Map<string, string[]>([
  ['2', ['a', 'b', 'c']],
  ['3', ['d', 'e', 'f']],
  ['4', ['g', 'h', 'i']],
  ['5', ['j', 'k', 'l']],
  ['6', ['m', 'n', 'o']],
  ['7', ['p', 'q', 'r', 's']],
  ['8', ['t', 'u', 'v']],
  ['9', ['w', 'x', 'y', 'z']]
]);
//暴力方法
function letterCombinations(digits: string): string[] {
  const tmp: string[][] = [];
  if (!digits || Array.from(digits).some(item => Number(item) < 2)) {
    return [];
  }

  for (let i = 0; i < digits.length; ++i) {
    if (map.has(digits[i])) {
      tmp.push(map.get(digits[i])!);
    }
  }

  return tmp.reduce((res, current) => {
    return flatArray(res.map(r => current.map(c => r + c)));
  });
}

//数组降维
function flatArray<T>(arr: any[]): T[] {
  return arr.reduce(
    (res, current) => (Array.isArray(current) ? [...res, ...flatArray(current)] : [...res, current]),
    []
  );
}

//回溯
function letterCombinations2(digits: string): string[] {
  const res: string[] = [];
  if (!digits || Array.from(digits).some(item => Number(item) < 2)) {
    return [];
  }
  const backtrack = (count: number, letters: string) => {
    if (digits.length === count) {
      res.push(letters);
      return;
    }

    const digit = digits[count];
    const phone = map.get(digit)!;
    for (let i = 0; i < phone.length; ++i) {
      backtrack(count + 1, letters + phone[i]);
    }
  };

  backtrack(0, '');

  return res;
}

letterCombinations2(numberStrEmpty);
letterCombinations2(numberStr);
