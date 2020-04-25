import {dump} from "dumpenvy";

import { SeaDawgCore } from "../src/core";
import { SeaDefaultSinkNode } from "../src/data";

/**
 * Cheap way to check the result against the graph given by the paper
 */
function main() {

  let seaDawg = new SeaDawgCore();
  seaDawg.add("cocoa", new SeaDefaultSinkNode());
  seaDawg.add("cola",  new SeaDefaultSinkNode());

  console.log("RESULT")
  console.log(dump(seaDawg));
}

main();