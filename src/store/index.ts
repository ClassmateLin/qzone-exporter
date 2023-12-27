import { Store } from "tauri-plugin-store-api";

const store = new Store(".settings.dat");

export async function set_value(key: string, value: any) {
    await store.set(key, value)
}


export async function get_value(key: string) {
    return await store.get(key)
}

export async function save_store() {
    await store.save()
}