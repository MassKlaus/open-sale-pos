import { invoke } from "@tauri-apps/api/tauri";
import { InvokeArgs } from "@tauri-apps/api/tauri";

type Arg = {
    name: string
}

type Product = {
    name: string
}

export type commands = {
    greet: {
        arg: Arg,
        return: string
    },
    fetch_products: {
        arg: {
            name: string
        },
        return: number
    },
    create_tag_category: {
        arg: {
            categoryName: string
        },
        return: void
    },
    get_tag_categories: {
        arg: void,
        return: any[]
    }
};

export function command<T extends keyof commands>(cmd: T, args?: commands[T]['arg']): Promise<commands[T]['return']>
{
    return invoke<commands[T]['return']>(cmd, args as InvokeArgs | undefined);
}