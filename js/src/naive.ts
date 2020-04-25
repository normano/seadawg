import { SeaSinkNode } from "./data";

const EDGE_OPEN = -2;

class SeaEdge {
  constructor(
    public wordsIdx: number,
    public startIdx: number,
    public endIdx: number,
    public dest: SeaNode | SeaSinkNode,
  ) {}

  clone(): SeaEdge {
    
    return new SeaEdge(this.wordsIdx, this.startIdx, this.endIdx, this.dest);
  }
}

class SeaNode {

  public toEdges: Map<string, SeaEdge>;

  constructor(
    public length: number,
    public suffix: SeaNode,
  ) {

    this.toEdges = new Map();
  }

  setEdge(wordsIdx: number, letter: string, startIdex: number, endIdx: number, dest: SeaNode | SeaSinkNode) {

    this.toEdges.set(letter, new SeaEdge(wordsIdx, startIdex, endIdx, dest));
  }

  clone(): SeaNode {

    const cloneNode = new SeaNode(this.length, this.suffix);

    for (let [key, edge] of this.toEdges.entries()) {
      const clonedEdge = edge.clone();

      cloneNode.toEdges.set(key, clonedEdge);
    }

    return cloneNode;
  }
}

/**
 * This class exists as a first version that naively stored strings.
 * Easier to tinker with.
 */
/** @internal */
export class SeaDawgCore<V> {

  // Properties of a CDAWG
  // S is a set of strings, S' is a the set of strings ending with a stop symbol
  // A CDAWG has an initial node (root), ||S|| - 1 internal nodes and |S| - 1 final nodes (sink)
  // Root node and initial nodes must have at least 2 outoging edges
  // Labels of two outgoing edges do not begin wit the same letter
  // Any Factor(S), substring, is represented by a path starting at the initial node
  // Any string in the Suffix(S') is represented by a path starting at the initial node and ending at the final node
  // Suppose that a path spelling out α∈Σ∗ ends at a node v. If a string β is always preceded by γ∈Σ∗ and α=γβ in any string x ∈ S′ such that β ∈ Factor(x), the path spelling out β also ends at node v.

  root: SeaNode;
  sink: SeaSinkNode;
  source: SeaNode;
  words = [];
  currentWordIdx: number = -1;
  wordSet = new Set<string>();
  wordTerminator: string = "\u0000";
  debug = {
    "canonize": false,
    "update": false,
  };

  constructor() {

    this.root = new SeaNode(-1, null);
    (<any>this.root).id = "root";

    this.source = new SeaNode(0, this.root);
    (<any>this.source).id = "source";
  }

  public add(word: string, sink: SeaSinkNode) {

    word = this._terminateWord(word);
    if (this.wordSet.has(word)) {
      return;
    }

    this.wordSet.add(word);
    this.sink = sink;
    
    // Initialize edges from root if needed
    let updateData: [SeaNode, number] = [this.source, 0];
    
    this.currentWordIdx = this.words.push(word) - 1;

    for(let wordIdx = 0; wordIdx < word.length; wordIdx++) {

      const letter = word[wordIdx];
      if (!this.root.toEdges.has(letter)) {
        this.root.setEdge(this.currentWordIdx, letter, wordIdx, wordIdx, this.source);
      }

      (<any>this.sink).length = wordIdx + 1; // This is e as citied in the paper

      updateData = this.update(word, letter, updateData[0], updateData[1], wordIdx);
    }
  }

  public findExact(word: string): SeaSinkNode {

    word = this._terminateWord(word);

    let targetNode: SeaSinkNode = null;
    let wordIdx = 0;

    let currentNode = this.source;
    
    while(true) {

      const wordFirstChar = word[wordIdx];
      const matchingEdge = currentNode.toEdges.get(wordFirstChar);
      
      if(!matchingEdge) {
        break;
      }

      const partialLength = matchingEdge.endIdx - matchingEdge.startIdx + 1;
      const edgeWord = this.words[matchingEdge.wordsIdx];
      
      if(edgeWord.substring(matchingEdge.startIdx, matchingEdge.endIdx + 1) === word.substring(wordIdx, wordIdx + partialLength)) {

        if (!(matchingEdge.dest instanceof SeaNode)) {
          
          if (matchingEdge.dest.length === word.length) {
            targetNode = matchingEdge.dest;
          }
          
          break;
        }

        currentNode = matchingEdge.dest;
        wordIdx += partialLength;
        continue;
      }

      break;
    }

    return targetNode;
  }

  update(word: string, letter: string, updateNode: SeaNode, startIdx: number, endIdx: number): [SeaNode, number] {

    let prevNode: SeaNode = null;
    let updateNodePrime: SeaNode | SeaSinkNode = null;
    let updateNodeNext: SeaNode = null;

    if(this.debug.update) {
      console.log("Before check endpoint", startIdx, endIdx, letter, word);
    }

    while (!this.checkEndpoint(updateNode, startIdx, endIdx - 1, letter, word)) {

      if (startIdx <= endIdx - 1) {

        const possibleExtension = this.extension(updateNode, startIdx, endIdx - 1, word);
        if (updateNodePrime === possibleExtension) {

          this.redirectEdge(updateNode, startIdx, endIdx - 1, updateNodeNext, word);
          [updateNode, startIdx] = this.canonize(updateNode.suffix, startIdx, endIdx - 1, word);

          if (this.debug.update) {
            console.log("After check endpoint, redirectEdge", startIdx, endIdx, letter, word);
          }
          continue;
        }

        updateNodePrime = possibleExtension;
        updateNodeNext = this.splitEdge(updateNode, startIdx, endIdx - 1, word);

      } else {

        updateNodeNext = updateNode;
      }

      updateNodeNext.setEdge(this.currentWordIdx, letter, endIdx, EDGE_OPEN, this.sink);

      if (prevNode != null) {
        prevNode.suffix = updateNodeNext;
      }

      prevNode = updateNodeNext;

      let snapNode = updateNode;
      [updateNode, startIdx] = this.canonize(updateNode.suffix, startIdx, endIdx - 1, word);

      if(this.debug.update) {
        console.log("After check endpoint, looping", startIdx, endIdx, letter, word);
      }

      if (this.debug.update && !updateNode) {
        console.error("update", word, letter, startIdx, endIdx, snapNode);
      }
    }

    if (prevNode != null) {
      prevNode.suffix = updateNode;
    }

    if (this.debug.update) {
      console.log("before separate", startIdx, endIdx);
    }

    return this.separateNode(updateNode, startIdx, endIdx, word);
  }

  checkEndpoint(src: SeaNode, startIdx: number, endIdx: number, letter: string, word: string): boolean {

    if (startIdx <= endIdx) {

      const edge = src.toEdges.get(word[startIdx]);
      word = this.words[edge.wordsIdx];
      return letter === word[edge.startIdx + endIdx - startIdx + 1];
    }

    return src.toEdges.has(letter);
  }

  canonize(node: SeaNode, startIdx: number, endIdx: number, word: string): [SeaNode, number] {

    if (startIdx > endIdx) {
      return [node, startIdx];
    }

    let edge = node.toEdges.get(word[startIdx]);

    if (this.debug.canonize && !edge) {
      console.log("canonize: Empty Edge detected", startIdx, endIdx, word, word[startIdx], node);
    }
    
    let edgeIdxDiff = getEndIdx(edge) - edge.startIdx;
    while(edgeIdxDiff <= endIdx - startIdx) {

      startIdx += edgeIdxDiff + 1;

      if (!(edge.dest instanceof SeaNode)) {
        
        throw new Error("Only SeaNodes should be returned by canonize");
      }

      node = edge.dest;

      let snapEdge = edge;
      if(startIdx <= endIdx) {

        if (this.debug.canonize) {
          console.log("canonize: startIdx <= endIdx", startIdx, endIdx, word, word[startIdx], snapEdge);
        }

        edge = node.toEdges.get(word[startIdx]);

        if (this.debug.canonize && !edge) {
          console.log("canonize: startIdx <= endIdx, empty edge", startIdx, endIdx, word, word[startIdx]);
        }
      }
      
      edgeIdxDiff = getEndIdx(edge) - edge.startIdx;
    }

    return [node, startIdx];
  }

  extension(node: SeaNode, startIdx: number, endIdx: number, word: string): SeaNode | SeaSinkNode {

    if (startIdx > endIdx) {
      return node;
    }

    const letter = word[startIdx];
    const edge = node.toEdges.get(letter);

    return edge.dest;
  }

  redirectEdge(src: SeaNode, startIdx: number, endIdx: number, dest: SeaNode, word: string) {

    const letter = word[startIdx];
    const edge = src.toEdges.get(letter);
    const newEndIdx = edge.startIdx + endIdx - startIdx;
    
    src.setEdge(edge.wordsIdx, letter, edge.startIdx, newEndIdx, dest);
  }

  // Adds a node into the middle of an edge, which results src -> newNode -> src.dest
  // Just need to think of it as splitting a string
  splitEdge(src: SeaNode, startIdx: number, endIdx: number, word: string): SeaNode {

    const letter = word[startIdx]; // This is the k variable
    const edge = src.toEdges.get(letter);
    const snapWord = word;
    word = this.words[edge.wordsIdx]; // Word at edge
    
    const subStringLength = endIdx - startIdx; // This is p - k
    const newNode = new SeaNode(src.length + subStringLength + 1, null);
    
    // newNode.edge[letter] -> src.dest
    const newStartIdx = edge.startIdx + subStringLength + 1;
    const newLetter = word[newStartIdx];

    if (typeof newLetter === "undefined" || newLetter === null) {
      console.log(`[snap, ${snapWord}, ${snapWord.length}]`, `[curr, ${word}, ${word.length}]`, `edge startIdx: ${edge.startIdx} => ${newStartIdx}`, `edge endIdx: ${edge.endIdx} => ${endIdx}`, `[snap: ${letter}, curr: ${newLetter}]`, subStringLength, startIdx, endIdx);
      console.log(require("util").inspect(src, {"depth": 4}));
      console.error("^ New letter should not be empty. Info above.");
    }

    newNode.setEdge(edge.wordsIdx, newLetter, newStartIdx, edge.endIdx, edge.dest);

    // src.edge[letter] -> newNode
    src.setEdge(edge.wordsIdx, letter, edge.startIdx, newStartIdx - 1, newNode);

    return newNode;
  }

  separateNode(src: SeaNode, startIdx: number, endIdx: number, word: string): [SeaNode, number] {

    let canonizedData = this.canonize(src, startIdx, endIdx, word);

    if (canonizedData[1] <= endIdx) {
      return canonizedData;
    }

    let canonNode = canonizedData[0];
    const sepLength = src.length + endIdx - startIdx + 1;

    if (canonNode.length === sepLength) {
      return canonizedData;
    }

    // Represents old state
    const sepNode = canonNode.clone();
    sepNode.length = sepLength;
    sepNode.suffix = canonNode.suffix;
    canonNode.suffix = sepNode; // is the new state

    while(true) {
      
      const edge = src.toEdges.get(word[startIdx]);

      src.setEdge(edge.wordsIdx, this.words[edge.wordsIdx][edge.startIdx], edge.startIdx, edge.endIdx, sepNode);

      [src, startIdx] = this.canonize(src.suffix, startIdx, endIdx - 1, word);
      
      let newCanonizedNodePair = this.canonize(src, startIdx, endIdx, word);

      if ((canonizedData[0] !== newCanonizedNodePair[0]) || (canonizedData[1] !== newCanonizedNodePair[1])) {
        break;
      }
    }
    
    return [sepNode, endIdx + 1];
  }

  private _terminateWord(word: string): string {
    return word + this.wordTerminator;
  }
}

/**
 * Most efficient way to return the value for open edges.
 */
function getEndIdx(edge: SeaEdge): number {
  
  return edge.endIdx === EDGE_OPEN ? edge.dest.length : edge.endIdx; // This is e(i) as citied in the paper
}