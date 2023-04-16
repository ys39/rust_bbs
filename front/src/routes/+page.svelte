<script>
	export let data;
</script>

<main class="container mx-auto my-8 flex-grow">
    <section class="grid grid-cols-1 gap-8">
        {#each data.list as post}
            <p class="divide-dashed">{post.content}</p>
        {/each}
    </section>
</main>