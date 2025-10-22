<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Button } from '$lib/components/ui/button';

	let testResult = '';
	let isLoading = false;

	async function testCacheFlow() {
		isLoading = true;
		testResult = 'Starting test...\n';

		const testUrl = 'https://example.com/test.jpg';
		const testPath = 'images/test123.jpg';

		try {
			// Test 1: Check if image is cached (should be empty)
			testResult += '1. Checking cache...\n';
			const cached = await invoke('get_cached_image_path', { url: testUrl });
			testResult += `   Result: ${cached || 'Not cached'}\n`;

			// Test 2: Cache the image
			testResult += '2. Caching image...\n';
			const id = await invoke('cache_image', { url: testUrl, local_path: testPath });
			testResult += `   Cached with ID: ${id}\n`;

			// Test 3: Check if image is now cached
			testResult += '3. Checking cache again...\n';
			const cached2 = await invoke('get_cached_image_path', { url: testUrl });
			testResult += `   Result: ${cached2}\n`;

			testResult += '\n✅ Test completed successfully!';
		} catch (error) {
			testResult += `\n❌ Error: ${error}`;
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="p-8">
	<h1 class="text-2xl font-bold mb-4">Database Cache Test</h1>

	<Button onclick={testCacheFlow} disabled={isLoading}>
		{isLoading ? 'Testing...' : 'Run Test'}
	</Button>

	{#if testResult}
		<pre class="mt-4 p-4 bg-gray-100 dark:bg-gray-800 rounded whitespace-pre-wrap">
{testResult}
		</pre>
	{/if}
</div>
