import { cache } from "react";

const REPO_NAME = "arlyon/async-stripe";
const CRATE_NAME = "async-stripe";
const GITHUB_TOKEN = process.env.GITHUB_TOKEN;

const HEADERS = GITHUB_TOKEN
	? {
			Authorization: `Bearer ${GITHUB_TOKEN}`,
		}
	: undefined;

const TIER_MAP: Record<string, string> = {
	ST_kwDOAIT8WM4AAfkP: "basic",
	ST_kwDOAIT8WM4AAfzC: "gold",
};

// we only include those who want to be included
// for privacy reasons. if you'd like to be on here
// please get in touch :)
const CONTRIBUTOR_WHITELIST = [
	{ login: "arlyon", adornment: "/community-expert.svg" },
	{ login: "erichCompSci" },
	{ login: "FL33TW00D" },
];

export type RepoData = {
	name: string;
	fullName: string;
	crate: string;
	stars: number;
	url: string;
	version: string;
};

export type Contributor = {
	login: string;
	html_url: string;
	avatar_url: string;
	adornment?: string;
	contributions: number;
};

export type Sponsor = {
	github: string;
	url: string;
	avatar: string;
	level: string;
};

/**
 * Gets useful information about a repository.
 *
 * @param fullName The full name of the repo in the form owner/name
 */
export const getRepoData = cache(
	async (fullName: string): Promise<RepoData> => {
		const res = await fetch(`https://api.github.com/repos/${fullName}`, {
			headers: HEADERS,
			next: { revalidate: 3600 }, // Cache for 1 hour
		});
		const {
			name,
			stargazers_count: stars,
			releases_url,
			html_url: url,
		} = await res.json();

		const releasesUrl = releases_url.replace("{/id}", "");

		const res2 = await fetch(releasesUrl, {
			headers: HEADERS,
			next: { revalidate: 3600 },
		});
		const tags = await res2.json();

		const version = tags.find((tag: { tag_name: string }) =>
			tag.tag_name.startsWith("async-stripe-v"),
		);

		return {
			name,
			fullName,
			crate: CRATE_NAME,
			stars,
			url,
			version: version.tag_name.replace("async-stripe-", ""),
		};
	},
);

export const getOurRepoData = () => getRepoData(REPO_NAME);

export const getContributorData = async (fullName: string) => {
	const res = await fetch(
		`https://api.github.com/repos/${fullName}/contributors`,
		{
			headers: HEADERS,
			next: { revalidate: 3600 },
		},
	);
	const contributors = await res.json();
	const filteredContributors = contributors
		.map((contributor: Contributor) => {
			const whitelist = CONTRIBUTOR_WHITELIST.find(
				(wl) => wl.login === contributor.login,
			);
			return whitelist ? { ...contributor, ...whitelist } : undefined;
		})
		.filter((f: Contributor | undefined) => f !== undefined);

	return { contributors: filteredContributors };
};

export const getOurContributors = () => getContributorData(REPO_NAME);

export const getSponsorData = async (name: string): Promise<Sponsor[]> => {
	const res = await fetch(`https://api.github.com/graphql`, {
		method: "POST",
		headers: HEADERS as HeadersInit,
		body: JSON.stringify({
			query: `{
        user(login: "${name}") {
          sponsorshipsAsMaintainer(first: 100) {
            totalCount
            edges {
              node {
                sponsorEntity {
                  ... on Organization {
                    avatarUrl
                    login
                    url
                  }
                  ... on User {
                    avatarUrl
                    login
                    url
                  }
                }
                tier {
                  name
                  id
                }
              }
            }
          }
        }
      }`,
			variables: {},
		}),
		next: { revalidate: 3600 },
	});

	const { data } = await res.json();
	const rawSponsors = data.user.sponsorshipsAsMaintainer.edges.map(
		(e: any) => e.node,
	);

	return rawSponsors.map((s: any) => ({
		github: s.sponsorEntity.login,
		url: s.sponsorEntity.url,
		avatar: s.sponsorEntity.avatarUrl,
		level: TIER_MAP[s.tier.id] ?? "basic",
	}));
};
