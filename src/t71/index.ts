export function minOperations(boxes: string): number[] {
  const ballIndexes: number[] = [];
  const n = boxes.length;
  for (let i = 0; i < n; i++) {
    if (boxes[i] === '1') ballIndexes.push(i);
  }
  const ans = new Array(n).fill(0);
  for (let i = 0; i < n; i++) {
    let step = 0;
    for (const bi of ballIndexes) {
      step += Math.abs(i - bi);
    }
    ans[i] = step;
  }
  return ans;
}

minOperations('110');
minOperations('001011');
