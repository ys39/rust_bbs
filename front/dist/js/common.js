'use strict';
form1.onsubmit = async (e) => {
  e.preventDefault();
  const fd = new FormData(form1);
  const fd_obj = Object.fromEntries(fd);
  let response = await fetch('http://nono:8080/', {
    method: 'POST',
    body: JSON.stringify(fd_obj),
    headers: {
      'Content-Type': 'application/json',
    },
  });
};
