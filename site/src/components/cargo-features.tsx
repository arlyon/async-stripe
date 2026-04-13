import { readFileSync } from "fs";
import { join } from "path";

interface CargoFeaturesProps {
  file: string;
}

export function CargoFeatures({ file }: CargoFeaturesProps) {
  const filePath = join(process.cwd(), "..", file);
  let content = "";
  try {
    content = readFileSync(filePath, "utf-8");
  } catch (e) {
    console.error(`Failed to read Cargo.toml at ${filePath}:`, e);
    return null;
  }

  const lines = content.split("\n");
  let inFeatures = false;
  
  const features: { name: string; description: string }[] = [];
  let currentComments: string[] = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
      inFeatures = trimmed === "[features]";
      continue;
    }

    if (!inFeatures) continue;

    if (trimmed.startsWith("#")) {
      const comment = trimmed.replace(/^#\s*/, "").trim();
      if (comment) {
        currentComments.push(comment);
      }
    } else if (trimmed && !trimmed.startsWith("#") && trimmed.includes("=")) {
      const featureMatch = trimmed.match(/^([a-zA-Z0-9_\-]+)\s*=/);
      if (featureMatch) {
        if (currentComments.length > 0) {
          features.push({
            name: featureMatch[1],
            description: currentComments.join(" "),
          });
        }
        currentComments = []; // reset for the next feature
      }
    } else if (!trimmed) {
      // Blank lines shouldn't reset comments if we want to allow spaced-out features
      // but they should reset comments if it's just a rogue comment block.
      // Usually comments are right above the feature. Let's keep it simple: reset on blank line.
      currentComments = [];
    }
  }

  if (features.length === 0) {
    return null;
  }

  return (
    <div className="my-6 w-full overflow-x-auto">
      <table className="w-full text-left border-collapse">
        <thead>
          <tr className="border-b border-b-zinc-200 dark:border-b-zinc-800 text-sm">
            <th className="py-2 pr-4 font-semibold">Feature</th>
            <th className="py-2 font-semibold">Description</th>
          </tr>
        </thead>
        <tbody className="align-baseline">
          {features.map((f) => (
            <tr key={f.name} className="border-b border-b-zinc-100 dark:border-b-zinc-900/50">
              <td className="py-3 pr-4 font-mono text-sm whitespace-nowrap">
                <code>{f.name}</code>
              </td>
              <td className="py-3 text-sm text-zinc-600 dark:text-zinc-400">
                {f.description}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
