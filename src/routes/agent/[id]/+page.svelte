<script lang="ts">
  import { goto, invalidateAll } from "$app/navigation";
  import { page } from "$app/stores";

  import { writable } from "svelte/store";
  import { invoke } from "@tauri-apps/api/tauri";

  import { Tooltip } from "flowbite-svelte";
  import Dialog from "$lib/Dialog.svelte";
  import { fly } from "svelte/transition";
  import { Circle, Moon, Plane, SyncLoader } from "svelte-loading-spinners";

  const plugins = writable([]);

  let openDeleteModal = writable(false);

  let keyInputElement: any = { type: "password" };
  let nameInputElement;

  let isUnsaved = false;
  let isSaving = false;

  export let data: {
    agents: Agent[];
  };

  $: agent = data.agents.find((agent) => agent.id === $page.params.id);

  interface Agent {
    id: string;
    name: string;
    key: string;
  }

  async function handleDelete() {
    try {
      await invoke("remove_agent", {
        id: agent.id,
      });

      await invalidateAll();
      await goto("/agent/");
    } catch (error) {
      console.error(error);
    }
  }

  function handleInput(
    event: Event & { currentTarget: EventTarget & HTMLInputElement }
  ) {
    isUnsaved =
      nameInputElement.value !== agent.name ||
      keyInputElement.value !== agent.key;
  }

  function handleReset() {
    nameInputElement.value = agent.name;
    keyInputElement.value = agent.key;

    isUnsaved = false;
  }

  async function handleSave() {
    isSaving = true;

    await invoke("update_agent", {
      id: agent.id,
      name: nameInputElement.value,
      key: keyInputElement.value,
    });

    await invalidateAll();
    await goto(`/agent/${agent.id}`);

    isSaving = false;
    isUnsaved = false;
  }
</script>

{#if agent}
  {#if isUnsaved}
    <div
      transition:fly={{ y: 50, duration: 200 }}
      class="fixed flex justify-between items-center rounded-lg bottom-0 left-0 right-0 translate-x-[50%] -translate-y-12 w-[50%] py-2 bg-black/40"
    >
      <div class="text-white ml-3">You have unsaved changes!</div>
      <div class="text-white flex space-x-4 mr-3">
        <button on:click={handleReset} class="hover:underline">Reset</button>
        {#if !isSaving}
          <button
            on:click={handleSave}
            class="bg-green-700/90 hover:bg-green-700/70 p-1 px-2 rounded-md"
            >Save Changes</button
          >
        {:else}
          <button disabled={true} class="bg-green-700/90 p-1 px-2 rounded-md"
            ><div class="flex items-center space-x-2">
              <Circle size="20" color="white" />
              <div>Saving</div>
            </div></button
          >
        {/if}
      </div>
    </div>
  {/if}

  <div class="mt-4 w-full flex">
    <div class="ml-6">
      <h3 class="text-white text-xl mb-5">Edit your agent</h3>
      <div class="flex flex-col space-y-6">
        <div>
          <div class="text-lg text-white">General</div>
          <div class="grid grid-cols-3 mt-3 border-t border-t-[#3e3f40] py-2">
            <div class="text-sm font-medium text-gray-300">Name</div>
            <input
              type="text"
              autocomplete="off"
              class="w-[calc(100%+150px)] rounded-md px-3 py-2 text-white text-sm outline-none border-[#3e3f40] focus:border-[#1C30BD] bg-[#161515]"
              value={agent.name}
              on:input={handleInput}
              bind:this={nameInputElement}
            />
          </div>
          <div class="grid grid-cols-3 mt-3 border-t border-t-[#3e3f40] py-2">
            <div class="text-sm font-medium text-gray-300">API Key</div>
            <div class="relative mt-2">
              <input
                autocomplete="off"
                autocorrect="off"
                id="chat-id"
                class="w-[calc(100%+150px)] rounded-md px-3 py-2 text-white text-sm outline-none border-[#3e3f40] focus:border-[#1C30BD] bg-[#161515]"
                type="password"
                value={agent.key}
                on:input={handleInput}
                bind:this={keyInputElement}
              />
              <Tooltip triggeredBy="#show-key"
                >{keyInputElement.type === "password" ? "Show" : "Hide"} API Key</Tooltip
              >
              <button
                id="show-key"
                on:click={() => {
                  keyInputElement.type =
                    keyInputElement.type === "password" ? "text" : "password";
                }}
                class="absolute inset-y-0 -right-2 translate-x-32 flex w-fit h-fit items-center my-2 text-bluegray-400"
              >
                {#if keyInputElement.type === "password"}
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="fill-white"
                    height="1.2em"
                    viewBox="0 0 576 512"
                    ><path
                      d="M288 32c-80.8 0-145.5 36.8-192.6 80.6C48.6 156 17.3 208 2.5 243.7c-3.3 7.9-3.3 16.7 0 24.6C17.3 304 48.6 356 95.4 399.4C142.5 443.2 207.2 480 288 480s145.5-36.8 192.6-80.6c46.8-43.5 78.1-95.4 93-131.1c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C433.5 68.8 368.8 32 288 32zM144 256a144 144 0 1 1 288 0 144 144 0 1 1 -288 0zm144-64c0 35.3-28.7 64-64 64c-7.1 0-13.9-1.2-20.3-3.3c-5.5-1.8-11.9 1.6-11.7 7.4c.3 6.9 1.3 13.8 3.2 20.7c13.7 51.2 66.4 81.6 117.6 67.9s81.6-66.4 67.9-117.6c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3z"
                    /></svg
                  >
                {:else}
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="fill-white"
                    height="1.1em"
                    viewBox="0 0 640 512"
                    ><path
                      d="M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7L525.6 386.7c39.6-40.6 66.4-86.1 79.9-118.4c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C465.5 68.8 400.8 32 320 32c-68.2 0-125 26.3-169.3 60.8L38.8 5.1zm151 118.3C226 97.7 269.5 80 320 80c65.2 0 118.8 29.6 159.9 67.7C518.4 183.5 545 226 558.6 256c-12.6 28-36.6 66.8-70.9 100.9l-53.8-42.2c9.1-17.6 14.2-37.5 14.2-58.7c0-70.7-57.3-128-128-128c-32.2 0-61.7 11.9-84.2 31.5l-46.1-36.1zM394.9 284.2l-81.5-63.9c4.2-8.5 6.6-18.2 6.6-28.3c0-5.5-.7-10.9-2-16c.7 0 1.3 0 2 0c44.2 0 80 35.8 80 80c0 9.9-1.8 19.4-5.1 28.2zm9.4 130.3C378.8 425.4 350.7 432 320 432c-65.2 0-118.8-29.6-159.9-67.7C121.6 328.5 95 286 81.4 256c8.3-18.4 21.5-41.5 39.4-64.8L83.1 161.5C60.3 191.2 44 220.8 34.5 243.7c-3.3 7.9-3.3 16.7 0 24.6c14.9 35.7 46.2 87.7 93 131.1C174.5 443.2 239.2 480 320 480c47.8 0 89.9-12.9 126.2-32.5l-41.9-33zM192 256c0 70.7 57.3 128 128 128c13.3 0 26.1-2 38.2-5.8L302 334c-23.5-5.4-43.1-21.2-53.7-42.3l-56.1-44.2c-.2 2.8-.3 5.6-.3 8.5z"
                    /></svg
                  >
                {/if}
              </button>
            </div>
          </div>
        </div>
        <div>
          <div class="text-lg text-white">Plugins</div>
          {#if $plugins.length === 0}
            <div class="text-gray-500 text-sm mt-2">No plugins installed</div>
          {:else}
            {#each $plugins as plugin}
              <div
                class="flex flex-row justify-between items-center mt-2 w-[60%] p-1 border border-[#3e3f40] rounded-md"
              >
                <div>
                  <div class="text-white ml-1">{plugin.name}</div>
                  <div class="flex flex-row ml-1 items-center">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      height="0.9em"
                      class="fill-gray-400 mr-1"
                      viewBox="0 0 512 512"
                      ><path
                        d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM216 336h24V272H216c-13.3 0-24-10.7-24-24s10.7-24 24-24h48c13.3 0 24 10.7 24 24v88h8c13.3 0 24 10.7 24 24s-10.7 24-24 24H216c-13.3 0-24-10.7-24-24s10.7-24 24-24zm40-208a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"
                      /></svg
                    >
                    <div class="text-sm text-gray-400">
                      {plugin.description}
                    </div>
                  </div>
                </div>
                <button
                  class="plugin-remove bg-red-500 border-2 hover:border-white border-transparent p-2 rounded-lg ml-2 mr-2 flex flex-row items-center"
                >
                  <svg
                    class="fill-white"
                    xmlns="http://www.w3.org/2000/svg"
                    height="1.2em"
                    viewBox="0 0 448 512"
                    ><path
                      d="M170.5 51.6L151.5 80h145l-19-28.4c-1.5-2.2-4-3.6-6.7-3.6H177.1c-2.7 0-5.2 1.3-6.7 3.6zm147-26.6L354.2 80H368h48 8c13.3 0 24 10.7 24 24s-10.7 24-24 24h-8V432c0 44.2-35.8 80-80 80H112c-44.2 0-80-35.8-80-80V128H24c-13.3 0-24-10.7-24-24S10.7 80 24 80h8H80 93.8l36.7-55.1C140.9 9.4 158.4 0 177.1 0h93.7c18.7 0 36.2 9.4 46.6 24.9zM80 128V432c0 17.7 14.3 32 32 32H336c17.7 0 32-14.3 32-32V128H80zm80 64V400c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16zm80 0V400c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16zm80 0V400c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16z"
                    /></svg
                  ></button
                >
              </div>
            {/each}
          {/if}
        </div>
        <div>
          <div class="text-lg text-white">Danger Zone</div>
          <button
            on:click={() => ($openDeleteModal = true)}
            class="outline-0 text-red-500 text-sm mt-2 cursor-pointer"
          >
            Remove this agent
          </button>
        </div>
      </div>
    </div>
  </div>

  <Dialog open={openDeleteModal}>
    <div class="p-4 dark:text-gray-900" role="dialog">
      <div class="max-w-sm space-y-10">
        <div class="space-y-5 text-center">
          <h3
            class="lg:text-3xl md:text-xl sm:text-sm font-medium leading-6 text-gray-800 dark:text-white/90"
          >
            Confirm agent removal
          </h3>
          <p
            class="lg:text-base md:text-sm sm:text-xs text-gray-500 dark:text-gray-300"
          >
            This will remove the agent {agent.name} and all its chats.
            <br /><strong>This action is permanent.</strong>
          </p>
        </div>
        <div class="flex justify-center mt-7">
          <span class="flex w-full rounded-md shadow-sm"
            ><button
              on:click={handleDelete}
              id="remove-agent"
              type="submit"
              class="w-full cursor-pointer min-w-28 justify-center rounded-md px-4 py-2 text-base font-medium leading-6 text-white focus:outline-none transition duration-300 ease-in-out cursor-auto bg-red-500 hover:bg-red-600"
              ><span>Remove agent</span></button
            ></span
          >
        </div>
      </div>
    </div>
  </Dialog>
{/if}
