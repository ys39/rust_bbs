<script>
  import { createEventDispatcher, getContext } from "svelte";
  const dispatch = createEventDispatcher();
  const stateContext = getContext("state");
  
  // 最大で5ページまで表示する
  let buttons = [-2, -1, 0, 1, 2];
  export let page = 0;
  export let pageSize;
  export let total;
  // ページ数
  $: pageCount = Math.floor(total / pageSize);

  // ページネーションのイベント
  function onChange(event, page) {
    const state = stateContext.getState();
    const detail = {
      originalEvent: event,
      page,
      offset: page * state.pageSize,
      pageSize: state.pageSize
    };
    if (detail.preventDefault !== true) {
      stateContext.setPage(detail.page, detail.offset);
    }
    dispatch("pageChange", detail);
  }
</script>

<div class="container mx-auto py-12">
  <div class="flex items-center justify-center">
    <nav aria-label="Page navigation" class="text-center">
    <ul class="flex items-center justify-between space-x-4">
      {#each buttons as button}
        {#if page + button >= 0 && page + button <= pageCount }
          <li>
          <button
            class:active={page === page + button}
            on:click={e => onChange(e, page + button)}
            class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-blue-300 focus:outline-none"
            >
            {page + button + 1}
          </button>
          </li>
        {/if}
      {/each}
    </ul>
    </nav>
  </div>
</div>

<style>
  .active{
    background-color: #3182ce;
    color: white;
  }
</style>