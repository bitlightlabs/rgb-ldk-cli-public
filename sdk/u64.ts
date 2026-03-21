export type U64Input = U64 | string | number | bigint;

export class U64 {
  static readonly MIN = 0n;
  static readonly MAX = (1n << 64n) - 1n;

  readonly #value: bigint;

  private constructor(value: bigint) {
    this.#value = value;
  }

  static from(input: U64Input): U64 {
    if (input instanceof U64) return input;

    if (typeof input === "bigint") {
      return U64.#checked(input);
    }

    if (typeof input === "number") {
      if (!Number.isFinite(input) || !Number.isInteger(input) || input < 0) {
        throw new Error(`Invalid u64 number: ${input}`);
      }
      if (!Number.isSafeInteger(input)) {
        throw new Error(`Unsafe u64 number (exceeds MAX_SAFE_INTEGER): ${input}`);
      }
      return U64.#checked(BigInt(input));
    }

    if (typeof input === "string") {
      if (!/^[0-9]+$/.test(input)) {
        throw new Error(`Invalid u64 string: ${input}`);
      }
      return U64.#checked(BigInt(input));
    }

    throw new Error("Unsupported u64 input");
  }

  static #checked(value: bigint): U64 {
    if (value < U64.MIN) throw new Error(`u64 underflow: ${value.toString()}`);
    if (value > U64.MAX) throw new Error(`u64 overflow: ${value.toString()}`);
    return new U64(value);
  }

  toBigInt(): bigint {
    return this.#value;
  }

  toString(): string {
    return this.#value.toString(10);
  }

  toJSON(): string {
    return this.toString();
  }

  eq(other: U64Input): boolean {
    return this.#value === U64.from(other).#value;
  }

  cmp(other: U64Input): -1 | 0 | 1 {
    const o = U64.from(other).#value;
    if (this.#value < o) return -1;
    if (this.#value > o) return 1;
    return 0;
  }

  add(other: U64Input): U64 {
    return U64.#checked(this.#value + U64.from(other).#value);
  }

  sub(other: U64Input): U64 {
    return U64.#checked(this.#value - U64.from(other).#value);
  }

  mul(other: U64Input): U64 {
    return U64.#checked(this.#value * U64.from(other).#value);
  }

  div(other: U64Input): U64 {
    const o = U64.from(other).#value;
    if (o === 0n) throw new Error("u64 division by zero");
    return U64.#checked(this.#value / o);
  }

  mod(other: U64Input): U64 {
    const o = U64.from(other).#value;
    if (o === 0n) throw new Error("u64 modulo by zero");
    return U64.#checked(this.#value % o);
  }
}

export function u64(input: U64Input): U64 {
  return U64.from(input);
}

export function isU64(value: unknown): value is U64 {
  return value instanceof U64;
}
