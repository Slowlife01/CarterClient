<script lang="ts">
  import { page } from "$app/stores";

  import Subsidebar from "$lib/Subsidebar.svelte";

  import AgentsIcon from "$lib/assets/AgentsIcon.svelte";
  import ADescription from "$lib/agent/Description.svelte";

  import { writable } from "svelte/store";

  export let data: {
    agents?: {
      id: string;
      name: string;
    }[];
  };

  let dataList = writable([]);
  $: dataList.set(
    data.agents.map((agent) => {
      return {
        title: agent.name,
        description: ADescription,
        dProps: { agent },
        selected: agent.id === $page.params.id,
        href: `/agent/${agent.id}`,
      };
    })
  );
</script>

<div
  class="agents-container my-5 rounded-l-xl flex flex-1 flex-row bg-[#121112]"
>
  <Subsidebar
    title="Agents"
    icon={AgentsIcon}
    pathToCreate={"/agent/create"}
    defaultDescription={"No description"}
    {dataList}
  />
  <slot />
</div>
