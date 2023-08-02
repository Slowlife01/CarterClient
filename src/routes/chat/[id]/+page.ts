import { invoke } from "@tauri-apps/api/tauri";

export async function load({ params }) {
    const chat = await invoke("get_chat", {
        chatId: params.id
    });

    return { chat };
}
