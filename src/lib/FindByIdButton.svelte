<script setup lang="ts">
import { Button, GradientButton } from 'flowbite-svelte';
import { invoke } from "@tauri-apps/api/tauri";

function getData() {
    let path: String = '/home/Mikhail/projects/SvelteTypeScript/person_database/src-tauri/config.yaml';
    invoke("get_info", {filePath: path}).then((message) => console.log(message));

}
</script>

<GradientButton class="ml-4" shadow color="green" on:click{getData}>Find</GradientButton>