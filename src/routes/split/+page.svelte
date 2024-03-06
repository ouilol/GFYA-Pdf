<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { open } from '@tauri-apps/api/dialog';
	import { appDir } from '@tauri-apps/api/path';
	import { stringify } from 'postcss';

	let sourceFile: string | null;
	let destination: string | null;

	async function handleOpenFolder() {
		const selected = await open({
			multiple: false,
			directory: true
		});
		console.log(selected);
		destination = selected;
	}

	async function submit() {
		if (sourceFile !== null && destination !== null){
			await invoke('split_pdf', { sourceFile: sourceFile, destinationFolder: destination });
		}
	}
</script>

<div class="flex justify-center items-center h-96">
	<div class="text-center">
		<h1 class="text-3xl">PDF Split</h1>

		<form method="POST">
			<label class="form-control w-full max-w-xs mt-4">
				<div class="label">
					<span class="label-text">Source file</span>
				</div>
				<input
					bind:value={sourceFile}
					type="file"
					class="file-input file-input-bordered w-full max-w-xs"
				/>
			</label>

			<label class="form-control w-full max-w-xs mt-4">
				<div class="label">
					<span class="label-text">Destination folder</span>
				</div>
				<input
					bind:value={destination}
					type="text"
					class="file-input file-input-bordered w-full max-w-xs"
					on:click={handleOpenFolder}
				/>
			</label>
		</form>
		<button class="mt-4 btn btn-primary" on:click={submit}>Split</button>
	</div>
</div>
