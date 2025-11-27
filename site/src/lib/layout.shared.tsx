import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import { Banknote, BookIcon, UsersIcon } from "lucide-react";
import { GithubLink } from "@/components/Nav";
import { getOurRepoData } from "./repo";

export async function baseOptions(): Promise<BaseLayoutProps> {
  const repoData = await getOurRepoData();
  return {
    nav: {
      title: "Async Stripe",
    },
    links: [
      {
        type: "custom",
        children: (
          <GithubLink
            url="https://github.com/arlyon/async-stripe"
            name="arlyon/async-stripe"
            stars={repoData.stars}
            version={repoData.version}
            className="justify-center"
          />
        ),
        secondary: true,
      },
      {
        icon: <UsersIcon />,
        text: "Contributors",
        url: "/contributors",
      },
      {
        icon: <Banknote />,
        text: "Sponsors",
        url: "/sponsors",
      },
      {
        icon: <BookIcon />,
        text: "Docs",
        url: "/docs",
        active: "nested-url",
      },
    ],
  };
}
