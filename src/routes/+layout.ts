import { invoke } from "@tauri-apps/api/tauri";

export const prerender = "auto";
export const ssr = false;

export async function load({ depends, route }) {
    depends("data");
    console.log({ route })

    if (route.id === "/") return;

    const is_db_ready = await invoke("is_db_ready");
    if (!is_db_ready) return;

    console.log("Loading data...");
    const agents = await invoke<{ isSelected: boolean; id: string }[] | null>("list_agents");

    let currentAgent: { id: string; isSelected: boolean } | undefined = agents?.find((agent) => agent.isSelected);
    let chats: { id: string; title: string; messages: { content: string }[] }[] = [];

    if (!currentAgent && agents?.length > 0) {
        currentAgent = await invoke("set_selected_agent", {
            id: agents[0].id
        });
    }

    if (currentAgent) {
        chats = await invoke("list_chats", {
            agentId: currentAgent.id
        });
    }

    return {
        currentAgent,
        agents,
        chats
    };
}
