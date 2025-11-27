import Image from "next/image";
import Link from "next/link";
import { getSponsorData } from "@/lib/repo";

export default async function SponsorsPage() {
	const sponsors = await getSponsorData("arlyon");

	return (
		<>
			<main className="flex-1 py-16 px-4">
				<div className="max-w-6xl mx-auto space-y-16">
					<div className="text-center space-y-6">
						<h1 className="text-5xl font-bold bg-gradient-to-r from-blue-600 to-violet-600 dark:from-blue-400 dark:to-violet-400 bg-clip-text text-transparent">
							Sponsors
						</h1>
						<p className="text-lg text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto">
							This project would not be possible without the kind contributions
							from these independent sponsors.
						</p>
					</div>

					<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
						{sponsors.map((sponsor) => (
							<Link href={sponsor.url} key={sponsor.github} className="group">
								<article
									className="flex flex-col items-center gap-6 p-6 rounded-2xl border-2 hover:-translate-y-2 hover:shadow-xl transition-all
                  ${sponsor.level === 'gold'
                    ? 'border-yellow-400 dark:border-yellow-500 hover:shadow-yellow-500/30'
                    : 'border-zinc-200 dark:border-zinc-800 hover:border-blue-500 dark:hover:border-blue-500 hover:shadow-blue-500/10'
                  }
                "
								>
									<div
										className={`rounded-2xl overflow-hidden ring-4 transition-all ${
											sponsor.level === "gold"
												? "ring-yellow-400 dark:ring-yellow-500"
												: "ring-zinc-200 dark:ring-zinc-800 group-hover:ring-blue-500"
										}`}
									>
										<Image
											src={sponsor.avatar}
											alt={sponsor.github}
											width={160}
											height={160}
											className="w-40 h-40"
										/>
									</div>
									<div className="text-center">
										<h3 className="text-xl font-semibold text-zinc-900 dark:text-zinc-100 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
											@{sponsor.github}
										</h3>
										{sponsor.level === "gold" && (
											<span className="inline-block mt-2 px-3 py-1 text-xs font-semibold bg-gradient-to-r from-yellow-400 to-yellow-600 text-yellow-950 rounded-full">
												Gold Sponsor
											</span>
										)}
									</div>
								</article>
							</Link>
						))}
					</div>

					<div className="flex justify-center">
						<Link
							href="https://github.com/sponsors/arlyon"
							className="group flex items-center gap-3 px-8 py-4 bg-pink-600 hover:bg-pink-700 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:shadow-pink-500/50 transition-all"
						>
							<svg
								aria-hidden="true"
								viewBox="0 0 16 16"
								fill="currentColor"
								className="w-5 h-5"
							>
								<path
									fillRule="evenodd"
									d="M4.25 2.5c-1.336 0-2.75 1.164-2.75 3 0 2.15 1.58 4.144 3.365 5.682A20.565 20.565 0 008 13.393a20.561 20.561 0 003.135-2.211C12.92 9.644 14.5 7.65 14.5 5.5c0-1.836-1.414-3-2.75-3-1.373 0-2.609.986-3.029 2.456a.75.75 0 01-1.442 0C6.859 3.486 5.623 2.5 4.25 2.5zM8 14.25l-.345.666-.002-.001-.006-.003-.018-.01a7.643 7.643 0 01-.31-.17 22.075 22.075 0 01-3.434-2.414C2.045 10.731 0 8.35 0 5.5 0 2.836 2.086 1 4.25 1 5.797 1 7.153 1.802 8 3.02 8.847 1.802 10.203 1 11.75 1 13.914 1 16 2.836 16 5.5c0 2.85-2.045 5.231-3.885 6.818a22.08 22.08 0 01-3.744 2.584l-.018.01-.006.003h-.002L8 14.25zm0 0l.345.666a.752.752 0 01-.69 0L8 14.25z"
								/>
							</svg>
							<span>Become a Sponsor</span>
						</Link>
					</div>
				</div>
			</main>

			<section className="border-t border-zinc-200 dark:border-zinc-800 py-16 px-4">
				<div className="max-w-4xl mx-auto space-y-6">
					<h2 className="text-3xl font-bold text-center">Additional Thanks</h2>
					<p className="text-zinc-600 dark:text-zinc-400 text-center">
						We would also like to thank{" "}
						<Link
							href="https://github.com/wyyerd"
							className="text-blue-600 dark:text-blue-400 hover:underline font-medium"
						>
							wyyerd/stripe-rs
						</Link>{" "}
						for building the original codegen, from which this project was
						built. A lot has changed since then, but this library was
						undoubtedly built from a strong foundation.
					</p>
				</div>
			</section>
		</>
	);
}
