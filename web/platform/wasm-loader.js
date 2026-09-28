export async function loadWasm(url) {
  const response = await fetch(url);
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes);

  return instance.exports;
}
