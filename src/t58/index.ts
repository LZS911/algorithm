export function advantageCount(nums1: number[], nums2: number[]): number[] {
  if (nums1.length !== nums2.length) {
    return [];
  }

  nums1.sort((a, b) => a - b);
  return nums2.map(n2 => {
    const num = nums1.find(n1 => n1 > n2);
    if (!!num || num === 0) {
      const index = nums1.findIndex(n1 => n1 === num);
      nums1.splice(index, 1);
      return num;
    }

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return nums1.shift()!;
  });
}

const res1 = advantageCount([2, 7, 11, 15], [1, 10, 4, 11]);
const res2 = advantageCount([12, 24, 8, 32], [13, 25, 32, 11]);
const res3 = advantageCount([0, 1, 2, 2, 4], [1, 3, 0, 0, 2]);

console.log(res1, res2, res3);
