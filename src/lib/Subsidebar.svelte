<script lang="ts">
  import { writable } from "svelte/store";
  import Time from "./Time.svelte";

  export let title: string;
  export let icon: any;

  export let dataList = writable<Data[]>([]);
  export let defaultDescription: string;

  export let pathToCreate: string;
  export let canCreate = true;

  interface Data {
    icon?: string;
    title: string;
    description?: any;
    dProps?: any;
    selected?: boolean;
    href?: string;
    date?: string;
  }
</script>

<div
  class="sidebar select-none rounded-l-lg bg-[#121112] xl:w-[25%] lg:w-[30%] md:w-[40%] flex flex-col px-4 pt-5 border-r border-[#3e3f40]"
>
  <div class="flex flex-row items-center">
    <h1 class="text-white mr-4">{title}</h1>
    <div
      class="h-7 w-7 rounded-full bg-[#1C30BD] flex items-center justify-center"
    >
      <div class="chat-count font-bold text-slate-200 text-sm">
        {$dataList.length}
      </div>
    </div>
  </div>
  <div class="flex flex-row my-4 justify-center">
    <div
      class="relative flex justify-center items-center h-[2.4rem] w-full mx-1 border border-[#3e3f40] rounded-lg"
    >
      <input
        type="text"
        required={true}
        class="text-white w-full focus:ring-0 rounded-lg border-0 bg-transparent"
        placeholder="Search {title.toLowerCase()}"
      />
      <svg
        class="fill-white mr-3"
        xmlns="http://www.w3.org/2000/svg"
        height="1em"
        viewBox="0 0 512 512"
        ><path
          d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"
        /></svg
      >
    </div>
    <a
      href={canCreate ? pathToCreate : pathToCreate.replace("/create", "")}
      class="border border-[#3e3f40] hover:bg-[#3e3f40] rounded-lg py-[9px] px-[10px] mb-1 mr-1 flex justify-center items-center"
    >
      <svg
        class="fill-white"
        xmlns="http://www.w3.org/2000/svg"
        height="1.1em"
        viewBox="0 0 448 512"
        ><path
          d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
        /></svg
      >
    </a>
  </div>
  <div class="flex flex-col h-full overflow-hidden justify-between">
    <div class="overflow-hidden">
      <div class="flex flex-col h-full w-full">
        <div class="flex flex-row">
          <div class="mt-1 mr-2">
            <svelte:component
              this={icon}
              className="fill-slate-600"
              height="0.8em"
            />
          </div>
          <span class="text-sm text-gray-400 -ml-1">All</span>
        </div>
        <div class="all-data pr-2 overflow-y-auto">
          {#if $dataList.length}
            {#each $dataList as data}
              <a
                href={data.href}
                data-current={data.selected}
                class="flex flex-col my-3 hover:bg-[#252527]/50 border border-transparent p-2 rounded-md"
              >
                <div class="flex justify-between items-center">
                  <h1 class="text-white text-sm">{data.title}</h1>
                  <Time timestamp={data.date} />
                </div>
                <div class="break-words line-clamp-2 text-sm text-gray-500">
                  {#if data.description && typeof data.description === "string"}
                    {data.description}
                  {:else if data.description}
                    <svelte:component
                      this={data.description}
                      {...data.dProps}
                    />
                  {:else}
                    {defaultDescription}
                  {/if}
                </div>
              </a>
            {/each}
          {:else}
            <div class="flex flex-col items-center justify-center h-full pt-3">
              <span class="text-sm text-gray-500"
                >No available {title.toLowerCase()}</span
              >
            </div>
          {/if}
        </div>
      </div>
    </div>
    <slot />
  </div>
</div>

<style>
  .all-data::-webkit-scrollbar {
    width: 4px;
  }

  .all-data::-webkit-scrollbar-track {
    background: transparent;
  }

  .all-data::-webkit-scrollbar-thumb {
    background: transparent;
    border-radius: 3px;
  }

  .all-data:hover::-webkit-scrollbar-thumb {
    @apply bg-[#b6b6b6];
  }

  [data-current="true"] {
    @apply border-[#3e3f40] bg-[#252527]/50;
  }
</style>
