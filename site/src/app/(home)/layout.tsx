import { HomeLayout } from "fumadocs-ui/layouts/home";
import { Footer } from "@/components/Footer";
import { baseOptions } from "@/lib/layout.shared";
import { getOurRepoData } from "@/lib/repo";

export default async function Layout({ children }: LayoutProps<"/">) {
  const repoData = await getOurRepoData();

  return (
    <HomeLayout {...(await baseOptions())}>
      {children}
      <Footer url={repoData.url} crate={repoData.crate} />
    </HomeLayout>
  );
}
