<script>
  export let open;
</script>

{#if $open}
  <div
    tabindex="-1"
    class="fixed flex justify-center items-center border-2 border-[#3e3f40] rounded-lg top-0 left-0 right-0 translate-x-[80%] translate-y-[75%] h-[40%] w-[40%] py-2 bg-[#0a0909]"
    role="dialog"
  >
    <nav class="fixed right-0 top-0 z-[100] px-4 py-4">
      <button
        on:click={() => open.set(false)}
        aria-label="Close"
        class="flex cursor-pointer items-center space-x-2 text-sm outline-none text-white/90"
        tabindex="-1"
        ><span>Close</span><span
          class="rounded-md bg-gray-800 px-1.5 py-0.5 text-xs">Esc</span
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="currentColor"
          aria-hidden="true"
          height="15"
          width="15"
          ><path
            fill-rule="evenodd"
            d="M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"
            clip-rule="evenodd"
          /></svg
        ></button
      >
    </nav>
    <slot />
  </div>
{/if}

<svelte:window
  on:keydown={(e) => {
    if (e.key === "Escape" && $open) {
      e.preventDefault();
      open.set(false);
    }
  }}
/>
