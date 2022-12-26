export function nearestValidPoint(x: number, y: number, points: number[][]): number {
  const n = points.length;
  let min = Number.MAX_VALUE;
  let res = -1;
  for (let i = 0; i < n; ++i) {
    const px = points[i][0],
      py = points[i][1];
    if (x === px) {
      const dist = Math.abs(y - py);
      if (dist < min) {
        min = dist;
        res = i;
      }
    } else if (y === py) {
      const dist = Math.abs(x - px);
      if (dist < min) {
        min = dist;
        res = i;
      }
    }
  }
  return res;
}

nearestValidPoint(3, 4, [
  [1, 2],
  [3, 1],
  [2, 4],
  [2, 3],
  [4, 4]
]);
