import { SeaDawgMap } from "../src";

function main() {

  const kvs = [
    ["cdawg", "is my dawg"],
    ["mydawg", "is a cdawg"]
  ];
  const seamap = new SeaDawgMap<string>();

  console.log("Inserting values");

  for(const [key, value] of kvs) {
    seamap.set(key, value);
  }

  console.log("Checking values");

  for(const [key, value] of kvs) {
    const actual = seamap.get(key);

    if(actual !== value) {
      console.error(`The key "${key}" did not have expected value "${value}" instead contains "${actual}"`);
    }
  }
  
  console.log("Completed run");
}

main();