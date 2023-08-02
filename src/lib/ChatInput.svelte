<script lang="ts">
  import { browser } from "$app/environment";

  import type { Writable } from "svelte/store";
  import { writable } from "svelte/store";

  import Icon from "svelte-awesome";

  import { RingLoader } from "svelte-loading-spinners";
  import { faPaperPlane } from "@fortawesome/free-solid-svg-icons/faPaperPlane";

  import FuzzySearch from "fuzzy-search";
  import { afterUpdate, onDestroy } from "svelte";
  import { afterNavigate, beforeNavigate } from "$app/navigation";

  export let onSend: (message: string) => void;
  export let isTypingMap: Writable<Map<string, boolean>>;
  export let message: Writable<string>;
  export let id: string;

  $: isTyping = $isTypingMap.get(id);

  const commands: Command[] = [
    {
      name: "/help",
      description: "Display a list of the available commands",
    },
    {
      name: "/plugin dev start",
      description: "Become a plugin developer and view your developer ID",
    },
    {
      name: "/plugin dev submit ",
      description: "Submit a plugin or update an existing plugin",
      options: [
        {
          name: "url",
          type: "string",
          description: "The url to carterplugin.json of your plugin",
        },
      ],
    },
    {
      name: "/plugin dev publish ",
      description: "Make your plugin available to the public",
    },
    {
      name: "/plugin dev list_submitted",
      description: "List all plugins you have submitted",
    },
    {
      name: "/plugin dev destroy ",
      description: "Delete a plugin you have submitted",
    },
    {
      name: "/plugin dev dev_token ",
      description: "Display the secret token for the specified plugin",
      options: [
        {
          name: "code",
          type: "string",
          description: "The plugin code",
        },
      ],
    },
    {
      name: "/plugin dev dev_token_reset ",
      description: "Reset the secret token for the specified plugin",
      options: [
        {
          name: "code",
          type: "string",
          description: "The plugin code",
        },
      ],
    },
    {
      name: "/plugin discover",
      description: "Display a link to the plugin discovery page",
    },
    {
      name: "/plugin info ",
      description: "Display information about a plugin",
      options: [
        {
          name: "code",
          type: "string",
          description: "The plugin code",
        },
      ],
    },
    {
      name: "/plugin install ",
      description: "Install a plugin to this relationship",
      options: [
        {
          name: "code",
          type: "string",
          description: "The plugin code",
        },
      ],
    },
    {
      name: "/plugin list",
      description: "List all plugins installed to this relationship",
    },
    {
      name: "/plugin uninstall ",
      description: "Uninstall a plugin from this relationship",
      options: [
        {
          name: "code",
          type: "string",
          description: "The plugin code",
        },
      ],
    },
    {
      name: "/plugin token",
      description: "Display the plugin token for this relationship",
    },
    {
      name: "/plugin token_reset",
      description: "Reset the plugin token for this relationship",
    },
    {
      name: "/report ",
      description: "Report something!",
      options: [
        {
          name: "comment",
          type: "string",
          description: "The comment to send to the developers",
          optional: true,
        },
      ],
    },
    {
      name: "/web",
      description: "Talk on the web",
    },
  ];

  interface Command {
    name: string;
    description: string;
    options?: {
      name: string;
      type: string;
      description: string;
      optional?: boolean;
    }[];
  }

  let textarea: HTMLTextAreaElement;

  let currentInput = writable("");
  let commandIndex = writable(0);

  let selectedCommand = writable(false);
  let displayedCommands: Writable<(Command & { index: number })[]> = writable(
    []
  );

  const unsubInput = currentInput.subscribe((value: string) => {
    if ($selectedCommand) return;

    const searcher = new FuzzySearch(commands, ["name"], {
      caseSensitive: false,
    });

    const results: {
      name: string;
      description: string;
    }[] = searcher.search($currentInput);

    $commandIndex = 0;
    $displayedCommands = results.map((result, index) => ({
      ...result,
      index,
    }));
  });

  const unsubCommand = commandIndex.subscribe((value: number) => {
    if ($commandIndex > $displayedCommands.length - 1) {
      $commandIndex = 0;
    }

    if ($commandIndex < 0) {
      $commandIndex = $displayedCommands.length - 1;
    }

    if (browser) {
      const command = document.querySelector<HTMLDivElement>(
        `#command[data-index="${$commandIndex}"]`
      );

      if (command) {
        const autocomplete = document.querySelector(".autocomplete");
        const commandTop = command.offsetTop;
        const commandBottom = commandTop + command.clientHeight;
        const autocompleteTop = autocomplete.scrollTop;
        const autocompleteBottom = autocompleteTop + autocomplete.clientHeight;

        if (commandTop < autocompleteTop) {
          autocomplete.scrollTo({
            top: commandTop - 10,
            behavior: "smooth",
          });
        } else if (commandBottom > autocompleteBottom) {
          autocomplete.scrollTo({
            top: commandBottom - autocomplete.clientHeight + 10,
            behavior: "smooth",
          });
        }
      }
    }
  });

  $: $currentInput = $message;
  $: currentCommand = $displayedCommands[$commandIndex];

  onDestroy(() => {
    unsubInput();
    unsubCommand();
  });

  beforeNavigate(() => {
    if (id) localStorage.setItem(`input:${id}`, $message);
    $message = "";
  });

  afterNavigate(() => {
    if (id) {
      const storedInput = localStorage.getItem(`input:${id}`);

      if (storedInput) {
        $message = storedInput;
      }
    }
  });

  afterUpdate(() => {
    if (textarea) {
      textarea.style.maxHeight = "200px";
      textarea.style.height = "auto";
      textarea.style.height = textarea.scrollHeight - 20 + "px";
    }
  });

  function handleInput(
    event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
  ) {
    $selectedCommand = false;
    $currentInput = event.currentTarget.value;
  }
</script>

{#if currentCommand && $selectedCommand}
  <div
    class="relative p-2 bg-[#252527]/50 rounded-t-lg border-[#3e3f40] border-x border-t mx-4 flex flex-row items-center"
  >
    <div class="text-white mr-3">{currentCommand.name}</div>
    <div class="text-sm text-gray-400 mr-3">{currentCommand.description}</div>
  </div>
{/if}
<div
  class="relative p-2 {$selectedCommand
    ? 'rounded-b-lg'
    : 'rounded-lg'} border-[#3e3f40] border mx-4 mb-4 flex items-center"
>
  {#if $displayedCommands.length && $currentInput.startsWith("/") && !$selectedCommand}
    <div
      class="autocomplete bg-[#121112] border-2 border-[#1d1e20] rounded-lg overflow-y-auto"
    >
      <div class="mx-4 my-2 flex flex-col">
        {#each $displayedCommands as command}
          <div
            role="option"
            tabindex={command.index === $commandIndex ? 0 : -1}
            aria-selected={command.index === $commandIndex}
            data-name={command.name}
            data-index={command.index}
            id="command"
            class="{command.index === $commandIndex
              ? 'bg-[#1d1e20]'
              : ''} hover:bg-[#1d1e20] my-1 rounded-md p-1 cursor-pointer select-none"
            on:click|preventDefault={() => {
              $selectedCommand = true;
              $message = command.name;
            }}
            on:keydown={(event) => {
              if (event.key === "Enter") {
                $selectedCommand = true;
                $message = command.name;
              }
            }}
          >
            <div class="flex space-x-3">
              <div class="text-white">{command.name}</div>
              {#if command.options}
                {#each command.options as option}
                  <div class="text-white bg-black rounded-md px-1">
                    {option.name}
                  </div>
                {/each}
              {/if}
            </div>
            <div class="text-gray-500">{command.description}</div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
  <textarea
    class="text-white focus:ring-0 rounded-lg resize-none border-0 flex-auto bg-transparent"
    bind:this={textarea}
    bind:value={$message}
    placeholder="Type '/' for commands"
    on:input={handleInput}
    on:keydown={(e) => {
      if (!e.shiftKey) {
        if (e.key === "Enter" && !isTyping && $message.trim().length > 0) {
          if ($selectedCommand) {
            $selectedCommand = false;
            onSend($message);
          } else if ($message.startsWith("/") && currentCommand) {
            e.preventDefault();

            $message = currentCommand.name;
            $selectedCommand = true;
          } else {
            e.preventDefault();

            onSend($message);
          }
        }
      }

      if ($message.startsWith("/")) {
        switch (e.key) {
          case "Tab":
            e.preventDefault();
            if (currentCommand) {
              $selectedCommand = true;
              $message = currentCommand.name;
            }
            break;
          case "ArrowDown":
            e.preventDefault();
            $commandIndex++;
            break;
          case "ArrowUp":
            e.preventDefault();
            $commandIndex--;
            break;
        }
      }
    }}
  />
  {#if !isTyping && $message.trim().length > 0}
    <button
      class="absolute bottom-0 right-0 -translate-y-3 -translate-x-4 bg-[#1C30BD] text-white py-2 px-3 font-bold rounded-lg"
      on:click={() => onSend($message)}
    >
      <Icon data={faPaperPlane} />
    </button>
  {:else}
    <button
      class="absolute bottom-0 right-0 -translate-y-3 -translate-x-4 text-[#3e3f40] py-2 px-3 pointer-events-none"
    >
      {#if isTyping}
        <RingLoader size="20" color="white" />
      {:else}
        <Icon data={faPaperPlane} />
      {/if}
    </button>
  {/if}
</div>

<style>
  .autocomplete::-webkit-scrollbar {
    width: 6px;
  }

  .autocomplete::-webkit-scrollbar-track {
    background: #3e3f40;
    border-radius: 10px;
  }

  .autocomplete::-webkit-scrollbar-thumb {
    background: #888;
    border-radius: 3px;
  }

  .autocomplete::-webkit-scrollbar-thumb:hover {
    background: #555;
  }

  @media (max-height: 600px) {
    .autocomplete {
      max-height: 12rem !important;
    }
  }

  .autocomplete {
    position: absolute;
    width: 100%;
    max-height: 20rem;
    bottom: calc(100% + 0.6rem);
    left: 50%;
    transform: translateX(-50%);
  }

  textarea::-webkit-scrollbar {
    width: 5px;
  }

  textarea::-webkit-scrollbar-track {
    background: transparent;
  }

  textarea::-webkit-scrollbar-thumb {
    background: transparent;
    border-radius: 3px;
  }

  textarea:hover::-webkit-scrollbar-thumb {
    @apply bg-[#b6b6b6];
  }
</style>
