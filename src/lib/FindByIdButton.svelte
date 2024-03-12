<script setup lang="ts">
import { GradientButton } from 'flowbite-svelte';
import { invoke } from "@tauri-apps/api/tauri";
import {persons,type Person, type Persons} from '../scripts/personsStorage'
</script>

<GradientButton class="ml-4" shadow color="green">Find</GradientButton>