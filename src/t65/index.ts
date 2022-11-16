export function arrayStringsAreEqual(word1: string[], word2: string[]): boolean {
  if (word1[0][0] !== word2[0][0]) {
    return false;
  }

  const toString = (arr: string[]) => {
    return arr.reduce((acc, cur) => {
      return acc + cur;
    }, '');
  };

  return toString(word1) === toString(word2);
}

console.log(arrayStringsAreEqual(['ab', 'c'], ['a', 'bc']));
console.log(arrayStringsAreEqual(['a', 'cb'], ['ab', 'c']));
console.log(arrayStringsAreEqual(['abc', 'd', 'defg'], ['abcddefg']));
