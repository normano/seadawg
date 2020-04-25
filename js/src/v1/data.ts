import { SeaEdge } from "./core";

/**
 * This interface exists to ensure that the length always exists and to save on memory.
 * Anything else that is needed should be in an implementer.
 */
export interface SeaSinkNode {
  readonly length: number;
  incomingEdges: Set<SeaEdge>;
}

/**
 * Default sink that only stores length.
 */
export class SeaDefaultSinkNode implements SeaSinkNode {

  public readonly length: number = 0;
  public readonly incomingEdges: Set<SeaEdge> = new Set();

  constructor() {}
}

/**
 * Sink node that can store data.
 */
export class SeaValueSinkNode<V = any> implements SeaSinkNode {

  public readonly length: number = 0;
  public readonly incomingEdges: Set<SeaEdge> = new Set();

  constructor(
    public data: V,
  ) {}
}