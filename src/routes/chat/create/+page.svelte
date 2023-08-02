<script lang="ts">
  import { Checkbox, Tooltip } from "flowbite-svelte";

  import { goto, invalidate } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { RingLoader } from "svelte-loading-spinners";

  let chatTitle = "";
  let chatId = "";

  let opener = true;

  let isLoading = false;
  let failed = false;

  let chatIdElement: HTMLInputElement;

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
    }[];
  };

  async function handleSubmit() {
    if (!chatIdElement?.value) {
      chatIdElement.classList.add("border-red-500");

      setTimeout(() => {
        chatIdElement.classList.remove("border-red-500");
      }, 1000);

      return;
    }

    try {
    isLoading = true;
    await invoke("create_chat", {
      id: chatId,
      title: chatTitle || "Untitled",
      agentId: data.currentAgent.id,
    });

    if (opener) localStorage.setItem(`chat:${chatId}:opener`, "true");
    isLoading = false;

    await invalidate("data");
    await goto(`/chat/${chatId}`);
   } catch (err) {
   console.log(err)
       isLoading = false;
       failed = true
    
    setInterval(() => failed = false, 3000)
    }
  }
</script>

<div class="flex flex-col w-full select-none">
  {#if data && data.agents.length}
    <div class="pb-2 mb-2">
      <h1 class="text-white mt-4 ml-6">Create a new chat</h1>
    </div>
    <div class="ml-6 w-[50%] lg:w-[50%] md:w-[65%]">
      <div class="space-y-5">
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-white"
            for="chat-id">Chat Identifier</label
          >
          <p class="mt-1 text-xs text-gray-500 dark:text-gray-300">
            This is something your agent will use to recognize this chat
          </p>
          <div class="relative mt-2">
            <input
              on:input={(e) =>
                e.currentTarget.classList.remove("border-red-500")}
              bind:this={chatIdElement}
              bind:value={chatId}
              autocomplete="off"
              autocorrect="off"
              id="chat-id"
              class="w-full rounded-md px-3 py-2 text-white text-sm outline-none border-[#3e3f40] focus:border-[#1C30BD] bg-[#161515]"
              placeholder="a-uniqu3-1d3nt1f13r"
              required={true}
              autofocus={true}
              type="text"
            />
            <Tooltip triggeredBy="#randomize">Randomize</Tooltip>
            <button
              id="randomize"
              on:click|preventDefault={(e) => {
                chatId = window.crypto.randomUUID();
              }}
              class="absolute inset-y-0 right-0 flex w-fit h-fit items-center mr-3 my-2 text-bluegray-400"
            >
              <svg
                class="fill-white"
                xmlns="http://www.w3.org/2000/svg"
                height="1.2em"
                viewBox="0 0 640 512"
                ><path
                  d="M274.9 34.3c-28.1-28.1-73.7-28.1-101.8 0L34.3 173.1c-28.1 28.1-28.1 73.7 0 101.8L173.1 413.7c28.1 28.1 73.7 28.1 101.8 0L413.7 274.9c28.1-28.1 28.1-73.7 0-101.8L274.9 34.3zM200 224a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zM96 200a24 24 0 1 1 0 48 24 24 0 1 1 0-48zM224 376a24 24 0 1 1 0-48 24 24 0 1 1 0 48zM352 200a24 24 0 1 1 0 48 24 24 0 1 1 0-48zM224 120a24 24 0 1 1 0-48 24 24 0 1 1 0 48zm96 328c0 35.3 28.7 64 64 64H576c35.3 0 64-28.7 64-64V256c0-35.3-28.7-64-64-64H461.7c11.6 36 3.1 77-25.4 105.5L320 413.8V448zM480 328a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"
                /></svg
              >
            </button>
          </div>
        </div>
        <div>
          <label
            class="text-sm font-medium text-gray-700 dark:text-white"
            for="chat-title">Chat Title</label
          >
          <span class="ml-1 text-xs text-gray-500">(Optional)</span>
          <div class="mt-2">
            <input
              bind:value={chatTitle}
              id="chat-title"
              autocomplete="off"
              class="w-full rounded-md px-3 py-2 text-white text-sm outline-none border-[#3e3f40] focus:border-[#1C30BD] bg-[#161515]"
              placeholder="How to exit vim"
              type="text"
            />
          </div>
        </div>
        <div class="w-fit ml-1">
          <Checkbox bind:checked={opener} color="blue"
            >Generate conversation opener</Checkbox
          >
        </div>
      </div>
    {#if failed}
    <div id="error" class="mt-3">
      <p class="text-red-500/80 text-xs">
        A chat with the same id already exists
      </p>
    </div>
    {/if}
      <div class="mt-8">
        <button
          on:click={handleSubmit}
          class="inline-flex border border-transparent w-full min-w-28 items-center justify-center
          overflow-hidden rounded-md bg-darkblue px-4 py-2 leading-6 text-white transition
          duration-300 ease-in-out focus:border-white"
          type="submit"
        >
          {#if isLoading}
            <RingLoader size="25" color="white" />
          {:else}
            <div
              class="flex items-center space-x-2 transition duration-200 ease-in-out"
            >
              Create chat
            </div>
          {/if}
        </button>
      </div>
    </div>
  {/if}
</div>
