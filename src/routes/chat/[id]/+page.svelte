<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { afterUpdate, onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  import { SyncLoader } from "svelte-loading-spinners";

  import ChatInput from "$lib/ChatInput.svelte";
  import CarterIcon from "$lib/assets/carter.svg";

  import { afterNavigate, goto, invalidate } from "$app/navigation";

  import SvelteMarkdown, { type Renderers } from "svelte-markdown";
  import Codeblock from "$lib/Codeblock.svelte";
  import ChatIcon from "$lib/assets/ChatIcon.svelte";

  const renderers: Partial<Renderers> = {
    code: Codeblock,
  };

  export let data: {
    chat: {
      id: string;
      agentKey: string;
      title: string;
      messages: {
        content?: string;
        isFromAgent?: boolean;
      }[];
    };
  };

  let messages = writable(
    new Map<
      string,
      {
        content?: string;
        isFromAgent?: boolean;
      }[]
    >()
  );

  $: chat = data.chat;
  let message = writable("");

  let element: HTMLDivElement;
  let deleteButton: HTMLButtonElement;

  let isTypingMap = writable(new Map());

  $: {
    if (chat.id && !$messages.has(chat.id)) {
      mapSet(messages, chat.id, chat.messages);
    }
  }

  afterUpdate(() => {
    element.scroll({ top: element.scrollHeight, behavior: "smooth" });
  });

  afterNavigate(() => {
    if (chat.id) {
      if (deleteButton) deleteButton.classList.remove("confirmation-bg");

      mapSet(messages, chat.id, chat.messages);
      if (localStorage.getItem(`chat:${chat.id}:opener`)) {
        sendMessage("opener");
        localStorage.removeItem(`chat:${chat.id}:opener`);
      }
    }
  });

  async function sendMessage(type: "opener" | "chat", input?: string) {
    let chatId = chat.id;
    let agentKey = chat.agentKey;

    localStorage.removeItem(`input:${chatId}`);
    message.set("");

    if (type === "opener") {
      mapSet(messages, chatId, [
        ...$messages.get(chatId),
        {
          isFromAgent: true,
        },
      ]);
    } else {
      mapSet(messages, chatId, [
        ...$messages.get(chatId),
        {
          content: input,
        },
        {
          isFromAgent: true,
        },
      ]);
    }

    mapSet(isTypingMap, chatId, true);
    const response = await invoke<CarterResponse>(type, {
      chatId: chatId,
      agentKey,
      input,
    });
    mapDelete(isTypingMap, chatId);

    await invalidate("data");
    if (response.output.audio && chatId === chat.id) {
      const audio = new Audio(response.output.audio);
      audio.play();
    }

    mapSet(messages, chatId, [
      ...$messages.get(chatId).slice(0, type === "chat" ? -1 : 0),
      {
        content: response.output.text,
        isFromAgent: true,
      },
    ]);
  }

  let deteteClickedAt;
  async function handleDelete() {
    if (deleteButton.classList.contains("confirmation-bg")) {
      if (Date.now() - deteteClickedAt > 1000) {
        await invoke("delete_chat", { id: chat.id });
        goto("/chat");
      }
    } else {
      deleteButton.classList.add("confirmation-bg");
      deteteClickedAt = Date.now();

      setTimeout(() => {
        if (deleteButton) deleteButton.classList.remove("confirmation-bg");
      }, 3000);
    }
  }

  function mapSet(map: Writable<Map<any, any>>, key: string, value: any) {
    map.update((map) => map.set(key, value));
  }

  function mapDelete(map: Writable<Map<any, any>>, key: string) {
    map.update((map) => {
      map.delete(key);

      return map;
    });
  }

  interface CarterResponse {
    output?: {
      text?: string;
      audio?: string;
    };
  }
</script>

<div class="relative flex flex-col w-full border-[#3e3f40]">
  <div
    class="absolute left-0 right-0 pr-9 z-[100] py-2 pb-2 mb-2 border-b border-[#3e3f40] justify-between bg-[#121112]/70 backdrop-blur-sm"
  >
    <div class="flex justify-between items-center">
      <div class="flex">
        <svelte:component
          this={ChatIcon}
          className="fill-white mt-2 ml-3"
          height="1.2em"
          width="1.2em"
        />
        <h1 class="text-white text-xl mt-1 ml-3">{chat.title}</h1>
      </div>
      <div class="flex space-x-2">
        <button
          on:click={handleDelete}
          bind:this={deleteButton}
          class="flex items-center justify-center w-9 h-9 rounded-md bg-[#1e1e1e] hover:bg-[#2e2e2e] transition-colors duration-200 ease-in-out border-2 border-[#3e3f40]"
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            class="z-[90] fill-white"
            height="1em"
            viewBox="0 0 448 512"
            ><path
              d="M170.5 51.6L151.5 80h145l-19-28.4c-1.5-2.2-4-3.6-6.7-3.6H177.1c-2.7 0-5.2 1.3-6.7 3.6zm147-26.6L354.2 80H368h48 8c13.3 0 24 10.7 24 24s-10.7 24-24 24h-8V432c0 44.2-35.8 80-80 80H112c-44.2 0-80-35.8-80-80V128H24c-13.3 0-24-10.7-24-24S10.7 80 24 80h8H80 93.8l36.7-55.1C140.9 9.4 158.4 0 177.1 0h93.7c18.7 0 36.2 9.4 46.6 24.9zM80 128V432c0 17.7 14.3 32 32 32H336c17.7 0 32-14.3 32-32V128H80zm80 64V400c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16zm80 0V400c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16zm80 0V400c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16z"
            /></svg
          ></button
        >
        <button
          class="flex items-center justify-center w-9 h-9 rounded-md bg-[#1e1e1e] hover:bg-[#2e2e2e] transition-colors duration-200 ease-in-out border-2 border-[#3e3f40]"
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            class="fill-white"
            height="1em"
            viewBox="0 0 512 512"
            ><path
              d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"
            /></svg
          ></button
        >
      </div>
    </div>
  </div>
  <div class="flex flex-col flex-1 overflow-hidden">
    <div
      bind:this={element}
      class="chat-messages h-full relative flex flex-col items-start pt-16 overflow-auto"
    >
      {#each $messages.get(chat.id) as message}
        <div
          class="chat-message flex items-end relative w-full my-5"
          data-user={message.isFromAgent ? "Carter" : "You"}
        >
          <div
            class="chat-text data-[user=Carter]:bg-[#141414] data-[user=You]:bg-[#161515] text-base flex flex-col break-words h-full text-white/90 border-[#3e3f40] border rounded-lg my-2 mx-6 w-full p-5 pr-7 pb-10"
          >
            {#if message.content}
              <SvelteMarkdown
                source={message.content}
                {renderers}
                options={{ breaks: true }}
              />
            {:else}
              <SyncLoader size="35" color="white" />
            {/if}
          </div>
          <div class="user-icon-container absolute bottom-0">
            <img
              class="h-12 w-12 rounded-xl"
              src={CarterIcon}
              alt="user-icon"
            />
          </div>
        </div>
      {/each}
    </div>
    <ChatInput
      id={chat.id}
      {message}
      {isTypingMap}
      onSend={(input) => sendMessage("chat", input)}
    />
  </div>
</div>

<style>
  .chat-messages::-webkit-scrollbar {
    width: 6px;
  }

  .chat-messages::-webkit-scrollbar-track {
    background: transparent;
  }

  .chat-messages::-webkit-scrollbar-thumb {
    background: #888;
    border-radius: 3px;
  }

  .chat-messages::-webkit-scrollbar-thumb:hover {
    background: #555;
  }

  [data-user="Carter"] .user-icon-container {
    left: 0;
    transform: translate(40px, 15%);
  }

  [data-user="You"] .user-icon-container {
    right: 0;
    transform: translate(-40px, 15%);
  }
</style>
