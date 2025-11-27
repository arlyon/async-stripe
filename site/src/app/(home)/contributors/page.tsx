import Image from "next/image";
import Link from "next/link";
import { getOurContributors } from "@/lib/repo";

export default async function ContributorsPage() {
	const { contributors } = await getOurContributors();

	return (
		<>
			<main className="flex-1 py-16 px-4">
				<div className="max-w-6xl mx-auto space-y-16">
					<div className="text-center space-y-6">
						<h1 className="text-5xl font-bold bg-gradient-to-r from-blue-600 to-violet-600 dark:from-blue-400 dark:to-violet-400 bg-clip-text text-transparent">
							Contributors
						</h1>
						<p className="text-lg text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto">
							Brought to you by the amazing people who have helped overhaul this
							project. Want to be included on this list?{" "}
							<Link
								href="https://github.com/arlyon/async-stripe/discussions/168"
								className="text-blue-600 dark:text-blue-400 hover:underline font-medium"
							>
								Get in touch
							</Link>
							.
						</p>
					</div>

					<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
						{contributors.map((contributor: any) => (
							<Link
								href={contributor.html_url}
								key={contributor.login}
								className="group"
							>
								<article className="flex flex-col items-center gap-6 p-6 rounded-2xl border border-zinc-200 dark:border-zinc-800 hover:border-blue-500 dark:hover:border-blue-500 hover:-translate-y-2 hover:shadow-xl hover:shadow-blue-500/10 transition-all">
									<div className="relative">
										<div className="rounded-2xl overflow-hidden ring-2 ring-zinc-200 dark:ring-zinc-800 group-hover:ring-blue-500 transition-all">
											<Image
												src={contributor.avatar_url}
												alt={contributor.login}
												width={160}
												height={160}
												className="w-40 h-40"
											/>
										</div>
										{contributor.adornment && (
											<div className="absolute -bottom-4 left-0 right-0 flex justify-center">
												<div className="bg-white dark:bg-zinc-950 rounded-full px-3 py-1 border-2 border-zinc-200 dark:border-zinc-800">
													<Image
														src={contributor.adornment}
														alt="Badge"
														width={120}
														height={24}
													/>
												</div>
											</div>
										)}
									</div>
									<div className="text-center mt-2">
										<h3 className="text-xl font-semibold text-zinc-900 dark:text-zinc-100 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
											@{contributor.login}
										</h3>
										<p className="text-sm text-zinc-500 dark:text-zinc-400 mt-1">
											{contributor.contributions} contributions
										</p>
									</div>
								</article>
							</Link>
						))}
					</div>
				</div>
			</main>
		</>
	);
}
