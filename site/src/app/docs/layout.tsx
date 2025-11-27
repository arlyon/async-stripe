import { GithubInfo } from "fumadocs-ui/components/github-info";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { HomeIcon } from "lucide-react";
import { GithubLink } from "@/components/Nav";
import { baseOptions } from "@/lib/layout.shared";
import { source } from "@/lib/source";

export default async function Layout({ children }: LayoutProps<"/docs">) {
  return (
    <DocsLayout tree={source.pageTree} {...(await baseOptions())}>
      {children}
    </DocsLayout>
  );
}
