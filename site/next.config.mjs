import { createMDX } from "fumadocs-mdx/next";

import path from "path";

const __dirname = import.meta.dirname;

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
	reactStrictMode: true,
	images: {
		domains: ["avatars.githubusercontent.com"],
	},
	experimental: {
		// Enable filesystem caching for `next dev`
		turbopackFileSystemCacheForDev: true,
		// Enable filesystem caching for `next build`
		turbopackFileSystemCacheForBuild: true,
	},
	async rewrites() {
		return [
			{
				source: "/docs/:path*.mdx",
				destination: "/llms.mdx/:path*",
			},
		];
	},
};

export default withMDX(config);
