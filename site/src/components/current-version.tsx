import { getOurRepoData } from "@/lib/repo";

export async function CurrentVersion() {
  const data = await getOurRepoData();
  return <code>{data.version}</code>;
}
