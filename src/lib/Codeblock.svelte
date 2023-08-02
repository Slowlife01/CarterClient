<script>
  import hljs from "highlight.js";
  import "highlight.js/styles/github-dark-dimmed.css";

  export let lang;
  export let text;

  $: code = hljs.highlight(
    hljs.getLanguage(lang) ? lang : "plaintext",
    text
  ).value;

  let copied = false;
</script>

<pre class="group">
  <button
    data-copied={copied}
    on:click={() => {
      navigator.clipboard.writeText(text);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 1000);
    }}
    class="group-hover:visible invisible absolute right-0 w-fit py-1 translate-y-16 -translate-x-16 border-2 border-[#3e3f40] rounded-lg px-2"
    >{#if !copied}<svg
        xmlns="http://www.w3.org/2000/svg"
        class="fill-white"
        height="1.5em"
        width="1.1em"
        viewBox="0 0 512 512"
        ><path
          d="M104.6 48H64C28.7 48 0 76.7 0 112V384c0 35.3 28.7 64 64 64h96V400H64c-8.8 0-16-7.2-16-16V112c0-8.8 7.2-16 16-16H80c0 17.7 14.3 32 32 32h72.4C202 108.4 227.6 96 256 96h62c-7.1-27.6-32.2-48-62-48H215.4C211.6 20.9 188.2 0 160 0s-51.6 20.9-55.4 48zM144 56a16 16 0 1 1 32 0 16 16 0 1 1 -32 0zM448 464H256c-8.8 0-16-7.2-16-16V192c0-8.8 7.2-16 16-16l140.1 0L464 243.9V448c0 8.8-7.2 16-16 16zM256 512H448c35.3 0 64-28.7 64-64V243.9c0-12.7-5.1-24.9-14.1-33.9l-67.9-67.9c-9-9-21.2-14.1-33.9-14.1H256c-35.3 0-64 28.7-64 64V448c0 35.3 28.7 64 64 64z"
        /></svg
      >{:else}<svg
        xmlns="http://www.w3.org/2000/svg"
        class="fill-white"
        height="1.5em"
        width="1.1em"
        viewBox="0 0 448 512"
        ><path
          d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"
        /></svg
      >{/if}</button
  >
  <code class="hljs language-{lang}">{@html code}</code>
</pre>
