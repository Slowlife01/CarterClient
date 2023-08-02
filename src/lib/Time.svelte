<script lang="ts">
  import { DateTime } from "luxon";
  import { writable } from "svelte/store";

  export let timestamp: string | number = 0;
  export let className = "";

  let formatted = writable("");
  $: formatDate(null, timestamp);

  const interval = setInterval(() => formatDate(interval), 1000);
  function formatDate(interval?: NodeJS.Timer, time = timestamp) {
    if (time === 0) {
      if (interval) clearInterval(interval);
      return;
    }

    const now = DateTime.now();
    const then = DateTime.fromJSDate(new Date(time));

    const diff = now.diff(then, ["days", "hours", "minutes", "seconds"]);

    if (diff.days > 0) formatted.set(then.toFormat("dd LLL"));
    else if (diff.hours > 0) formatted.set(then.toFormat("h:mm a"));
    else if (diff.minutes > 0) formatted.set(`${diff.minutes} m`);
    else if (diff.seconds > 0) formatted.set("just now");
  }
</script>

<span class="text-gray-400 text-xs {className}">{$formatted}</span>
