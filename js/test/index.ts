import test from "ava";
import { arrayShuffle } from "@adriantombu/array-shuffle";

import { SeaDawgCore } from "../src/core";
import { SeaValueSinkNode } from "../src/data";

const words = [
  "cocoa",
  "abbabc",
  "cola",
  "coca cola",
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
];

test("Size = 1 when adding one word", t => {

  const seaDawg = new SeaDawgCore();
  
  const sink  = new SeaValueSinkNode(1);
  seaDawg.add(words[0], sink);

  t.true(seaDawg.size === 1, "did not get only 1 element in graph");
});

test("Word cannot be found after delete", t => {

  const seaDawg = new SeaDawgCore();
  
  const sink0  = new SeaValueSinkNode(1);
  seaDawg.add(words[0], sink0);
  const sink1  = new SeaValueSinkNode(2);
  seaDawg.add(words[1], sink1);
  const sink2  = new SeaValueSinkNode(3);
  seaDawg.add(words[2], sink2);

  t.true(seaDawg.size === 3, "Did not get only 3 element in graph");

  seaDawg.delete(words[1]);
  t.true(seaDawg.size === 2, "Did not get only 2 element in graph");

  seaDawg.delete(words[0]);
  t.true(seaDawg.size === 1, "Did not get only 1 element in graph");

  seaDawg.delete(words[2]);
  t.true(seaDawg.size === 0, "Graph is not empty");
});

test("findExact find all tens words when adding ten words", t => {

  const expectedSinks = new Map();
  const seaDawg = new SeaDawgCore();
  
  for(let idx = 0; idx < 10; idx++) {
    const word = words[idx];
    const sink  = new SeaValueSinkNode(word);
    seaDawg.add(word, sink);
    expectedSinks.set(word, sink);
  }

  t.true(seaDawg.size === 10, "Did not get only 10 elements in graph");

  for(const word of expectedSinks.keys()) {
    
    const sinkNode = seaDawg.findExact(word);
    
    t.true(!!sinkNode, `Could not find ${word} in graph.`);
    t.is(sinkNode, expectedSinks.get(word), `Sink node for ${word} is not the expected.`);
  }
});

test("findExact does not find words outside of added ten words", t => {

  const seaDawg = new SeaDawgCore();
  
  for(let idx = 0; idx < 10; idx++) {
    const word = words[idx];
    const sink  = new SeaValueSinkNode(word);
    seaDawg.add(word, sink);
  }

  t.true(seaDawg.size === 10, "Did not get only 10 elements in graph");

  for(let idx = 10; idx < words.length ; idx++) {
    
    const word = words[idx];
    const sinkNode = seaDawg.findExact(word);
    
    t.true(!sinkNode, `Found ${word} in graph which is unexpected.`);
  }
});

test("findExact must not find deleted words after removal", t => {

  const expectedSinks = new Map();
  const seaDawg = new SeaDawgCore();
  
  for(const word of words) {
    const sink  = new SeaValueSinkNode(word);
    seaDawg.add(word, sink);
    expectedSinks.set(word, sink);
  }

  t.true(seaDawg.size === words.length, "Did not get all the words in the list");

  for(const word of words) {
    
    const sinkNode = seaDawg.findExact(word);
    
    t.true(!!sinkNode, `Could not find ${word} in graph.`);
    t.is(sinkNode, expectedSinks.get(word), `Sink node for ${word} is not the expected.`);
  }
  
  const deletedWords = arrayShuffle(words).splice(words.length / 2, words.length);

  for(const deletedWord of deletedWords) {
    seaDawg.delete(deletedWord);
  }

  t.true(seaDawg.size === (words.length - deletedWords.length), "Did not remove selected deleted words");
  
  for(const deletedWord of deletedWords) {
    
    const sinkNode = seaDawg.findExact(deletedWord);
    
    t.true(!sinkNode, `Found ${deletedWord} in graph which is unexpected.`);
  }
});