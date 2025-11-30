"use client";

import init, { parse, list_types } from "@arlyon/async-stripe-parser";
import { useEffect, useState } from "react";

export function EventParser() {
	const [eventJson, setEventJson] = useState("");
	const [selectedType, setSelectedType] = useState("Event");
	const [availableTypes, setAvailableTypes] = useState<string[]>([]);
	const [result, setResult] = useState<{
		success: boolean;
		message: string;
		parsed?: any;
	} | null>(null);
	const [wasmReady, setWasmReady] = useState(false);

	// Initialize WASM module
	useEffect(() => {
		init()
			.then(() => {
				setWasmReady(true);
				const types = list_types();
				setAvailableTypes(types);
				if (types.length > 0) {
					setSelectedType(types[0]);
				}
			})
			.catch((error) => {
				console.error("Failed to initialize WASM:", error);
			});
	}, []);

	const handleParse = async () => {
		if (!wasmReady) {
			setResult({
				success: false,
				message: "WASM module is still loading...",
			});
			return;
		}

		try {
			// Parse the JSON first to validate and get the parsed object
			const parsed = JSON.parse(eventJson);

			// Use the WASM parser to validate the event structure
			// This will throw an error with path information if validation fails
			parse(selectedType, eventJson);

			setResult({
				success: true,
				message: "Event parsed and validated successfully!",
				parsed,
			});
		} catch (error) {
			if (typeof error === "string") {
				console.log(error);
				setResult({
					success: false,
					message: error,
				});
			} else {
				setResult({
					success: false,
					message: error instanceof Error ? error.message : "Unknown error",
					parsed: error,
				});
			}
		}
	};

	return (
		<div className="border border-gray-200 dark:border-gray-800 rounded-lg p-6 bg-gray-50 dark:bg-gray-950/20 not-prose">
			<div className="space-y-4">
				<div>
					<label
						htmlFor="event-type"
						className="block text-sm font-medium mb-2"
					>
						Type:
					</label>
					<select
						id="event-type"
						className="w-full p-2 border rounded-md bg-white dark:bg-gray-950"
						value={selectedType}
						onChange={(e) => setSelectedType(e.target.value)}
						disabled={!wasmReady}
					>
						{availableTypes.map((type) => (
							<option key={type} value={type}>
								{type}
							</option>
						))}
					</select>
				</div>
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
						onKeyDown={(e) => {
							if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
								e.preventDefault();
								handleParse();
							}
						}}
						onPaste={() => {
							// Trigger parse after paste (give it a moment for the value to update)
							setTimeout(() => handleParse(), 100);
						}}
					/>
				</div>

				<button
					type="button"
					onClick={handleParse}
					disabled={!wasmReady}
					className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{wasmReady ? "Parse Event" : "Loading..."}
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
