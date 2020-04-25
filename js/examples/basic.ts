import {dump} from "dumpenvy";

// import { SeaDawgCore } from "../src/v1/core";
// import { SeaValueSinkNode } from "../src/v1/data";
import { SeaDawgCore } from "../src/v2/core";
import { SeaValueSinkNode } from "../src/v2/data";

function main() {

  let words = [
    "cocoa",
    "abbabc",
    "cola",
    "coca cola",
    "coca",
    "key",
    "fob",
    "baby",
    "GG",
    "Good Game",
    "Dawg",
    "aye aye captain",
    "Matey",
    "Ohhhhhhhhhhhhhh",
    "arrrrrrrrrr ye scurvy dawg",
    "walk da plank",
    "who lives in a pipeapple under da sea?",
    "black beard, a fearsome pirate",
    "wowza coco nut",
  ];
  let chosenWord = words[0];

  let seaDawg = new SeaDawgCore();

  console.log("Adding words");

  for(const word of words) {
    console.log(`Adding '${word}'`);
    const sink  = new SeaValueSinkNode(word);

    if (word === chosenWord) {
      console.log(`Storing data in word ${chosenWord}`)
      sink.data = "GG";
    }

    seaDawg.add(word, sink);
  }
  
  //console.log(dump(seaDawg));
  console.log("Finding words");

  for(const word of words) {
    
    const sinkNode = seaDawg.findExact(word);

    if(!sinkNode) {
      console.error(`Did not find word "${word}" in graph!`);
    }
  }

  console.log("Completed finding words");

  // Exact match
  console.log("Finding cocoa");
  const sinkNode = <SeaValueSinkNode>seaDawg.findExact(chosenWord);

  if(!sinkNode || sinkNode.data !== "GG") {
    console.error(`Did not find matching data for  "${chosenWord}"'!`);
  }

  console.log(`Find exact of 'a' returns null: ${seaDawg.findExact("a") === null}`);
  console.log(`Find exact of 'coa' returns null: ${seaDawg.findExact("coa") === null}`);
  
  /// Prefixes
  console.log("Finding prefixed nodes of: 'w'");
  const nodesWithPrefixOfW = seaDawg.findWithPrefix("w");

  if(nodesWithPrefixOfW.length !== 3) {
    console.error("Did not return correct number of prefixes for 'w'");
  }

  console.log("Finding prefixed nodes of: 'c'");
  const nodesWithPrefixOfc = seaDawg.findWithPrefix("c");

  //console.log(nodesWithPrefixOfc, nodesWithPrefixOfc.length);
  if(nodesWithPrefixOfc.length !== 4) {
    console.error("Did not return correct number of prefixes for 'c'");
  }

  console.log("Finding prefixed nodes of: 'co'");
  const nodesWithPrefixOfco = seaDawg.findWithPrefix("co");

  //console.log(nodesWithPrefixOfco, nodesWithPrefixOfco.length);
  if(nodesWithPrefixOfco.length !== 4) {
    console.error("Did not return correct number of prefixes for 'co'");
  }

  /// Super strings
  console.log("Finding superstrings of: 'w'");
  const nodesSuperStringOfW = seaDawg.findWithSubstring("w");

  //console.log(nodesSuperStringOfW.length, nodesSuperStringOfW);
  if(nodesSuperStringOfW.length !== 5) {
    console.error("Did not return correct number of super strings for 'w'");
  }

  console.log("Finding superstrings of: 'c'");
  const nodesSuperStringOfc = seaDawg.findWithSubstring("c");

  //console.log(nodesSuperStringOfc.length, nodesSuperStringOfc);
  if(nodesSuperStringOfc.length !== 9) {
    console.error("Did not return correct number of super strings for 'c'");
  }
/*
  console.log("Removing `cocoa`");
  seaDawg.delete("cocoa");
  const afterCocoaDeletedValue = seaDawg.findExact("cocoa");

  if(afterCocoaDeletedValue) {
    console.error("Cocoa still exists");
  }

  console.log("Adding cocoa again");

  seaDawg.add("cocoa", new SeaValueSinkNode("cocoa"));

  const afterCocoaAddedValue = seaDawg.findExact("cocoa");

  if(!afterCocoaAddedValue) {
    console.error("Cocoa doesn't exist after re-adding.");
  }

  console.log("Removing all words")
  for(const word of words) {
    
    seaDawg.delete(word);
  }

  if(seaDawg.size !== 0) {

    console.error("Graph is not empty after all words removed");
  }
  */
}

main();