import { readFileSync } from "fs";
import { parseRaces, Race } from "./parse.js";

function distanceTravelledInRace(race: Race) {
  return (holdDuration: number) => {
    return holdDuration * (race.duration - holdDuration);
  };
}

function beatsCurrentBestDistanceFor(race: Race) {
  return (distanceTravelled: number) => {
    return distanceTravelled > race.distanceToBeat;
  };
}

function numberOfWaysToWin(race: Race): number {
  return Array.from({ length: race.duration }, (_, i) => i)
    .map(distanceTravelledInRace(race))
    .filter(beatsCurrentBestDistanceFor(race)).length;
}

function part1(races: Race[]): number {
  return races.map(numberOfWaysToWin).reduce((a, b) => a * b, 1);
}

function part2(race: Race): number {
  return numberOfWaysToWin(race);
}

const races = parseRaces(readFileSync("input.txt", "utf8"));
console.log(`Part 1: ${part1(races)}`);
console.log(
  `Part 2: ${part2({ duration: 47707566, distanceToBeat: 282107911471062 })}`,
);
