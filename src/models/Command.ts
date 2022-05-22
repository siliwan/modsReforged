import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";
import { Result, fromResult, fromError } from "./Result";

export type Command<TArgs, TRes> = {
    commandName: string
} & TArgs

export async function Invoke<TRes>(cmd: string, args?: InvokeArgs | undefined, throwOnError?: boolean): Promise<Result<TRes, unknown>> {
    try {
        return fromResult(await invoke<TRes>(cmd, args))
    } catch (error) {
        if(throwOnError) throw error;

        return fromError(error);        
    }
}