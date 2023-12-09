import { readFileSync } from "fs";

const getDiffs = (seq: number[]) => {
  const diffs = [];
  for (let i = 1; i < seq.length; i++) {
    diffs.push(seq[i] - seq[i - 1]);
  }
  return diffs;
};

type Direction = "forwards" | "backwards";

const extrapolate = (seq: number[], dir: Direction): number => {
  const prev = dir === "forwards" ? seq[seq.length - 1] : seq[0];

  const values = [];
  seq = getDiffs(seq);
  while (!seq.every((diff) => diff === 0)) {
    values.push(dir === "forwards" ? seq[seq.length - 1] : seq[0]);
    seq = getDiffs(seq);
  }

  return dir === "forwards"
    ? values.reduce((a, b) => a + b, prev)
    : prev - values.reverse().reduce((a, b) => b - a);
};

const sumExtrapolatedValues = (sequences: number[][], direction: Direction) => {
  return sequences
    .map((sequence) => extrapolate(sequence, direction))
    .reduce((a, b) => a + b);
};

const sequences = readFileSync("input.txt", "utf8")
  .trim()
  .split("\n")
  .map((line) => line.split(" ").map(Number));

console.log(`Part 1: ${sumExtrapolatedValues(sequences, "forwards")}`);
console.log(`Part 2: ${sumExtrapolatedValues(sequences, "backwards")}`);
