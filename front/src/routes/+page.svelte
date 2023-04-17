<script>
	export let data;
  import Paging from '$lib/components/Paging.svelte';
</script>

<main class="container mx-auto my-8 flex-grow">

    <form action="" method="post" class="bg-white p-8 rounded-md shadow-md w-full max-w-md mx-auto">
        <div class="mb-6">
          <label for="message" class="block text-gray-700 text-sm font-semibold mb-2">メッセージ</label>
          <textarea id="message" name="message" rows="4" class="w-full px-3 py-2 text-gray-700 border rounded-md focus:border-blue-500 focus:ring-2 focus:ring-blue-300 focus:outline-none resize-none" placeholder="メッセージを入力"></textarea>
        </div>
        <div class="flex justify-end">
            <button type="submit" class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-6 rounded-full transition duration-200 ease-in focus:ring-4 focus:ring-blue-300 focus:outline-none">送信</button>
        </div>
      </form>
      
      <Paging />

      <div class="container mx-auto py-6">
        <ul>
        {#each data.list as post}
          <li class="bg-white p-6 rounded-md shadow-md mb-4">
            <h2 class="text-xl font-semibold mb-4">投稿者: {post.name}</h2>
            <p class="text-gray-700 mb-4">
                {post.content}
            </p>
            <p class="text-gray-500 text-sm">投稿日: {post.created_at}</p>
          </li>
          {/each}
        </ul>
      </div>
      
      <Paging />

</main>