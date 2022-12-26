export function squareIsWhite(coordinates: string): boolean {
  return (
    (Number(coordinates[1]) % 2 === 1 && coordinates[0].charCodeAt(0) % 2 === 0) ||
    (Number(coordinates[1]) % 2 === 0 && coordinates[0].charCodeAt(0) % 2 === 1)
  );
}

console.log(squareIsWhite('h3'));
