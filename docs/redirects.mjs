// Pages that moved in the change from MkDocs to Starlight (ANG-T-0040). After
// the build, this writes a page at each old address that sends the reader to
// the new one. Each target is relative, so the pages work under any base path.
// An address that is still a page of the site is not written over.
import { existsSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { redirectPage } from 'starlight-aurora/mdbook-redirects';

// Old address -> new address, both from the site root.
const MOVED = {
	'quick-start/': 'tutorials/quick-start/',
	'contributing/': 'how-to-guides/contributing/',
	'contributing/code_of_conduct/': 'how-to-guides/contributing/code_of_conduct/',
	'contributing/documentation_style_guide/': 'how-to-guides/contributing/documentation_style_guide/',
};

// The generated API reference was at api/<module>/; it is now at
// reference/api/<module>/.
const API = 'reference/api/';

// API pages that plissken 0.2.0 no longer writes -> the page that now shows
// the same items, else the parent module. Paths under reference/api/. Each
// gets a redirect at reference/api/<old>/ and at api/<old>/.
const DROPPED = {
	'angreal/task/': 'angreal/',
	'angreal/python_bindings/decorators/': 'angreal/',
	'angreal/integrations/uv/': 'angreal/',
	'angreal/utils/': 'rust/angreal/utils/',
	'angreal/py_logger/': 'rust/angreal/py_logger/',
	'angreal/python_bindings/venv/': 'angreal/integrations/venv/',
	'angreal/python_bindings/integrations/compose/': 'angreal/integrations/docker_integration/',
	'angreal/python_bindings/integrations/flox/': 'angreal/integrations/flox/',
	'angreal/python_bindings/integrations/git/': 'angreal/_integrations/git_module/',
	'angreal/python_bindings/integrations/oci/': 'angreal/integrations/oci/',
	'rust/angreal/integrations/': 'rust/angreal/',
	'rust/angreal/macros/': 'rust/angreal/',
};
for (const [from, to] of Object.entries(DROPPED)) {
	MOVED[API + from] = API + to;
	MOVED['api/' + from] = API + to;
}

export default function movedPages() {
	return {
		name: 'angreal-moved-pages',
		hooks: {
			'astro:build:done': async ({ dir, pages, logger }) => {
				const out = fileURLToPath(dir);
				const map = { ...MOVED };
				for (const { pathname } of pages) {
					if (pathname.startsWith(API)) map[pathname.slice('reference/'.length)] = pathname;
				}
				let written = 0;
				for (const [from, to] of Object.entries(map)) {
					const file = join(out, from, 'index.html');
					if (existsSync(file)) continue;
					const up = '../'.repeat(from.split('/').length - 1);
					await mkdir(join(out, from), { recursive: true });
					await writeFile(file, redirectPage(up + to));
					written++;
				}
				logger.info(`${written} redirects from the old addresses`);
			},
		},
	};
}
