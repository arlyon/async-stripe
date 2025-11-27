import { readFileSync } from "fs";
import { highlight } from "fumadocs-core/highlight";
import * as Base from "fumadocs-ui/components/codeblock";
import Link from "next/link";
import { join } from "path";

interface CodeFromFileProps {
	file: string;
	lang?: string;
	title?: string;
	startLine?: number;
	endLine?: number;
	highlightLines?: number[];
	href?: string | { base: string };
	dedent?: boolean;
}

export async function CodeFromFile({
	file,
	lang = "rust",
	title,
	startLine,
	endLine,
	highlightLines = [],
	href,
	dedent = false,
}: CodeFromFileProps) {
	// Read the file from the project root
	const filePath = join(process.cwd(), "..", file);
	let content = readFileSync(filePath, "utf-8");

	// Extract specific line range if specified
	if (startLine !== undefined || endLine !== undefined) {
		const lines = content.split("\n");
		const start = (startLine ?? 1) - 1;
		const end = endLine ?? lines.length;
		content = lines.slice(start, end).join("\n");
	}

	// Dedent if requested
	if (dedent) {
		const lines = content.split("\n");
		// Find the minimum indentation (ignoring empty lines)
		const minIndent = lines
			.filter((line) => line.trim().length > 0)
			.reduce((min, line) => {
				const match = line.match(/^(\s*)/);
				const indent = match ? match[1].length : 0;
				return Math.min(min, indent);
			}, Number.POSITIVE_INFINITY);

		// Strip that many spaces from all lines
		if (minIndent > 0 && minIndent !== Number.POSITIVE_INFINITY) {
			content = lines.map((line) => line.slice(minIndent)).join("\n");
		}
	}

	// Build meta string for line highlighting
	const metaParts = [];
	if (highlightLines.length > 0) {
		metaParts.push(`{${highlightLines.join(",")}}`);
	}
	const meta = metaParts.length > 0 ? metaParts.join(" ") : undefined;

	const rendered = await highlight(content, {
		lang,
		meta,
		components: {
			pre: (props) => <Base.Pre {...props} />,
		},
	});

	const text = title ?? file;
	const range =
		startLine && endLine ? { start: startLine, end: endLine } : undefined;

	const titleC = href ? (
		<Link
			href={
				typeof href === "string"
					? createUrl(undefined, href, range)
					: createUrl(href.base, file, range)
			}
			target="_blank"
			className="hover:underline"
		>
			{text}
		</Link>
	) : (
		text
	);

	return (
		<Base.CodeBlock
			title={titleC}
			code={rendered}
			lang={lang}
			data-line-numbers
			data-line-numbers-start={startLine}
		>
			{rendered}
		</Base.CodeBlock>
	);
}

// given a base and a path, produce a url
const createUrl = (
	base: string | undefined,
	path: string,
	range?: { start: number; end: number },
) => {
	const url = new URL(path, base);
	if (range) {
		// #L46-L49
		url.hash = `L${range.start}-L${range.end}`;
	}
	return url.toString();
};
