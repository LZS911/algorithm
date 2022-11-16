export function maximumUnits(boxTypes: number[][], truckSize: number): number {
  let res = 0;

  boxTypes.sort((a, b) => b[1] - a[1]);

  for (let i = 0; i < boxTypes.length; ++i) {
    if (boxTypes[i][0] > truckSize) {
      res += truckSize * boxTypes[i][1];
      break;
    }
    res += boxTypes[i][0] * boxTypes[i][1];
    truckSize -= boxTypes[i][0];
  }
  return res;
}

maximumUnits(
  [
    [1, 3],
    [2, 2],
    [3, 1]
  ],
  4
);
