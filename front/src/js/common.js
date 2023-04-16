export async function getSelect() {
  const res = await fetch('http://nono:8080/getposts');
  return await res.json();
}
