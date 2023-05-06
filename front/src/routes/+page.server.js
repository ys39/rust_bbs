import { error } from '@sveltejs/kit';

export const actions = {
	create: async ({ request }) => {
		const fd = await request.formData();
		const fd_obj = Object.fromEntries(fd);
		const res = await fetch('http://172.100.1.100:8080/', {
			method: 'POST',
			body: JSON.stringify(fd_obj),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if(res.status === 400){
			throw error(400, 'bad request');
		}
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
