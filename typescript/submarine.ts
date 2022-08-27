function getInput() {
  return `
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    `;
}

function parseLine(line: string): [number, number] {
  const [direction, distance] = line.trim().split(' ');

  if (!direction || !direction) return [0, 0];

  const amount = parseInt(distance);

  if (direction === 'forward') {
    return [amount, 0];
  } else if (direction === 'up') {
    return [0, -amount];
  }
  return [0, amount];
}

export function Run() {
  const result = getInput()
    .split('\n')
    .map((line) => parseLine(line))
    .reduce(
      (acc, curr) => {
        acc[0] += curr[0];
        acc[1] += curr[1];
        return acc;
      },
      [0, 0]
    );

  console.log(result, result[0] * result[1]);
}
