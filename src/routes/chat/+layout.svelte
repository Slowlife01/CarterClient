<script lang="ts">
  import { page } from "$app/stores";

  import Subsidebar from "$lib/Subsidebar.svelte";
  import ChatIcon from "$lib/assets/ChatIcon.svelte";

  import { Tooltip } from "flowbite-svelte";
  import { writable } from "svelte/store";

  export let data: {
    currentAgent: {
      id: string;
      name: string;
      key: string;
    };
    agents: {}[];
    chats: {
      id: string;
      title: string;
      agentId: string;
      messages: {
        content?: string;
        createdAt?: string;
        isFromAgent?: boolean;
      }[];
    }[];
  };

  let dataList = writable([]);
  $: chats = data.chats.sort((a, b) => {
    if (a.messages.at(-1) && b.messages.at(-1)) {
      return (
        Date.parse(b.messages[0]?.createdAt) -
        Date.parse(a.messages[0]?.createdAt)
      );
    }
  });

  $: dataList.set(
    chats.map((chat) => {
      return {
        title: chat.title,
        description: chat.messages[0]?.content,
        selected: chat.id === $page.params.id,
        href: `/chat/${chat.id}`,
        date: chat.messages[0]?.createdAt,
      };
    })
  );
</script>

<div class="chat-container my-5 rounded-l-xl flex flex-1 flex-row bg-[#121112]">
  <Subsidebar
    icon={ChatIcon}
    {dataList}
    title="Chats"
    defaultDescription="Empty chat"
    canCreate={data && data.agents.length > 0}
    pathToCreate="/chat/create"
  >
    {#if data.currentAgent}
      <div
        class="agent border border-[#3e3f40] flex flex-row mt-1 mb-4 space-x-2 rounded-lg p-1 mr-2"
      >
        <div class="w-11 h-9 ml-1 mt-1">
          <img
            class="w-full h-full rounded-full"
            src="https://cdn.discordapp.com/avatars/1113620041125089340/21ee632fbe7561b2243e7a2a9d7f4373.webp?size=1280"
            alt="avatar"
          />
        </div>
        <div class="flex flex-row justify-between items-center w-full">
          <div class="flex flex-col items-start">
            <span class="text-xs text-gray-400">Your AI Agent</span>
            <span class="text-xs text-gray-300">{data.currentAgent.name}</span>
          </div>
          <div
            class="switch-agent cursor-pointer mr-2 hover:bg-[#3e3f40] p-1 rounded-lg"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="fill-white"
              height="1em"
              viewBox="0 0 576 512"
              ><path
                d="M272 416c17.7 0 32-14.3 32-32s-14.3-32-32-32H160c-17.7 0-32-14.3-32-32V192h32c12.9 0 24.6-7.8 29.6-19.8s2.2-25.7-6.9-34.9l-64-64c-12.5-12.5-32.8-12.5-45.3 0l-64 64c-9.2 9.2-11.9 22.9-6.9 34.9s16.6 19.8 29.6 19.8l32 0 0 128c0 53 43 96 96 96H272zM304 96c-17.7 0-32 14.3-32 32s14.3 32 32 32l112 0c17.7 0 32 14.3 32 32l0 128H416c-12.9 0-24.6 7.8-29.6 19.8s-2.2 25.7 6.9 34.9l64 64c12.5 12.5 32.8 12.5 45.3 0l64-64c9.2-9.2 11.9-22.9 6.9-34.9s-16.6-19.8-29.6-19.8l-32 0V192c0-53-43-96-96-96L304 96z"
              /></svg
            >
            <Tooltip triggeredBy=".switch-agent">Switch Agent</Tooltip>
          </div>
        </div>
      </div>
    {:else}
      <div
        class="agent border border-[#3e3f40] flex flex-col items-center mb-2 rounded-lg p-1"
      >
        <div class="text-gray-300 w-fit">No agent available</div>
        <a class="w-fit" href="/agent/create">Add agent</a>
      </div>
    {/if}
  </Subsidebar>
  <slot />
</div>
