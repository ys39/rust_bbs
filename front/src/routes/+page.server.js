/*
export async function load() {
	// getパラメータの取得
	//const queryParams = page.query;
	//const p = queryParams.get('page');

	// データの取得
	const res = await fetch('http://172.100.1.100:8080/getposts');
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
		//props: {
		//	p
		//},
	};
}
*/

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
	},
	delete: async ({ request }) => {
		const fd = await request.formData();
		const fd_obj = Object.fromEntries(fd);
		await fetch('http://172.100.1.100:8080/delete', {
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
