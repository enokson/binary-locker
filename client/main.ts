const exampleConfig = {
  servers: [
    { url: 'str' }
  ]
}

type ServerConfig = {
  url: string
}

type Config = {
  servers: ServerConfig[],
}

type LocalConfig = {
  readDir: string,
  writeDir: string,
  store?: string
}

export function add(a: number, b: number): number {
  return a + b;
}

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  console.log("Add 2 + 3 =", add(2, 3));
}
