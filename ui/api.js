const BASE = '/api';

export async function create(name, content) {
  const response = await fetch(`${BASE}/create`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name, content }),
  });

  await checkResponse(response);

  // new id
  return await response.json();
}

export async function read(id) {
  const response = await fetch(`${BASE}/read/${id}`);
  await checkResponse(response);

  return await response.json();
}

export async function update(id, name, content) {
  const response = await fetch(`${BASE}/update/${id}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name, content }),
  });

  await checkResponse(response);
}

export async function remove(id) {
  const response = await fetch(`${BASE}/remove/${id}`, {
    method: 'DELETE',
  });

  await checkResponse(response);
}

export async function list() {
  const response = await fetch(`${BASE}/list`);
  await checkResponse(response);

  return await response.json();
}

async function checkResponse(response) {
  if (!response.ok) {
    throw new Error(`Response status: ${response.status} - ${await response.text()}`);
  }
}
