<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { afterNavigate } from "$app/navigation";

  import ChatIcon from "$lib/assets/ChatIcon.svelte";
  import AgentsIcon from "$lib/assets/AgentsIcon.svelte";
  import SettingsIcon from "$lib/assets/SettingsIcon.svelte";
  import CarterIcon from "$lib/assets/carter.svg";

  const tabName = writable("");
  onMount(() => updateTabName());
  afterNavigate(() => updateTabName())

  function updateTabName() {
    $tabName = window.location.pathname.split("/")[1];
  }

  const tabs = [
    {
      name: "chat",
      icon: ChatIcon,
    },
    {
      name: "agent",
      icon: AgentsIcon,
    },
    {
      name: "settings",
      icon: SettingsIcon,
    },
  ];
</script>

<div class="sidebar flex flex-col text-white mt-6 mx-3">
  <div class="flex flex-col justify-start h-[20%]">
    <img class="rounded-md w-10" src="{CarterIcon}" alt="The app icon" />
  </div>
  <div class="flex flex-col justify-center">
    {#each tabs as tab}
      <a
        data-tab={tab.name}
        data-selected={$tabName === tab.name}
        class="w-fit p-3 data-[selected=true]:bg-[#1C30BD] data-[selected=true]:rounded-lg mb-4"
        href="/{tab.name}"
      >
        <svelte:component
          this={tab.icon}
          className="{$tabName === tab.name ? 'fill-white' : ''} fill-slate-600"
        />
      </a>
    {/each}
  </div>
</div>
