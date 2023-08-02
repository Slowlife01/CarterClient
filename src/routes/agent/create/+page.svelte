<script lang="ts">
  import { goto, invalidate } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";

  import { RingLoader } from "svelte-loading-spinners";

  let isLoading = false;

  let keyInput: HTMLInputElement;
  let nameInput: HTMLInputElement;

  let failed = false;

  function errorBorder(element: HTMLInputElement) {
    if (element.value) return;

    element.classList.add("border-red-500");

    setTimeout(() => {
      element.classList.remove("border-red-500");
    }, 1000);
  }

  async function handleSubmit() {
    if (!keyInput?.value || !nameInput?.value) {
      errorBorder(keyInput);
      errorBorder(nameInput);

      return;
    }

    isLoading = true;
    try {
    const agent = await invoke<{
      id: string;
      name: string;
    }>("create_agent", {
      key: keyInput.value,
      name: nameInput.value,
    });

    await invalidate("data");
    await goto(`/agent/${agent.id}`);
} catch {
    isLoading = false;
    failed = true;

    setInterval(() => failed = false, 3000)
}
  }
</script>

<div class="flex flex-col w-full">
  <div class="pb-2 mb-2">
    <h1 class="text-white mt-4 ml-6">Add a new agent</h1>
  </div>
  <div class="ml-6 w-[50%] lg:w-[50%] md:w-[65%]">
    <div class="space-y-5">
      <div>
        <label
          class="text-sm font-medium text-gray-700 dark:text-white"
          for="api-key">API key</label
        >
        <p class="mt-1 text-xs text-gray-500 dark:text-gray-300">
          You will find this in the <a
            target="_blank"
            href="https://controller.carterlabs.ai">Controller</a
          >
        </p>
        <div class="mt-2">
          <input
            bind:this={keyInput}
            name="key"
            autocorrect="off"
            autocomplete="off"
            class="block w-full rounded-md px-3 py-2 text-white text-sm outline-none border-[#3e3f40] focus:border-[#1C30BD] bg-[#161515]"
            placeholder="Secret API key"
            required={true}
            autofocus={true}
            type="text"
          />
        </div>
      </div>
      <div>
        <label
          class="text-sm font-medium text-gray-700 dark:text-white"
          for="agent-name">Agent name</label
        >
        <div class="relative mt-2">
          <input
            bind:this={nameInput}
            autocomplete="off"
            name="name"
            class="block w-full rounded-md px-3 py-2 text-white text-sm outline-none border-[#3e3f40] focus:border-[#1C30BD] bg-[#161515]"
            placeholder="Javis"
            required={true}
            type="text"
          />
        </div>
      </div>
    </div>
    {#if failed}
    <div id="error" class="mt-3">
      <p class="text-red-500/80 text-xs">
        An agent with the same key already exists
      </p>
    </div>
    {/if}
    <div class="mt-3">
      <button
        class="inline-flex border border-transparent w-full min-w-28 h-11 items-center justify-center
        overflow-hidden rounded-md bg-darkblue px-4 py-2 leading-6 text-white transition
        duration-300 ease-in-out focus:border-white"
        on:click={handleSubmit}
      >
        {#if isLoading}
          <RingLoader size="25" color="white" />
        {:else}
          <div
            class="flex items-center space-x-2 transition duration-200 ease-in-out"
          >
            Add agent
          </div>
        {/if}
      </button>
    </div>
  </div>
</div>
