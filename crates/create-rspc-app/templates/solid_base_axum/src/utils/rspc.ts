import { QueryClient } from "@tanstack/solid-query";
import { FetchTransport, createClient } from "@rspc/client";
import { createSolidQueryHooks } from "@rspc/solid-query";

import type { Procedures } from "./bindings"; // These are generated by rspc in Rust for you.

const client = createClient<Procedures>({
  transport: new FetchTransport("http://localhost:9000/rspc"),
});

const queryClient = new QueryClient();
const rspc = createSolidQueryHooks<Procedures>();

export { rspc, client, queryClient };
