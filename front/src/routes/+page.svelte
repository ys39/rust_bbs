<script>
	export let data;
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


      <div class="container mx-auto py-12">
        <div class="flex items-center justify-center">
          <nav aria-label="Page navigation" class="text-center">
            <ul class="flex items-center justify-between space-x-4">
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">1</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">2</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">3</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">4</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">5</a>
              </li>
            </ul>
          </nav>
        </div>
      </div>


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

      <div class="container mx-auto py-12">
        <div class="flex items-center justify-center">
          <nav aria-label="Page navigation" class="text-center">
            <ul class="flex items-center justify-between space-x-4">
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">1</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">2</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">3</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">4</a>
              </li>
              <li>
                <a href="#" class="block px-3 py-1 text-lg font-bold text-gray-700 rounded-md hover:bg-gray-300 focus:ring-2 focus:ring-blue-300 focus:outline-none">5</a>
              </li>
            </ul>
          </nav>
        </div>
      </div>

</main>