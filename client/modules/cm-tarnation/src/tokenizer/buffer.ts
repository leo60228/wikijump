import { search } from "wj-util"
import type { SerializedTokenizerContext, Token } from "../types"
import { Chunk } from "./chunk"
import type { TokenizerContext } from "./context"

/** Number of tokens per chunk. */
const CHUNK_SIZE = 32

export class TokenizerBuffer {
  /** The actual array of chunks that the buffer manages. */
  buffer: Chunk[] = []

  /** The last chunk in the buffer. */
  get last() {
    if (!this.buffer.length) return null
    return this.buffer[this.buffer.length - 1]
  }

  /** Retrieves a `Chunk` from the buffer. */
  get(index: number): Chunk | null {
    return this.buffer[index] ?? null
  }

  /** Compiles every chunk and returns the resultant tokens. */
  compile() {
    const compiled: Token[] = []
    for (let idx = 0; idx < this.buffer.length; idx++) {
      const chunk = this.buffer[idx]
      compiled.push(...chunk.compile())
    }
    return compiled
  }

  /**
   * Adds tokens to the buffer, splitting them automatically into chunks.
   *
   * @param context - The context to track position and stack state with.
   * @param tokens - The tokens to add to the buffer.
   */
  add(context: TokenizerContext | SerializedTokenizerContext, ...tokens: Token[]) {
    for (let idx = 0; idx < tokens.length; idx++) {
      const token = tokens[idx]

      // last chunk, or new chunk if last is too big or doesn't exist
      const chunk =
        this.last && this.last.size < CHUNK_SIZE
          ? this.last
          : new Chunk(context.pos, context.stack)

      chunk.add(token)

      // add chunk if it's a new one
      if (this.last !== chunk) this.buffer.push(chunk)
    }
  }

  /**
   * Splits the buffer into a left and right section. The left section
   * takes the indexed chunk, which will have its tokens cleared.
   *
   * @param index - The chunk index to split on.
   */
  split(index: number) {
    if (!this.get(index)) throw new Error("Tried to split buffer on invalid index!")

    if (this.buffer.length <= 1) return { left: this, right: new TokenizerBuffer() }

    const leftRaw = this.buffer.slice(0, index + 1)
    const rightRaw = this.buffer.slice(index + 1)

    // this buffer "is" left, so we only need to make right
    this.buffer = leftRaw
    const right = new TokenizerBuffer()
    right.buffer = rightRaw

    if (this.last) this.last.setTokens([])

    return { left: this, right }
  }

  /** Binary search comparator function. */
  private searchCmp = ({ pos }: Chunk, target: number) => pos === target || pos - target

  /**
   * Searches for the closest chunk to the given position.
   *
   * @param pos - The position to find.
   * @param side - The side to search on. -1 is left (before), 1 is right
   *   (after). 0 is the default, and it means either side.
   * @param precise - If true, the search will require an exact hit. If the
   *   search misses, it will return `null` for both the token and index.
   */
  search(pos: number, side: 1 | 0 | -1 = 0, precise = false) {
    if (this.buffer.length === 0) return { chunk: null, index: null }

    const result = search(this.buffer, pos, this.searchCmp, { precise })
    if (!result) return { chunk: null, index: null }

    let { index } = result
    let chunk = this.buffer[index]

    // direct hit or we don't care about sidedness
    if (chunk.pos === pos || side === 0) return { chunk, index }

    // correct for sidedness
    while (chunk && side === 1 ? chunk.pos < pos : chunk.pos > pos) {
      index = side === 1 ? index + 1 : index - 1
      chunk = this.buffer[index]
    }

    // no valid chunks
    if (!chunk) return { chunk: null, index: null }

    return { chunk, index }
  }
}
