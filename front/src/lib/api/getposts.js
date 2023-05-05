export async function getposts(offset, row_count) {
	// データの取得
	let fd = new FormData();
	fd.set('offset', offset);
	fd.set('row_count', row_count);
	const fd_obj = Object.fromEntries(fd);
	const res = await fetch('http://172.100.1.100:8080/getposts', {
		method: 'POST',
		body: JSON.stringify(fd_obj),
		headers: {
			'Content-Type': 'application/json'
		}
	});
	const posts = await res.json();

	// 必ずオブジェクトを返す
	return {
		list: posts[0].map((post) => ({
			name: 'anonymous',
			id: post.id,
			content: post.content,
			created_at: post.created_at
		})),
		all_count: posts[1].count,
	};
}