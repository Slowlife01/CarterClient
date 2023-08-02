<script>
  import { goto } from "$app/navigation";
  import ChatQuestion from "$lib/assets/ChatQuestion.svelte";
  export let data;

  $: {
    const agents = data.chats;
    if (agents?.[0]) {
      goto(`/chat/${agents[0].id}`);
    }
  }
</script>

<div class="flex flex-col w-full items-center justify-center">
  {#if data && !data.chats.length}
    <ChatQuestion className="fill-white" height="10em" width="10em" />
{#if data.agents.length}
    <h1 class="text-white">There are no chats</h1>
{:else}
    <h1 class="text-white">No available agent</h1>
{/if}
    {#if data.agents.length}
      <p class="text-gray-400 text-sm select-none pointer-events-none">
        Why not <a class="pointer-events-auto" href="/chat/create">create one</a
        > and start a conversation?
      </p>
    {:else}
      <p class="text-gray-400 text-sm select-none pointer-events-none">
        Let's start by <a class="pointer-events-auto" href="/agent/create"
          >adding</a
        > your first agent
      </p>{/if}
  {/if}
</div>
