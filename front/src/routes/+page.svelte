<script>
	//export let data;
  import Paging from '../components/Paging.svelte';
  import { enhance } from '$app/forms';
  import { getposts } from '$lib/api/getposts.js';
  import { onMount, setContext, getContext } from "svelte";

  // 変数
  let page = 0; //first page
  let pageSize = 5; //5 posts per page
  let offset = page * pageSize;
  let data = [];

  // コンポーネント間で値を共有するためのコンテキスト
  setContext("state", {
    getState: () => ({
      page,
      pageSize,
      offset,
    }),
    setPage: (_page, _offset) => {
      page = _page;
      offset = _offset;
    },
  });
  const stateContext = getContext("state");

  // onMountで初期データを取得
	onMount(async () => {
    await load(offset);
	});

  // 初期時にデータを取得
  async function load(_offset) {
    data = await getposts(_offset, pageSize);
  }

  // ページネーションのイベント
  function onPageChange(event) {
    load(event.detail.offset);
  }

  // メッセージ送信時のイベント
  const formCreateNote = ({form, data, action, cancel}) => {
    // メッセージ送信前の処理
    const { content } = Object.fromEntries(data);
    if(content.length === 0) {
      alert('メッセージを入力してください');
      cancel();
    }
    // メッセージ送信後の処理
    return async ({ result, update }) => {
      switch (result.type) {
        case 'success':
          stateContext.setPage(0, 5);
          load(0);
          break;
        default:
          break;    
      }
      await update();
    };
  };

  // メッセージ削除時のイベント
  const formDeleteNote = ({form, data, action, cancel}) => {
    // メッセージ送信前の処理
    const { id } = Object.fromEntries(data);
    if(id.length === 0) {
      alert('IDを入力してください');
      cancel();
    }
    // メッセージ送信後の処理
    return async ({ result, update }) => {
      switch (result.type) {
        case 'success':
          stateContext.setPage(0, 5);
          load(0);
          break;
        default:
          break;    
      }
      await update();
    };
  };
</script>

<main class="container mx-auto my-8 flex-grow">

  <form
    action="?/create"
    method="post"
    class="bg-white p-8 rounded-md shadow-md w-full max-w-md mx-auto"
    id="form1"
    use:enhance={formCreateNote}
  >
    <div class="mb-6">
      <label for="content" class="block text-gray-700 text-sm font-semibold mb-2">メッセージ</label>
      <textarea name="content" rows="4" class="w-full px-3 py-2 text-gray-700 border rounded-md focus:border-blue-500 focus:ring-2 focus:ring-blue-300 focus:outline-none resize-none" placeholder="メッセージを入力"></textarea>
    </div>
    <div class="flex justify-end">
      <button type="submit" class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-6 rounded-full transition duration-200 ease-in focus:ring-4 focus:ring-blue-300 focus:outline-none">送信</button>
    </div>
  </form>
    
    {#if data.all_count}
      <div class="bg-violet-300 text-white p-3 rounded-lg shadow-md mt-4 w-1/4">
        <p>現在の件数: <span class="font-bold">{data.all_count}</span></p>
      </div>
    {:else}
      Loading...
    {/if}
    
    <!-- <子コンポーネント名 on:"受け取るイベント名"={イベントを受け取った時に実行したい関数} > -->
    <!-- on:pageChangeで子コンポーネントのpageChangeイベントを監視している -->
    <!-- pageChangeイベントの発火を検知して、onPageChangeを実行 -->
    <Paging
      {page} {pageSize} total={data.all_count}
      on:pageChange={onPageChange}
    />

    <div class="container mx-auto py-6">
      <ul>
      {#if data.list}
        {#each data.list as post}
          <li class="bg-white p-6 rounded-md shadow-md mb-4">
            <h2 class="text-xl font-semibold mb-4">投稿者: {post.name}</h2>
            <p class="text-gray-700 mb-4">
                {post.content}
            </p>
            <form
              action="?/delete"
              method="post"
              use:enhance={formDeleteNote}
            >
              <input type="hidden" name="id" value={post.id} />
              <button type="submit" class="bg-red-400 hover:bg-red-600 text-white text-xs font-bold py-2 px-6 rounded-full transition duration-200 ease-in focus:ring-4 focus:ring-blue-300 focus:outline-none float-right">削除</button>
            </form>
            <p class="text-gray-500 text-sm">投稿日: {post.created_at}</p>
          </li>
          {/each}
        {:else}
         <p>Loading...</p>
        {/if}
      </ul>
    </div>

    <Paging
      {page} {pageSize} total={data.all_count}
      on:pageChange={onPageChange}
    />

</main>