export async function load() {
	// データの取得
	const res = await fetch('http://172.100.1.100:8080/getposts');
	const posts = await res.json();

	// 必ずオブジェクトを返す
	return {
		list: posts.map((post) => ({
			name: 'anonymous',
			id: post.id,
			content: post.content,
			created_at: post.created_at
		}))
	};
}

export const actions = {
	create: async ({ request }) => {
		const fd = await request.formData();
		const fd_obj = Object.fromEntries(fd);
		await fetch('http://172.100.1.100:8080/', {
			method: 'POST',
			body: JSON.stringify(fd_obj),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		return {
			status: 302,
			msg: 'ok'
		};
	}
};
