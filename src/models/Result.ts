export type Result<T, E = never> =
| { /** @internal */ isOk: true; value: T }
| { /** @internal */ isOk: false; error: E };

export function isOk<TRes, E> (res: Result<TRes, E>) : res is Result<TRes, never> {
  return res.isOk
}

export function isErr<TRes, E> (res: Result<TRes, E>) : res is Result<never, E> {
  return !res.isOk
}

export function fromResult<TRes> (res: TRes) : Result<TRes> {
  return {
    isOk: true,
    value: res
  }
}

export function fromError<TError> (error: TError) : Result<never, TError> {
  return {
    isOk: false,
    error: error
  }
}
