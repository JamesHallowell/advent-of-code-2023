export type Race = {
  duration: number;
  distanceToBeat: number;
};

export function parseRaces(input: string): Race[] {
  const whitespace = /\s+/;
  const tokens = input.trim().split(whitespace);
  const numberOfRaces = tokens.length / 2 - 1;
  const times = tokens.slice(1, numberOfRaces + 1).map(Number);
  const distances = tokens.slice(numberOfRaces + 2).map(Number);
  return times.map(
    (duration, i): Race => ({ duration, distanceToBeat: distances[i] }),
  );
}
