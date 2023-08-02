<script>
  import Time from "$lib/Time.svelte";

  export let agent;
</script>

<div>
  <span>Last edited: </span>
  <Time timestamp={agent.updatedAt} />
</div>
