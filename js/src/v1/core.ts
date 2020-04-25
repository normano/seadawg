import { ScalableCuckooFilter } from "cuckoo-filter";

import { SeaSinkNode } from "./data";

export class SeaEdge {
  constructor(
    public src: SeaNode,
    public partial: string,
    public startIdx: number,
    public endIdx: number,
    public dest: SeaNode | SeaTermNode,
  ) {}
}

export class SeaTermNode {
  public toSinkNodes: Set<SeaSinkNode> = new Set();

  constructor() {

    this.toSinkNodes = new Set();
  }

  addSinkNode(node: SeaSinkNode) {
    this.toSinkNodes.add(node);
    //node.incomingEdges.add(edge);
  }
}

export class SeaNode {

  public toEdges: Map<string, SeaEdge>;
  public incomingEdges: Set<SeaEdge>;

  constructor(
    public length: number,
    public suffix: SeaNode,
  ) {

    this.toEdges = new Map();
    this.incomingEdges = new Set();
  }

  setEdge(partial: string, letter: string, startIdx: number, endIdx: number, dest: SeaNode | SeaTermNode) {

    const existingEdge = this.toEdges.get(letter);

    if(existingEdge) {
      existingEdge.partial = partial;
      existingEdge.startIdx = startIdx;
      existingEdge.endIdx = endIdx;
      
      if(existingEdge.dest !== dest) {

        if (existingEdge.dest instanceof SeaNode) {
          existingEdge.dest.incomingEdges.delete(existingEdge);
        }

        if (dest instanceof SeaNode) {
          dest.incomingEdges.add(existingEdge);
        }

        existingEdge.dest = dest;
      }
    } else {

      const edge = new SeaEdge(this, partial, startIdx, endIdx, dest);
      this.toEdges.set(letter, edge);
      
      if (dest instanceof SeaNode) {
        dest.incomingEdges.add(edge);
      }
    }
  }

  removeEdge(letter: string): SeaEdge {
    
    const existingEdge = this.toEdges.get(letter);

    if(existingEdge === null) {
      return null;
    }

    this.toEdges.delete(letter);

    if (existingEdge.dest instanceof SeaNode) {
      existingEdge.dest.incomingEdges.delete(existingEdge);
    }

    return existingEdge;
  }

  clone(): SeaNode {
    const cloneNode = new SeaNode(this.length, this.suffix);

    for (let [key, edge] of this.toEdges.entries()) {
      cloneNode.setEdge(edge.partial, key, edge.startIdx, edge.endIdx, edge.dest);
    }

    return cloneNode;
  }
}

/**
 * Online Multi String CDAWG extended with the property of same terminator.
 */
export class SeaDawgCore {

  // Properties of a CDAWG
  // S is a set of strings, S' is a the set of strings ending with a stop symbol
  // A CDAWG has an initial node (root), ||S|| - 1 internal nodes and |S| - 1 final nodes (sink)
  // Root node and initial nodes must have at least 2 outoging edges
  // Labels of two outgoing edges do not begin wit the same letter
  // Any Factor(S), substring, is represented by a path starting at the initial node
  // Any string in the Suffix(S') is represented by a path starting at the initial node and ending at the final node
  // Suppose that a path spelling out α∈Σ∗ ends at a node v. If a string β is always preceded by γ∈Σ∗ and α=γβ in any string x ∈ S′ such that β ∈ Factor(x), the path spelling out β also ends at node v.

  private root: SeaNode;
  private sink: SeaSinkNode;
  private source: SeaNode;
  private debug: boolean = true;
  private wordTerminator: string = "\u0000";
  private _size: number = 0;

  constructor() {

    this.root = new SeaNode(-1, null);
    (<any>this.root).id = "root";

    this.source = new SeaNode(0, this.root);
    (<any>this.source).id = "source";
  }
  
  public get size(): number {
    return this._size;
  }

  public add(word: string, sink: SeaSinkNode) {

    //word = this._terminateWord(word);
    this.sink = sink;

    (<any>this.sink).id = "sink";
    
    let updateData: [SeaNode, number] = [this.source, 0];

    //TODO: Was given an idea to instead traverse the graph until reaching the node that can no longer be traversed for the path and use that as the starting path. This version is good enough right now.
    for(let wordIdx = 0; wordIdx < word.length; wordIdx++) {

      const letter = word[wordIdx];
      
      // Initialize edges from root if needed
      if (!this.root.toEdges.has(letter)) {
        this.root.setEdge(null, letter, wordIdx, wordIdx, this.source);
      }

      (<any>this.sink).length = wordIdx + 1; // This is e as citied in the paper

      updateData = this._update(word, letter, updateData[0], updateData[1], wordIdx);
    }

    for(let wordStartIdx = 0; wordStartIdx < word.length; wordStartIdx++) {
      let termNode: SeaTermNode;
      let subNode: SeaNode = this.source;

      let wordIdx = wordStartIdx;
      let edge: SeaEdge;

      while(wordIdx < word.length) {
        edge = subNode.toEdges.get(word[wordIdx]);
        
        if(edge) {
          wordIdx += edge.partial.length;
          subNode = <SeaNode>edge.dest;
          continue;
        }
        break;
      }
      
      if (wordIdx > word.length) {
        subNode = this._splitEdge(edge.src, 0, 0, edge.partial);
        subNode.suffix = subNode; //TODO Not sure why this would be suitable
      }

      if (subNode.toEdges.has(this.wordTerminator)) {
        termNode = <SeaTermNode>subNode.toEdges.get(this.wordTerminator).dest;
      } else {
        termNode = new SeaTermNode();
        subNode.setEdge(this.wordTerminator, this.wordTerminator, 0, 0, termNode);
      }
      
      this.sink.incomingEdges.add(subNode.toEdges.get(this.wordTerminator));
      termNode.addSinkNode(this.sink);
    }

    this._size++;
    this.sink = null;

    return sink;
  }

  // Deletes word from graph if it exists (returns true) otherwise returns false.
  // Warning: Experimental, it is not proven to be the inverse operation. If it can be proven that delete(w2, CDAWG([w1, w2, ..., wx])) = add(w1, CDAWG([w3, ..., wx])) for all cases then this will no longer be experimental.
  // Remark 1: Have not seen any literature around deletion of words in a CDAWG, so that means have to be creative and accept some cost. Don't do deletions if a true CDAWG is required.
  // Remark 2: Not claiming that the graph is in a minimal form after deletion, but that the
  // word we added can no longer be reached. A best attempt at minimization will be performed.
  /*public delete(word: string): boolean {

    const sinkNode = this.findExact(word);

    if(!sinkNode) {
      return false;
    }

    const edges = Array.from(sinkNode.incomingEdges);

    this._size--;
    this._cleanup(edges);
  }
*/
  // Removes specified edges and any merges or removes any source nodes that are no longer needed
  // recursively.
  private _cleanup(edges: SeaEdge[]) {

    for (const edge of edges) {

      edge.src.removeEdge(edge.partial[0]);
    }

    for (const edge of edges) {

      const srcNode = edge.src;

      if(srcNode.length <= 0) {
        continue;
      }

      const srcNodeOutDegree = srcNode.toEdges.size;
      if(srcNodeOutDegree == 1) {
        const dest = Array.from(srcNode.toEdges.values())[0];

        this._mergeEdge(srcNode, dest);
        srcNode.removeEdge(dest.partial[0]);
      } else if (srcNodeOutDegree == 0) {

        this._cleanup(Array.from(srcNode.incomingEdges));
      }
    }
  }

  private _mergeEdge(srcNode: SeaNode, destEdge: SeaEdge) {

    const incomingEdges = Array.from(srcNode.incomingEdges);

    //TODO: I am concerned about the suffix link, I should make sure to copy it to each edges src nodes
    // but if src nodes have their own suffix link then there would be conflict. It makes me think
    // that nodes in general should live up until they have zero outgoing edges only rather than merge.
    if (destEdge.dest instanceof SeaNode) {
      destEdge.dest.length += destEdge.partial.length;
    }

    for(const incomingEdge of incomingEdges) {

      incomingEdge.src.setEdge(
        incomingEdge.partial + destEdge.partial,
        incomingEdge.partial[0],
        incomingEdge.startIdx,
        incomingEdge.endIdx + destEdge.partial.length,
        destEdge.dest,
      );
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

      const wordLengthRemaining = word.length - wordIdx;
      const partialLength = matchingEdge.partial.length;
      
      if (partialLength > wordLengthRemaining) {
        const dest = <SeaNode>matchingEdge.dest;
        if (!dest.toEdges.has("\0")) {
          break;
        }

        const termNode = <SeaTermNode>dest.toEdges.get(this.wordTerminator).dest;

        for(const sinkNode of termNode.toSinkNodes) {
          if (sinkNode.length === word.length - 1) {
            targetNode = sinkNode;
            break;
          }
        }
      } else if(matchingEdge.partial === word.substring(wordIdx, wordIdx + partialLength)) {

        if (wordIdx + partialLength === word.length) {
          
          if(matchingEdge.dest instanceof SeaTermNode) {
            for(const sinkNode of matchingEdge.dest.toSinkNodes) {
              if (sinkNode.length === word.length - 1) {
                targetNode = sinkNode;
                break;
              }
            }
          }
          
          break;
        }
        
        if(matchingEdge.dest instanceof SeaTermNode) {
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

  public findWithPrefix(word: string): Array<[string, SeaSinkNode]> {

    const result: Array<[string, SeaSinkNode]> = [];

    const context: PrefixContext = {
      "mode": TraversalMode.Traversal,
      "node": this.source,
      "wordIdx": 0,
      "edgesToScan": [],
      "traversedWord": "",
      result,
      "sinkNode": null,
    };

    this._executeTraversal(
      new PrefixTraverser(word),
      context,
    );

    return result;
  }

  public findWithSubstring(word: string): Array<[string, SeaSinkNode]> {

    const result: Array<[string, SeaSinkNode]> = [];

    const context: FindSuperStringContext = {
      "mode": TraversalMode.Traversal,
      "node": this.source,
      "wordIdx": 0,
      "edgesToScan": [],
      "traversedWord": "",
      result,
      "sinkNode": null,
    };

    this._executeTraversal(
      new FindSuperStringTraverser(word),
      context,
    );

    return result;
  }

  // Traversals are executed in an iterative manner rather than recursion.
  private _executeTraversal(
    traverser: Traverser,
    baseContext: TraversalContext,
  ) {

    // Initialize
    const result = baseContext.result;
    const traversalContexts: Array<TraversalContext> = [baseContext];
    traverser.setup(baseContext, traversalContexts);

    while(traversalContexts.length > 0) {
      const traversalContext = traversalContexts.pop();

      const shouldTraverse = traversalContext.edgesToScan.length > 0;
      if(!shouldTraverse) {
        continue;
      }

      traversalContexts.push(traversalContext); // Keep it active
      const edge = traversalContext.edgesToScan.pop();

      if (traversalContext.mode === TraversalMode.Traversal) {

        traverser.traverse(edge, traversalContext, traversalContexts);

      } else if (traversalContext.mode === TraversalMode.Collection) {
        
        traverser.collect(edge, traversalContext, traversalContexts);

      } else if (traversalContext.mode === TraversalMode.Sink) {

        const traversedWord = this.removeTerminator(traversalContext.traversedWord);

        const sinkNode = traversalContext.sinkNode;
        if (traverser.shouldAcceptSinkNode(sinkNode, traversalContext.wordIdx, traversedWord)) {
          result.push([traversedWord, sinkNode]);
        }
      }
    }
  }

  private _update(word: string, letter: string, updateNode: SeaNode, startIdx: number, endIdx: number): [SeaNode, number] {

    let prevNode: SeaNode = null;
    let updateNodePrime: SeaNode = null;
    let updateNodeNext: SeaNode = null;
    let prevEndIdx = endIdx - 1;

    while (!this._checkEndpoint(updateNode, startIdx, prevEndIdx, letter, word)) {

      if (startIdx <= prevEndIdx) {

        const possibleExtension = this._extension(updateNode, startIdx, prevEndIdx, word);
        if (updateNodePrime === possibleExtension) {

          this._redirectEdge(updateNode, startIdx, prevEndIdx, updateNodeNext, word);
          [updateNode, startIdx] = this._canonize(updateNode.suffix, startIdx, prevEndIdx, word);

          continue;
        }

        updateNodePrime = possibleExtension;
        updateNodeNext = this._splitEdge(updateNode, startIdx, prevEndIdx, word);

      } else {

        updateNodeNext = updateNode;
      }

      let termNode: SeaTermNode;
      let subNode: SeaNode;
      
      if(updateNodeNext.toEdges.has(letter)) {
        subNode = <SeaNode>updateNodeNext.toEdges.get(letter).dest;
      } else {
        subNode = new SeaNode(updateNodeNext.length + word.length - endIdx, updateNodeNext);
        const endLen = word.length;
        const partial = word.substring(endIdx, endLen);
        
        updateNodeNext.setEdge(partial, letter, endIdx, endIdx + partial.length - 1, subNode);
      }

      if (subNode.toEdges.has(this.wordTerminator)) {
        termNode = <SeaTermNode>subNode.toEdges.get(this.wordTerminator).dest;
      } else {
        termNode = new SeaTermNode();
        
        subNode.setEdge(this.wordTerminator, this.wordTerminator, 0, 0, termNode);
      }
      
      this.sink.incomingEdges.add(subNode.toEdges.get(this.wordTerminator));
      termNode.addSinkNode(this.sink);

      if (prevNode != null) {
        prevNode.suffix = updateNodeNext;
      }

      prevNode = updateNodeNext;

      let snapNode = updateNode;
      
      [updateNode, startIdx] = this._canonize(updateNode.suffix, startIdx, prevEndIdx, word);

      if (this.debug && !updateNode) {
        console.error("empty update node", word, letter, startIdx, endIdx, snapNode);
      }
    }

    if (prevNode != null) {
      prevNode.suffix = updateNode;
    }

    return this._separateNode(updateNode, startIdx, endIdx, word);
  }

  private _checkEndpoint(src: SeaNode, startIdx: number, endIdx: number, letter: string, word: string): boolean {

    if (startIdx <= endIdx) {

      const edge = src.toEdges.get(word[startIdx]);
      word = edge.partial;
      
      return letter === word[endIdx - startIdx + 1];
    }

    return src.toEdges.has(letter);
  }

  private _canonize(node: SeaNode, startIdx: number, endIdx: number, word: string): [SeaNode, number] {

    if (startIdx > endIdx) {
      return [node, startIdx];
    }

    let edge = node.toEdges.get(word[startIdx]);
    
    if(this.debug && edge == null) {
      console.log("bad edge", startIdx, endIdx, `[${word[startIdx]}]`, word, node)
    }
    let edgeIdxDiff = getEndIdx(edge) - edge.startIdx;

    while(edgeIdxDiff <= endIdx - startIdx) {

      startIdx += edgeIdxDiff + 1;

      if (!(edge.dest instanceof SeaNode)) {
        
        throw new Error("Only SeaNodes should be returned by canonize");
      }

      node = edge.dest;

      if(startIdx <= endIdx) {
        edge = node.toEdges.get(word[startIdx]);
      }
      
      edgeIdxDiff = getEndIdx(edge) - edge.startIdx;
    }
    
    return [node, startIdx];
  }

  private _extension(node: SeaNode, startIdx: number, endIdx: number, word: string): SeaNode {

    if (startIdx > endIdx) {
      return node;
    }

    const letter = word[startIdx];

    const edge = node.toEdges.get(letter);

    return <SeaNode>edge.dest;
  }

  private _redirectEdge(src: SeaNode, startIdx: number, endIdx: number, dest: SeaNode, word: string) {

    const letter = word[startIdx];
    const edge = src.toEdges.get(letter);
    const subStringIdxDiff = endIdx - startIdx;
    const newEndIdx = edge.startIdx + subStringIdxDiff;
    const newPartial = edge.partial.substring(0, subStringIdxDiff + 1);
    
    src.setEdge(newPartial, letter, edge.startIdx, newEndIdx, dest);
  }

  // Adds a node into the middle of an edge, which results src -> newNode -> src.dest
  // Just think of it as splitting a string
  private _splitEdge(src: SeaNode, startIdx: number, endIdx: number, word: string): SeaNode {
    
    const letter = word[startIdx]; // This is the k variable
    const edge = src.toEdges.get(letter);
    const snapWord = word;
    word = edge.partial; // Word at edge
    
    const subStringIdxDiff = endIdx - startIdx; // This is p - k
    const newLetter = word[subStringIdxDiff + 1];

    const newStartIdx = edge.startIdx + subStringIdxDiff + 1;
    
    let newNode = new SeaNode(src.length + subStringIdxDiff + 1, null);
    
    // newNode.edge[letter] -> src.dest
    
    if (this.debug && (typeof newLetter === "undefined" || newLetter === null)) {
      console.log(`[snap, ${snapWord}, ${snapWord.length}]`, `[curr, ${word}, ${word.length}]`, `edge startIdx: ${edge.startIdx} => ${newStartIdx}`, `edge endIdx: ${edge.endIdx} => ${endIdx}`, `[snap: ${letter}, curr: ${newLetter}]`, subStringIdxDiff, startIdx, endIdx);
      console.log(require("util").inspect(src, {"depth": 4}));
      console.error("^ New letter should not be empty. Info above.");
    }
    newNode.setEdge(edge.partial.substring(subStringIdxDiff + 1), newLetter, newStartIdx, edge.endIdx, edge.dest);

    // src.edge[letter] -> newNode
    src.setEdge(edge.partial.substring(0, subStringIdxDiff + 1), letter, edge.startIdx, newStartIdx - 1, newNode);

    return newNode;
  }

  private _separateNode(src: SeaNode, startIdx: number, endIdx: number, word: string): [SeaNode, number] {

    let canonizedData = this._canonize(src, startIdx, endIdx, word);

    if (canonizedData[1] <= endIdx) {
      return canonizedData;
    }

    const canonNode = canonizedData[0];
    const sepLength = src.length + endIdx - startIdx + 1;

    if (canonNode.length === sepLength) {
      return canonizedData;
    }

    const sepNode = canonNode.clone();
    sepNode.length = sepLength;
    (<any>sepNode).id = "sep";
    (<any>sepNode).sepFrom = canonNode;
    sepNode.suffix = canonNode.suffix;
    canonNode.suffix = sepNode;

    if(sepNode.toEdges.has(this.wordTerminator)) {
      
      this.sink.incomingEdges.add(sepNode.toEdges.get(this.wordTerminator));
    }

    while(true) {
      const edge = src.toEdges.get(word[startIdx]);

      src.setEdge(edge.partial, edge.partial[0], edge.startIdx, edge.endIdx, sepNode);

      [src, startIdx] = this._canonize(src.suffix, startIdx, endIdx - 1, word);
      
      let newCanonizedNodePair = this._canonize(src, startIdx, endIdx, word);

      if ((canonizedData[0] !== newCanonizedNodePair[0]) || (canonizedData[1] !== newCanonizedNodePair[1])) {
        break;
      }
    }
    
    return [sepNode, endIdx + 1];
  }

  private _terminateWord(word: string): string {
    return word + this.wordTerminator;
  }

  private removeTerminator(word:string): string {
    return word.endsWith(this.wordTerminator) ? word.substring(0, word.length - 1) : word;
  }
}

function getEndIdx(edge: SeaEdge): number {

  return edge.endIdx;
}

enum TraversalMode {
  Traversal,
  Collection,
  Sink,
}

interface TraversalContext {
  mode: TraversalMode,
  node: SeaNode,
  edgesToScan: Array<SeaEdge>;
  traversedWord: string,
  wordIdx: number,
  result: Array<[string, SeaSinkNode]>,
  sinkNode: SeaSinkNode,
}

interface PrefixContext extends TraversalContext {}

interface FindSuperStringContext extends TraversalContext {
}

interface Traverser {
  // Initial selection of edges
  setup(ontext: TraversalContext, traversalContexts: Array<TraversalContext>);

  // Moves down the graph and initiates further traversals or collections
  traverse(matchingEdge: SeaEdge, context: TraversalContext, secondaryTraversalContexts: Array<TraversalContext>);

  // After pruning select sink edges that will be considered
  collect(edge: SeaEdge, traversalContext: TraversalContext, traversalContexts: TraversalContext[]);

  // Should sink node be added to the result set
  shouldAcceptSinkNode(sinkNode: SeaSinkNode, currentWordIdx: number, word: string): boolean;
}

class PrefixTraverser implements Traverser {
  
  constructor(
    private _prefixWord: string,
  ) {}
  
  setup(context: PrefixContext, traversalContexts: Array<TraversalContext>) {
    
    let wordIdx = context.wordIdx;
    const node = context.node;

    const wordFirstChar = this._prefixWord[wordIdx];
    const matchingEdge = node.toEdges.get(wordFirstChar);

    if(!matchingEdge) {
      return;
    }

    const proposedContext: PrefixContext = {
      "mode": TraversalMode.Traversal,
      "node": node,
      wordIdx,
      "traversedWord": context.traversedWord,
      "edgesToScan": [matchingEdge],
      "result": context.result,
      "sinkNode": null,
    };

    traversalContexts.push(proposedContext);
  }

  // Moves down the graph and initiates further traversals or collections
  traverse(matchingEdge: SeaEdge, context: TraversalContext, traversalContexts: Array<TraversalContext>) {
    
    const wordIdx = context.wordIdx;
    const word =  this._prefixWord;
    const partialLength = matchingEdge.partial.length;
    const wordLengthRemaining = word.length - wordIdx;
    
    if (wordLengthRemaining < 0) {
      return;
    }

    if(
      partialLength > wordLengthRemaining && 
      matchingEdge.partial.substring(0, wordLengthRemaining) === word.substring(wordIdx, wordIdx + wordLengthRemaining)
    ) {
      const traversalContext: TraversalContext = {
        "mode": TraversalMode.Collection,
        "node": null,
        "result": context.result,
        "wordIdx": wordIdx,
        "traversedWord": context.traversedWord,
        "edgesToScan": [matchingEdge],
        "sinkNode": context.sinkNode,
      }
      traversalContexts.push(traversalContext);

    } else if(matchingEdge.partial === word.substring(wordIdx, wordIdx + partialLength)) {

      const wordSubstring = word.substring(wordIdx, wordIdx + partialLength);
      if (partialLength === wordLengthRemaining) {
        const traversalContext: TraversalContext = {
          "mode": TraversalMode.Collection,
          "node": null,
          "result": context.result,
          "wordIdx":  wordIdx,
          "traversedWord": context.traversedWord,
          "edgesToScan": [matchingEdge],
          "sinkNode": context.sinkNode,
        };
        traversalContexts.push(traversalContext);
      } else if(partialLength < wordLengthRemaining) {

        if (matchingEdge.dest instanceof SeaNode) {
          const traversalContext: TraversalContext = {
            "mode": TraversalMode.Traversal,
            "node": matchingEdge.dest,
            "result": context.result,
            "wordIdx":  wordIdx + partialLength,
            "traversedWord": context.traversedWord + wordSubstring,
            "edgesToScan": Array.from(matchingEdge.dest.toEdges.values()),
            "sinkNode": context.sinkNode,
          };
          traversalContexts.push(traversalContext);
        }
      }
    } 
  }

  // Collection process is a series of traversals to collect the full word after the initial pruning
  collect(edge: SeaEdge, traversalContext: TraversalContext, traversalContexts: TraversalContext[]) {
    
    const node = edge.dest;
    const traversedWord = traversalContext.traversedWord + edge.partial;

    if(node instanceof SeaNode) {

      const newTraversalContext: TraversalContext = {
        "mode": TraversalMode.Collection,
        "node": null,
        "result": traversalContext.result,
        "wordIdx": traversalContext.wordIdx + edge.partial.length,
        traversedWord,
        "edgesToScan": Array.from(node.toEdges.values()),
        "sinkNode": traversalContext.sinkNode,
      };
      traversalContexts.push(newTraversalContext);
    } else if (node instanceof SeaTermNode) {
      
      for(const sinkNode of node.toSinkNodes.values()) {

        const newTraversalContext: TraversalContext = {
          "mode": TraversalMode.Sink,
          "node": null,
          "result": traversalContext.result,
          "wordIdx": traversalContext.wordIdx,
          traversedWord,
          "edgesToScan": [edge],
          "sinkNode": sinkNode,
        };

        traversalContexts.push(newTraversalContext);
      }
    }
  }

  // Only accept words that match the original word - terminator
  shouldAcceptSinkNode(sinkNode: SeaSinkNode, _currentWordIdx: number, finalWord: string) {
    
    return finalWord.length === sinkNode.length;
  }
}

/**
 * Super strings are strings that contain a substring.
 * In order for this to work, we need to traverse until getting to a sink node.
 * Once we have a sink node associated with the substring, we can backtrack using
 * the source node to reconstruct the original string.
 */
class FindSuperStringTraverser implements Traverser {
  
  private _duplicateFilter : ScalableCuckooFilter;

  constructor(
    private _substringWord: string,
  ) {
    this._duplicateFilter = new ScalableCuckooFilter();
  }
  
  setup(context: FindSuperStringContext, traversalContexts: Array<TraversalContext>) {

    let wordIdx = context.wordIdx;
    const node = context.node;

    const wordFirstChar = this._substringWord[wordIdx];
    const matchingEdge = node.toEdges.get(wordFirstChar);

    if(!matchingEdge) {
      return;
    }

    const proposedContext: FindSuperStringContext = {
      "mode": TraversalMode.Traversal,
      "node": node,
      wordIdx,
      "traversedWord": context.traversedWord,
      "edgesToScan": [matchingEdge],
      "result": context.result,
      "sinkNode": context.sinkNode,
    };

    traversalContexts.push(proposedContext);
  }

  // This function never returns true because the initial forward edge selection process is done once
  // since we will have a sufficient suffix. that prunes the search space.
  // The idea is to then traverse to the sink.
  // Once we have a sink, then initiate collection.
  traverse(matchingEdge: SeaEdge, context: FindSuperStringContext, traversalContexts: Array<TraversalContext>) {
    const wordIdx = context.wordIdx;
    const partialLength = matchingEdge.partial.length;

    if (matchingEdge.dest instanceof SeaTermNode) {
    
      for (const sinkNode of matchingEdge.dest.toSinkNodes.values()) {
        
        const nextTraversalContext: FindSuperStringContext = {
          "mode": TraversalMode.Collection,
          "node": null,
          "result": context.result,
          "wordIdx": 0,
          "traversedWord": "",
          "edgesToScan": Array.from(sinkNode.incomingEdges.values()),
          "sinkNode": sinkNode,
        };
        traversalContexts.push(nextTraversalContext);
      }
    } else if (matchingEdge.dest instanceof SeaNode) {
      
      const nextTraversalContext: FindSuperStringContext = {
        "mode": TraversalMode.Traversal,
        "node": matchingEdge.dest,
        "wordIdx": wordIdx + partialLength,
        "traversedWord": context.traversedWord + matchingEdge.partial,
        "edgesToScan": Array.from(matchingEdge.dest.toEdges.values()),
        "result": context.result,
        "sinkNode": context.sinkNode,
      };
      traversalContexts.push(nextTraversalContext);
    }
  }

  // Collection faciliates going backward through the links and reconstructing the original strings
  collect(edge: SeaEdge, traversalContext: FindSuperStringContext, traversalContexts: TraversalContext[]) {

    const node = edge.src;
    const traversedWord = edge.partial + traversalContext.traversedWord;
    
    if(node.length > 0) {
      const newTraversalContext: FindSuperStringContext = {
        "mode": TraversalMode.Collection,
        "node": node,
        "result": traversalContext.result,
        "wordIdx": 0,
        traversedWord,
        "edgesToScan": Array.from(node.incomingEdges.values()),
        "sinkNode": traversalContext.sinkNode,
      };
      traversalContexts.push(newTraversalContext);

      return;
    }

    const newTraversalContext: FindSuperStringContext = {
      "mode": TraversalMode.Sink,
      "node": null,
      "result": traversalContext.result,
      "wordIdx": 0,
      traversedWord,
      "edgesToScan": [edge],
      "sinkNode": traversalContext.sinkNode,
    };
    traversalContexts.push(newTraversalContext);
  }

  // Only accept words that match the original word - terminator
  shouldAcceptSinkNode(sinkNode: SeaSinkNode, currentWordIdx: number, finalWord: string) {
    
    if(finalWord.length === sinkNode.length && !this._duplicateFilter.contains(finalWord)) {

      this._duplicateFilter.add(finalWord);
      return true;
    }

    return false;
  }
}