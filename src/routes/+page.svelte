<script lang="ts">
  import { goto, invalidate } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  async function checkState(interval?: NodeJS.Timeout) {
    const isReady = await invoke("is_db_ready");

    if (isReady) {
      await invalidate("data");
      if (interval) clearInterval(interval);
      await goto("/chat");
    }

    return isReady;
  }

  onMount(async () => {
    if (await checkState()) return;
    const interval = setInterval(() => checkState(interval), 300);
  });
</script>

<div class="flex flex-col w-full justify-center items-center">
  <div class="sphere">
    <div class="svg-container">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="100%"
        height="100%"
        viewBox="0 0 16 16"
      >
        <circle cx="8" cy="8" r="8" fill="rgb(161,79,196)" />
        <circle cx="8" cy="8" r="8" fill="url(#gradient)" />
        <defs>
          <radialGradient
            id="gradient"
            cx="50%"
            cy="50%"
            r="50%"
            fx="50%"
            fy="50%"
          >
            <stop
              offset="0%"
              style="stop-color:rgb(161,79,196); stop-opacity:1"
            />
            <stop
              offset="100%"
              style="stop-color:rgb(57,106,252); stop-opacity:1"
            />
          </radialGradient>
        </defs>
        <path
          fill="rgb(255,255,255)"
          fill-rule="evenodd"
          d="M6.05947432 8c0 3.30434783 6.91368603.60126735 7.7806591 2.08695652C14.85500202 11.82608696 10.71101469 14 8.08921149 14 4.72848418 14 2 11.31149015 2 8s2.72848418-6 6.08921148-6c2.72271796 0 6.76579053 2.4347826 5.75092195 4.17391304C13.01454878 7.5886769 6.05947432 4.69565217 6.05947432 8z"
        />
      </svg>
    </div>
  </div>
</div>

<style>
  .sphere {
    width: 250px; /* Adjust the size as needed */
    height: 250px;
    border-radius: 50%;
    background: radial-gradient(
      circle,
      rgba(161, 79, 196, 0.8) 0%,
      rgba(57, 106, 252, 0.8) 100%
    );
    box-shadow: 0 5px 20px rgba(0, 0, 0, 0.3); /* Adding a shadow for a 3D effect */
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden; /* Hide the shiny part overflowing from the circle */
    animation: growSphere 2s infinite alternate; /* Adding a grow animation */
  }

  .svg-container {
    width: 80%; /* Adjust the size of the SVG */
    height: 80%; /* Adjust the size of the SVG */
  }

  @keyframes growSphere {
    0% {
      transform: scale(1);
    }
    100% {
      transform: scale(1.2);
    } /* Adjust the scale value for the desired grow effect */
  }
</style>
