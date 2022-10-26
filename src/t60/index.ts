export function countStudents(students: number[], sandwiches: number[]): number {
  const count = [];
  count[0] = students.filter(v => v === 0).length;
  count[1] = students.filter(v => v === 1).length;
  for (const val of sandwiches) {
    if (count[val] > 0) {
      count[val]--;
    } else {
      break;
    }
  }

  return count[0] + count[1];
}

console.log(countStudents([1, 1, 0, 0], [0, 1, 0, 1]));
console.log(countStudents([1, 1, 1, 0, 0, 1], [1, 0, 0, 0, 1, 1]));
