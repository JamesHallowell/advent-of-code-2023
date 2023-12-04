import { readFileSync } from "fs";

const file = readFileSync("input.txt", "utf-8");

function parseNumbers(numbers: string[]): number[] {
  return numbers.map((token) => parseInt(token));
}

type Card = {
  number: number;
  winning: Set<number>;
  mine: Set<number>;
};

const cards = file
  .split("\n")
  .filter((line) => line.length > 0)
  .map((line): Card => {
    const tokens = line.split(/\s+/);
    const number = parseInt(tokens[1].slice(0, -1));
    const middle = tokens.findIndex((token) => token === "|");
    const winningNumbers = parseNumbers(tokens.slice(2, middle));
    const myNumbers = parseNumbers(tokens.slice(middle + 1, tokens.length));

    return {
      number,
      winning: new Set(winningNumbers),
      mine: new Set(myNumbers),
    };
  });

const part1 = cards
  .map((card) => {
    let points = 0;
    for (const number of card.mine) {
      if (card.winning.has(number)) {
        points = points === 0 ? 1 : points * 2;
      }
    }
    return points;
  })
  .reduce((a, b) => a + b, 0);

console.log(`Part 1: ${part1}`);

class Matches {
  matches: Map<Card, number> = new Map();

  get(card: Card): number {
    const matches = this.matches.get(card);
    if (matches !== undefined) {
      return matches;
    }

    const matching = Array.from(card.winning).reduce(
      (matching, winner) => (card.mine.has(winner) ? matching + 1 : matching),
      0,
    );
    this.matches.set(card, matching);
    return matching;
  }
}

const matches = new Matches();

const queue = [...cards];
let total = 0;

while (queue.length > 0) {
  const card = queue.pop()!;
  const wins = matches.get(card);
  queue.push(...cards.slice(card.number, card.number + wins));
  total++;
}

console.log(`Part 2: ${total}`);
