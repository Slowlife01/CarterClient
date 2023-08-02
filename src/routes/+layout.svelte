<script>
  import { page } from "$app/stores";
  import Sidebar from "../lib/Sidebar.svelte";

  import "@fontsource-variable/outfit";
  import "../app.scss";

  $: hasParams = $page.route.id !== "/";
</script>

<main class="container dark flex flex-row w-screen h-screen bg-[#0a0909]">
  {#if hasParams}
    <Sidebar />
  {/if}
  <slot />
</main>

<svelte:window on:contextmenu|preventDefault />
