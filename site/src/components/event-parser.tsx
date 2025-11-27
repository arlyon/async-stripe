"use client";

import { CodeBlock } from "fumadocs-ui/components/codeblock";
import { AlertTriangle } from "lucide-react";
import { useState } from "react";

export function EventParser() {
	const [eventJson, setEventJson] = useState("");
	const [result, setResult] = useState<{
		success: boolean;
		message: string;
		parsed?: unknown;
	} | null>(null);

	const handleParse = async () => {
		try {
			// For now, just parse the JSON to validate it
			// TODO: Replace with WASM parser when ready
			const parsed = JSON.parse(eventJson);
			setResult({
				success: true,
				message: "Event parsed successfully!",
				parsed,
			});
		} catch (error) {
			setResult({
				success: false,
				message:
					error instanceof Error ? error.message : "Failed to parse JSON",
			});
		}
	};

	return (
		<div className="border border-dashed border-yellow-500 rounded-lg p-6 bg-yellow-50 dark:bg-yellow-950/20 not-prose">
			{/* Under Construction Header */}
			<div className="flex items-center gap-2 mb-4 text-yellow-700 dark:text-yellow-500">
				<AlertTriangle />
				<h3 className="font-bold text-lg">Under Construction</h3>
			</div>

			<p className="text-sm text-yellow-800 dark:text-yellow-400 mb-4">
				This feature is being built! Eventually this will use WASM to parse
				Stripe events using the actual async-stripe library.
			</p>

			<div className="space-y-4">
				<div>
					<label
						htmlFor="event-json"
						className="block text-sm font-medium mb-2"
					>
						Paste a Stripe Event JSON:
					</label>
					<textarea
						id="event-json"
						className="w-full h-48 p-3 border rounded-md font-mono text-sm bg-white dark:bg-gray-950"
						placeholder='{\n  "id": "evt_...",\n  "object": "event",\n  ...\n}'
						value={eventJson}
						onChange={(e) => setEventJson(e.target.value)}
					/>
				</div>

				<button
					type="button"
					onClick={handleParse}
					className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
				>
					Parse Event
				</button>

				{result && (
					<div
						className={`p-4 rounded-md ${
							result.success
								? "bg-green-50 dark:bg-green-950/20 border border-green-200 dark:border-green-800"
								: "bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800"
						}`}
					>
						<h4
							className={`font-semibold mb-2 ${
								result.success
									? "text-green-800 dark:text-green-400"
									: "text-red-800 dark:text-red-400"
							}`}
						>
							{result.success ? "✓ Success" : "✗ Error"}
						</h4>
						<p
							className={
								result.success
									? "text-green-700 dark:text-green-300"
									: "text-red-700 dark:text-red-300"
							}
						>
							{result.message}
						</p>
						{result.parsed && (
							<div className="mt-4">
								<pre className="text-xs overflow-auto p-3 bg-gray-100 dark:bg-gray-900 rounded">
									{JSON.stringify(result.parsed, null, 2)}
								</pre>
							</div>
						)}
					</div>
				)}
			</div>
		</div>
	);
}
