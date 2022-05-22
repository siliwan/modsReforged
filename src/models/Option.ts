export type Option<T> = 
    Some<T> | None

export const None: Option<never> = { _tag: 'None' }
export const Some: <T>(value: T) => Option<T> = <T>(value: T): Option<T> => ({ _tag: "Some", value: value })

export function is_Some<T>(option: Option<T>): boolean {
    return option._tag == 'Some'
}

export function is_None<T>(option: Option<T>): boolean {
    return option._tag == 'None'
}

interface None {
    readonly _tag: 'None'
}

interface Some<A> {
    readonly _tag: 'Some'
    readonly value: A
}
      